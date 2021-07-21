use std::ops::{Deref, DerefMut, Range};
use std::path::Path;

use crate::commands;
use crate::commands::SCROLL_TO;
use crate::syntax::StateCache;
use crate::text_buffer::{position, rope_utils, EditStack, SelectionLineRange};

use druid::{Data, FontStyle};
use druid::{
    kurbo::{BezPath, Line, PathEl, Point, Rect, Size},
    piet::{PietText, RenderContext, Text, TextAttribute, TextLayout, TextLayoutBuilder},
    widget::Flex,
    Affine, Application, BoxConstraints, ClipboardFormat, Color, Command, Env, Event, EventCtx, FileDialogOptions,
    FontWeight, HotKey, KeyEvent, LayoutCtx, LifeCycle, LifeCycleCtx, MouseButton, PaintCtx, SysMods, Target,
    UpdateCtx, Widget, WidgetExt, WidgetId,
};

use rfd::MessageDialog;

mod env {
    use druid::Key;

    pub const FONT_SIZE: Key<f64> = Key::new("nonepad.editor.font_height");
    pub const FONT_ADVANCE: Key<f64> = Key::new("nonepad.editor.font_advance");
    pub const FONT_BASELINE: Key<f64> = Key::new("nonepad.editor.font_baseline");
    pub const FONT_DESCENT: Key<f64> = Key::new("nonepad.editor.font_descent");
    pub const FONT_HEIGHT: Key<f64> = Key::new("nonepad.editor.fonth_height");
    pub const PAGE_LEN: Key<u64> = Key::new("nonepad.editor.page_len");
}

pub const FONT_NAME: &str = "Consolas";
pub const FONT_SIZE: f64 = 14.;
pub const FONT_WEIGTH: FontWeight = FontWeight::SEMI_BOLD;
pub const EDITOR_LEFT_PADDING: f64 = 2.;
pub const SCROLLBAR_X_PADDING: f64 = 2.;

// pub const BG_COLOR: Key<Color> = Key::new("nondepad.editor.fg_color");
// pub const FG_COLOR: Key<Color> = Key::new("nondepad.editor.bg_color");
// pub const FG_SEL_COLOR: Key<Color> = Key::new("nondepad.editor.fg_selection_color");
// pub const BG_SEL_COLOR: Key<Color> = Key::new("nondepad.editor.bg_selection_color");

#[derive(Debug, Default)]
struct SelectionPath {
    elem: Vec<PathEl>,
    last_range: Option<SelectionLineRange>,
    last_x: f64,
}

impl Deref for SelectionPath {
    type Target = Vec<PathEl>;
    fn deref(&self) -> &Self::Target {
        &self.elem
    }
}

impl DerefMut for SelectionPath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elem
    }
}

impl SelectionPath {
    fn new() -> Self {
        Self {
            elem: Vec::new(),
            last_range: None,
            last_x: 0.5,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CommonMetrics {
    font_advance: f64,
    font_baseline: f64,
    font_descent: f64,
    font_height: f64,

    font_size: f64,

    page_len: u64,
}

impl CommonMetrics {
    pub fn new(text_ctx: &mut PietText, font_name: &str, size: Size) -> Self {
        let mut metrics = CommonMetrics::default();
        let font = text_ctx.font_family(font_name).unwrap();
        let layout = text_ctx
            .new_text_layout("8")
            .default_attribute(TextAttribute::Weight(FONT_WEIGTH))
            .font(font, metrics.font_size)
            .build()
            .unwrap();
        metrics.font_advance = layout.size().width;
        metrics.font_baseline = layout.line_metric(0).unwrap().baseline;
        metrics.font_height = layout.line_metric(0).unwrap().height;
        metrics.font_descent = metrics.font_height - metrics.font_baseline;
        metrics.page_len = (size.height / metrics.font_height).round() as u64;
        metrics
    }

    pub fn from_env(env: &Env) -> Self {
        CommonMetrics {
            font_baseline: env.get(env::FONT_BASELINE),
            font_advance: env.get(env::FONT_ADVANCE),
            font_descent: env.get(env::FONT_DESCENT),
            font_height: env.get(env::FONT_HEIGHT),
            font_size: env.get(env::FONT_SIZE),
            page_len: env.get(env::PAGE_LEN),
        }
    }

    pub fn to_env(&self, env: &mut Env) {
        env.set(env::FONT_BASELINE, self.font_baseline);
        env.set(env::FONT_ADVANCE, self.font_advance);
        env.set(env::FONT_DESCENT, self.font_descent);
        env.set(env::FONT_HEIGHT, self.font_height);
        env.set(env::FONT_SIZE, self.font_size);
        env.set(env::PAGE_LEN, self.page_len);
    }
}

impl Default for CommonMetrics {
    fn default() -> Self {
        CommonMetrics {
            font_advance: 0.0,
            font_baseline: 0.0,
            font_descent: 0.0,
            font_height: 0.0,
            font_size: FONT_SIZE,
            page_len: 0,
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
enum HeldState {
    None,
    Grapheme,
    Word,
    Line,
}

impl HeldState {
    fn is_held(&self) -> bool {
        *self != HeldState::None
    }
}

#[derive(Debug)]
pub struct EditorView {
    //editor: EditStack,
    delta_y: f64,
    delta_x: f64,
    page_len: usize,
    metrics: CommonMetrics,
    font_name: String,

    bg_color: Color,
    fg_color: Color,
    fg_sel_color: Color,
    bg_sel_color: Color,

    size: Size,
    owner_id: WidgetId,

    longest_line_len: f64,

    held_state: HeldState,
    highlight_cache: StateCache,
    //handle: WindowHandle,
}

impl Widget<EditStack> for EditorView {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, editor: &mut EditStack, _env: &Env) {
        
        match event {
            Event::WindowConnected => {
                ctx.request_focus();
            }
            Event::KeyDown(event) => {
                commands::COMMANDSET.hotkey_submit(ctx, event, self, editor);
                // if HotKey::new(SysMods::CmdShift, Key::KeyP).matches(event) {
                //     handle.app_ctx().open_palette(vec![], |u| println!("Palette result {}", u));
                //     _ctx.request_paint();
                //     return true;
                // }

                if HotKey::new(SysMods::AltCmd, druid::keyboard_types::Key::ArrowDown).matches(event) {
                    editor.duplicate_down();
                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }
                if HotKey::new(SysMods::AltCmd, druid::keyboard_types::Key::ArrowUp).matches(event) {
                    editor.duplicate_up();
                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }
                match event {
                    KeyEvent {
                        key: druid::keyboard_types::Key::ArrowRight,
                        mods,
                        ..
                    } => {
                        editor.forward(mods.shift(), mods.ctrl());
                        self.put_caret_in_visible_range(ctx, editor);
                        ctx.request_paint();
                        ctx.set_handled();
                        return;
                    }
                    KeyEvent {
                        key: druid::keyboard_types::Key::ArrowLeft,
                        mods,
                        ..
                    } => {
                        editor.backward(mods.shift(), mods.ctrl());
                        self.put_caret_in_visible_range(ctx, editor);
                        ctx.request_paint();
                        ctx.set_handled();
                        return;
                    }
                    KeyEvent {
                        key: druid::keyboard_types::Key::ArrowUp,
                        mods,
                        ..
                    } => {
                        editor.up(mods.shift());
                        self.put_caret_in_visible_range(ctx, editor);
                        ctx.request_paint();
                        ctx.set_handled();
                        return;
                    }
                    KeyEvent {
                        key: druid::keyboard_types::Key::ArrowDown,
                        mods,
                        ..
                    } => {
                        editor.down(mods.shift());
                        self.put_caret_in_visible_range(ctx, editor);
                        ctx.request_paint();
                        ctx.set_handled();
                        return;
                    }
                    KeyEvent {
                        key: druid::keyboard_types::Key::PageUp,
                        mods,
                        ..
                    } => {
                        for _ in 0..self.page_len {
                            editor.up(mods.shift());
                        }
                        self.put_caret_in_visible_range(ctx, editor);
                        ctx.request_paint();
                        ctx.set_handled();
                        return;
                    }
                    KeyEvent {
                        key: druid::keyboard_types::Key::PageDown,
                        mods,
                        ..
                    } => {
                        for _ in 0..self.page_len {
                            editor.down(mods.shift())
                        }
                        self.put_caret_in_visible_range(ctx, editor);
                        ctx.request_paint();
                        ctx.set_handled();
                        return;
                    }
                    KeyEvent {
                        key: druid::keyboard_types::Key::End,
                        mods,
                        ..
                    } => {
                        editor.end(mods.shift());
                        self.put_caret_in_visible_range(ctx, editor);
                        ctx.request_paint();
                        ctx.set_handled();
                        return;
                    }
                    KeyEvent {
                        key: druid::keyboard_types::Key::Home,
                        mods,
                        ..
                    } => {
                        editor.home(mods.shift());
                        self.put_caret_in_visible_range(ctx, editor);
                        ctx.request_paint();
                        ctx.set_handled();
                        return;
                    }
                    KeyEvent {
                        key: druid::keyboard_types::Key::Tab,
                        ..
                    } => {
                        
                        editor.tab();
                        self.invalidate_highlight_cache(editor);
                        self.put_caret_in_visible_range(ctx, editor);
                        ctx.request_paint();
                        ctx.set_handled();
                        return;
                    }
                    _ => (),
                }

                if HotKey::new(None, druid::keyboard_types::Key::Escape).matches(event) {
                    editor.cancel_mutli_carets();
                    editor.cancel_selection();
                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }

                if HotKey::new(None, druid::keyboard_types::Key::Backspace).matches(event) {
                    
                    editor.backspace();
                    self.invalidate_highlight_cache(editor);
                    self.put_caret_in_visible_range(ctx, editor);
                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }
                if HotKey::new(None, druid::keyboard_types::Key::Delete).matches(event) {
                    
                    editor.delete();
                    self.invalidate_highlight_cache(editor);
                    self.put_caret_in_visible_range(ctx, editor);
                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }

                if HotKey::new(None, druid::keyboard_types::Key::Enter).matches(event) {
                    // || HotKey::new(None, druid::keyboard_types::Key::Return).matches(event) {
                    
                    editor.insert(editor.file.linefeed.to_str());
                    self.invalidate_highlight_cache(editor);
                    self.put_caret_in_visible_range(ctx, editor);
                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }
                if HotKey::new(SysMods::Cmd, "v").matches(event) {
                    let clipboard = Application::global().clipboard();
                    let supported_types = &[ClipboardFormat::TEXT];
                    let best_available_type = clipboard.preferred_format(supported_types);
                    if let Some(format) = best_available_type {
                        let data = clipboard
                            .get_format(format)
                            .expect("I promise not to unwrap in production");
                        
                        editor.insert(String::from_utf8_lossy(&data).as_ref());
                        self.invalidate_highlight_cache(editor);
                    }
                    self.put_caret_in_visible_range(ctx, editor);
                    // TODO: The bug is fixed.
                    // in druid-shell, there is a bug with get_string, it dont close the clipboard, so after a paste, other application can't use the clipboard anymore
                    // get_format correctly close the slipboard
                    // let s= Application::global().clipboard().get_string().unwrap_or_default().clone();
                    // editor.insert(&dbg!(s));

                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }
                if HotKey::new(SysMods::Cmd, "c").matches(event) {
                    Application::global().clipboard().put_string(editor.selected_text());

                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }
                if HotKey::new(SysMods::Cmd, "x").matches(event) {
                    Application::global().clipboard().put_string(editor.selected_text());
                    
                    editor.delete();
                    self.invalidate_highlight_cache(editor);
                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }
                if HotKey::new(SysMods::Cmd, "a").matches(event) {
                    editor.select_all();
                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }
                if HotKey::new(SysMods::Cmd, "z").matches(event) {
                    
                    editor.undo();
                    self.invalidate_highlight_cache(editor);
                    self.put_caret_in_visible_range(ctx, editor);
                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }
                if HotKey::new(SysMods::Cmd, "y").matches(event) {
                    
                    editor.redo();
                    self.invalidate_highlight_cache(editor);
                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }
                if HotKey::new(SysMods::Cmd, "o").matches(event) {
                    if editor.is_dirty()
                        && !MessageDialog::new()
                            .set_level(rfd::MessageLevel::Warning)
                            .set_title("Are you sure?")
                            .set_description("Discard unsaved change?")
                            .set_buttons(rfd::MessageButtons::YesNo)
                            .show()
                    {
                        ctx.set_handled();
                        return;
                    }

                    let options = FileDialogOptions::new().show_hidden();
                    ctx.submit_command(Command::new(druid::commands::SHOW_OPEN_PANEL, options, Target::Auto));
                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }
                if HotKey::new(SysMods::Cmd, "s").matches(event) {
                    //self.save(editor, ctx);
                    if editor.filename.is_some() {
                        ctx.submit_command(Command::new(druid::commands::SAVE_FILE, (), Target::Auto));
                    } else {
                        let options = FileDialogOptions::new().show_hidden();
                        ctx.submit_command(Command::new(druid::commands::SHOW_SAVE_PANEL, options, Target::Auto))
                    }
                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }
                if HotKey::new(SysMods::CmdShift, "s").matches(event) {
                    let options = FileDialogOptions::new().show_hidden();
                    ctx.submit_command(Command::new(druid::commands::SHOW_SAVE_PANEL, options, Target::Auto));

                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }
                if HotKey::new(SysMods::Cmd, "f").matches(event) {
                    ctx.submit_command(Command::new(
                        commands::SHOW_SEARCH_PANEL,
                        editor.main_cursor_selected_text(),
                        Target::Global,
                    ));

                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }
                if HotKey::new(SysMods::Cmd, "d").matches(event) {
                    // if editor.has_many_carets() {
                    //     duplicate_cursor_from_str()
                    // } else {

                    // }
                    editor
                        .buffer
                        .duplicate_cursor_from_str(&editor.main_cursor_selected_text());
                    // self.put_caret_in_visible_range(ctx, editor);
                }

                if let druid::keyboard_types::Key::Character(text) = event.key.clone() {
                    if event.mods.ctrl() || event.mods.alt() || event.mods.meta() {
                        return;
                    }
                    if text.chars().count() == 1 && text.chars().next().unwrap().is_ascii_control() {
                        return;
                    }
                    
                    editor.insert(&text);
                    self.invalidate_highlight_cache(editor);
                    self.put_caret_in_visible_range(ctx, editor);
                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }
            }
            Event::Wheel(event) => {
                // if editor.len_lines() as f64 * self.metrics.font_height >= ctx.size().height {
                //     self.delta_y -= event.wheel_delta.y;
                //     if self.delta_y > 0. {
                //         self.delta_y = 0.;
                //     }
                //     if -self.delta_y
                //         > editor.len_lines() as f64 * self.metrics.font_height - 4. * self.metrics.font_height
                //     {
                //         self.delta_y =
                //             -((editor.len_lines() as f64) * self.metrics.font_height - 4. * self.metrics.font_height)
                //     }
                // }

                ctx.submit_command(Command::new(
                    commands::SCROLL_TO,
                    (
                        Some(self.delta_x - event.wheel_delta.x),
                        Some(self.delta_y - event.wheel_delta.y),
                    ),
                    self.owner_id,
                ));

                // self.delta_x -= event.wheel_delta.x;
                // if self.delta_x > 0. {
                //     self.delta_x = 0.;
                // }
                if ctx.is_active() && event.buttons.contains(MouseButton::Left) {
                    let (x, y) = self.pix_to_point(event.pos.x, event.pos.y, ctx, editor);
                    let p = editor.point(x, y);
                    editor.move_main_caret_to(p, true, false);
                    self.put_caret_in_visible_range(ctx, editor);
                }
                ctx.request_paint();
                ctx.set_handled();
            }
            Event::MouseDown(event) => {
                if matches!(event.button, MouseButton::Left) {
                    let (x, y) = self.pix_to_point(event.pos.x, event.pos.y, ctx, editor);
                    editor.cancel_mutli_carets();
                    // FIXME: Update is not called if the caret position is not modified,
                    let p = editor.point(x, y);
                    match event.count {
                        1 => {
                            editor.move_main_caret_to(p, event.mods.shift(), false);
                            self.held_state = HeldState::Grapheme;
                        }
                        2 => {
                            editor.move_main_caret_to(p, event.mods.shift(), true);
                            self.held_state = HeldState::Word;
                        }
                        3 => {
                            editor.select_line(p.line, event.mods.shift());
                            self.held_state = HeldState::Line;
                        }
                        4 => {
                            editor.select_all();
                            self.held_state = HeldState::None;
                        }
                        _ => (),
                    }
                    ctx.set_active(true);
                }
                ctx.request_focus();
                ctx.request_paint();
                ctx.set_handled();
            }
            Event::MouseUp(_event) => {
                ctx.set_active(false);
                ctx.set_handled();
                self.held_state = HeldState::None;
            }
            Event::MouseMove(event) => {
                if self.held_state.is_held() && ctx.is_active() && event.buttons.contains(MouseButton::Left) {
                    let (x, y) = self.pix_to_point(event.pos.x, event.pos.y, ctx, editor);
                    let p = editor.point(x, y);
                    match self.held_state {
                        HeldState::Grapheme => editor.move_main_caret_to(p, true, false),
                        HeldState::Word => editor.move_main_caret_to(p, true, true),
                        HeldState::Line => editor.select_line(p.line, true),
                        HeldState::None => unreachable!(),
                    }
                    self.put_caret_in_visible_range(ctx, editor);
                }
            }
            Event::WindowCloseRequested => {
                if editor.is_dirty() {
                    if !MessageDialog::new()
                        .set_level(rfd::MessageLevel::Warning)
                        .set_description("The currently opened editor is not saved. Do you really want to quit?")
                        .set_title("Not saved")
                        .set_buttons(rfd::MessageButtons::OkCancle)
                        .show()
                    {
                        ctx.set_handled();
                    }
                }
            }

            Event::Command(cmd) if cmd.is(druid::commands::SAVE_FILE_AS) => {
                let file_info = cmd.get_unchecked(druid::commands::SAVE_FILE_AS);
                if let Err(e) = self.save_as(editor, file_info.path()) {
                    println!("Error writing file: {}", e);
                }
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(druid::commands::SAVE_FILE) => {
                if let Err(e) = self.save(editor) {
                    println!("Error writing file: {}", e);
                }

                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(druid::commands::OPEN_FILE) => {
                if let Some(file_info) = cmd.get(druid::commands::OPEN_FILE) {
                    if let Err(e) = self.open(editor, file_info.path()) {
                        MessageDialog::new()
                            .set_level(rfd::MessageLevel::Error)
                            .set_title("Error")
                            .set_description(&format!("Error loading file {}", e))
                            .set_buttons(rfd::MessageButtons::Ok)
                            .show();
                    }
                }
            }
            Event::Command(cmd) if cmd.is(commands::REQUEST_NEXT_SEARCH) => {
                if let Some(data) = cmd.get(commands::REQUEST_NEXT_SEARCH) {
                    editor.search_next(data);
                    self.put_caret_in_visible_range(ctx, editor);
                }
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(commands::GIVE_FOCUS) => ctx.request_focus(),
            Event::Command(cmd) if cmd.is(commands::SELECT_LINE) => {
                let (line, expand) = *cmd.get_unchecked(commands::SELECT_LINE);
                editor.buffer.select_line(line.into(), expand);
                self.put_caret_in_visible_range(ctx, editor);
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(commands::SCROLL_TO) => {
                let d = *cmd.get_unchecked(commands::SCROLL_TO);
                self.delta_x = d.0.unwrap_or(self.delta_x);
                self.delta_y = d.1.unwrap_or(self.delta_y);
                ctx.set_handled();
                ctx.request_paint();
            }
            Event::Command(cmd) if cmd.is(commands::RESET_HELD_STATE) => {
                self.held_state = HeldState::None;
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _data: &EditStack, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            self.bg_color = env.get(crate::theme::EDITOR_BACKGROUND);
            self.fg_color = env.get(crate::theme::EDITOR_FOREGROUND);
            self.fg_sel_color = env.get(crate::theme::SELECTION_BACKGROUND);
            self.bg_sel_color = env.get(crate::theme::EDITOR_FOREGROUND);

            ctx.register_for_focus();
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &EditStack, data: &EditStack, _env: &Env) {
        if !old_data.buffer.same_content(&data.buffer) {
            self.invalidate_highlight_cache(data);
            
        }
        
        ctx.request_paint();
    }

    fn layout(&mut self, layout_ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &EditStack, _env: &Env) -> Size {
        self.size = bc.max();

        self.metrics = CommonMetrics::new(layout_ctx.text(), &self.font_name, self.size);
        self.page_len = (self.size.height / self.metrics.font_height).round() as usize;

        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EditStack, env: &Env) {
        self.paint_editor(data, ctx, env);
    }
}

impl EditorView {
    pub fn new(owner_id: WidgetId) -> Self {
        EditorView {
            bg_color: Color::BLACK,
            fg_color: Color::WHITE,
            fg_sel_color: Color::BLACK,
            bg_sel_color: Color::WHITE,

            metrics: Default::default(),
            font_name: FONT_NAME.to_string(),
            delta_x: 0.0,
            delta_y: 0.0,
            page_len: 0,

            size: Default::default(),
            owner_id,
            longest_line_len: 0.,
            held_state: HeldState::None,
            highlight_cache: StateCache::new(),
        }
    }
    fn visible_range(&self) -> Range<usize> {
        (-self.delta_y / self.metrics.font_height) as usize
            ..((-self.delta_y + self.size.height) / self.metrics.font_height) as usize + 1
    }

    fn add_bounded_range_selection<L: TextLayout>(
        &mut self,
        y: f64,
        range: Range<position::Relative>,
        layout: &L,
        path: &mut SelectionPath,
    ) {
        let s = layout.hit_test_text_position(range.start.into());
        let e = layout.hit_test_text_position(range.end.into());

        path.clear();
        path.push(PathEl::MoveTo(Point::new(s.point.x.ceil() + 0.5, y.ceil() + 0.5)));
        path.push(PathEl::LineTo(Point::new(e.point.x.ceil() + 0.5, y.ceil() + 0.5)));
        path.push(PathEl::LineTo(Point::new(
            e.point.x.ceil() + 0.5,
            (self.metrics.font_height + y).ceil() + 0.5,
        )));
        path.push(PathEl::LineTo(Point::new(
            s.point.x.ceil() + 0.5,
            (self.metrics.font_height + y).ceil() + 0.5,
        )));
        path.push(PathEl::ClosePath);
    }

    fn add_range_from_selection<L: TextLayout>(
        &mut self,
        y: f64,
        range: Range<position::Relative>,
        layout: &L,
        path: &mut Vec<PathEl>,
    ) -> f64 {
        let s = layout.hit_test_text_position(range.start.into());
        let e = layout.hit_test_text_position(range.end.into());

        path.clear();
        path.push(PathEl::MoveTo(Point::new(
            s.point.x.ceil() + 0.5,
            (self.metrics.font_height + y).ceil() + 0.5,
        )));
        path.push(PathEl::LineTo(Point::new(s.point.x.ceil() + 0.5, y.ceil() + 0.5)));
        path.push(PathEl::LineTo(Point::new(
            (e.point.x + self.metrics.font_advance).ceil() + 0.5,
            y.ceil() + 0.5,
        )));
        path.push(PathEl::LineTo(Point::new(
            (e.point.x + self.metrics.font_advance).ceil() + 0.5,
            (self.metrics.font_height + y).ceil() + 0.5,
        )));
        s.point.x //.ceil()+0.5
    }

    fn add_range_to_selection<L: TextLayout>(
        &mut self,
        y: f64,
        range: Range<position::Relative>,
        layout: &L,
        path: &mut SelectionPath,
    ) {
        let e = layout.hit_test_text_position(range.end.into());
        match &path.last_range {
            Some(SelectionLineRange::RangeFrom(_)) if range.end == position::Relative::from(0) => {
                path.push(PathEl::ClosePath);
            }
            Some(SelectionLineRange::RangeFull) if range.end == position::Relative::from(0) => {
                path.push(PathEl::LineTo(Point::new(0.5, y.ceil() + 0.5)));
                path.push(PathEl::ClosePath);
            }
            Some(SelectionLineRange::RangeFrom(_)) if path.last_x > e.point.x => {
                path.push(PathEl::ClosePath);
                path.push(PathEl::MoveTo(Point::new(e.point.x.ceil() + 0.5, y.ceil() + 0.5)));
                path.push(PathEl::LineTo(Point::new(
                    e.point.x.ceil() + 0.5,
                    (self.metrics.font_height + y).ceil() + 0.5,
                )));
                path.push(PathEl::LineTo(Point::new(
                    0.5,
                    (self.metrics.font_height + y).ceil() + 0.5,
                )));
                path.push(PathEl::LineTo(Point::new(0.5, y.ceil() + 0.5)));
                path.push(PathEl::ClosePath);
            }
            Some(SelectionLineRange::RangeFrom(_)) | Some(SelectionLineRange::RangeFull) => {
                path.push(PathEl::LineTo(Point::new(e.point.x.ceil() + 0.5, y.ceil() + 0.5)));
                path.push(PathEl::LineTo(Point::new(
                    e.point.x.ceil() + 0.5,
                    (self.metrics.font_height + y).ceil() + 0.5,
                )));
                path.push(PathEl::LineTo(Point::new(
                    0.5,
                    (self.metrics.font_height + y).ceil() + 0.5,
                )));
                path.push(PathEl::LineTo(Point::new(0.5, y.ceil() + 0.5)));
                path.push(PathEl::ClosePath);
            }
            None => {
                path.clear();
                path.push(PathEl::MoveTo(Point::new(0.5, y.ceil() + 0.5)));
                path.push(PathEl::LineTo(Point::new(e.point.x.ceil() + 0.5, y.ceil() + 0.5)));
                path.push(PathEl::LineTo(Point::new(
                    e.point.x.ceil() + 0.5,
                    (self.metrics.font_height + y).ceil() + 0.5,
                )));
                path.push(PathEl::LineTo(Point::new(
                    0.5,
                    (self.metrics.font_height + y).ceil() + 0.5,
                )));
                path.push(PathEl::ClosePath);
            }
            _ => {
                unreachable!();
            }
        }
    }

    fn add_range_full_selection<L: TextLayout>(
        &mut self,
        y: f64,
        range: Range<position::Relative>,
        layout: &L,
        path: &mut SelectionPath,
    ) {
        let e = layout.hit_test_text_position(range.end.into());
        match &path.last_range {
            Some(SelectionLineRange::RangeFrom(_)) if path.last_x > e.point.x => {
                path.push(PathEl::ClosePath);
                path.push(PathEl::MoveTo(Point::new(0.5, y.ceil() + 0.5)));
                path.push(PathEl::LineTo(Point::new(
                    (e.point.x + self.metrics.font_advance).ceil() + 0.5,
                    y.ceil() + 0.5,
                )));
                path.push(PathEl::LineTo(Point::new(
                    (e.point.x + self.metrics.font_advance).ceil() + 0.5,
                    (self.metrics.font_height + y).ceil() + 0.5,
                )));
            }
            Some(SelectionLineRange::RangeFrom(_)) if path.last_x <= e.point.x => {
                // insert a point at the begining of the line
                path[0] = PathEl::LineTo(Point::new(path.last_x.ceil() + 0.5, y.ceil() + 0.5));
                path.insert(0, PathEl::MoveTo(Point::new(0.5, y.ceil() + 0.5)));
                path.push(PathEl::LineTo(Point::new(
                    (e.point.x + self.metrics.font_advance).ceil() + 0.5,
                    y.ceil() + 0.5,
                )));
                path.push(PathEl::LineTo(Point::new(
                    (e.point.x + self.metrics.font_advance).ceil() + 0.5,
                    (self.metrics.font_height + y).ceil() + 0.5,
                )));
            }
            None => {
                // the precedent line was outside the visible range
                path.clear();
                path.push(PathEl::MoveTo(Point::new(0.5, y.ceil() + 0.5)));
                path.push(PathEl::LineTo(Point::new(
                    (e.point.x + self.metrics.font_advance).ceil() + 0.5,
                    y.ceil() + 0.5,
                )));
                path.push(PathEl::LineTo(Point::new(
                    (e.point.x + self.metrics.font_advance).ceil() + 0.5,
                    (self.metrics.font_height + y).ceil() + 0.5,
                )));
            }
            _ => {
                path.push(PathEl::LineTo(Point::new(
                    (e.point.x + self.metrics.font_advance).ceil() + 0.5,
                    y.ceil() + 0.5,
                )));
                path.push(PathEl::LineTo(Point::new(
                    (e.point.x + self.metrics.font_advance).ceil() + 0.5,
                    (self.metrics.font_height + y).ceil() + 0.5,
                )));
            }
        }
    }

    fn paint_editor(&mut self, editor: &EditStack, ctx: &mut PaintCtx, env: &Env) -> bool {
        let font = ctx.render_ctx.text().font_family(&self.font_name).unwrap();
        // self.size.to_rect();
        let rect = Rect::new(0.0, 0.0, self.size.width, self.size.height);
        ctx.render_ctx.fill(rect, &self.bg_color);

        let clip_rect = ctx.size().to_rect().inset((1., 0., 0., 0.));
        ctx.render_ctx.clip(clip_rect);
        ctx.render_ctx
            .transform(Affine::translate((self.delta_x + EDITOR_LEFT_PADDING, 0.0)));

        let mut line = String::new();
        let mut indices = Vec::new();
        let mut ranges = Vec::new();
        let mut selection_path = Vec::new();
        let mut current_path = SelectionPath::new();

        // Draw selection first
        // TODO: cache layout to reuse it when we will draw the text
        let mut dy = (self.delta_y / self.metrics.font_height).fract() * self.metrics.font_height;
        for line_idx in self.visible_range() {
            editor.displayable_line(position::Line::from(line_idx), &mut line, &mut indices, &mut Vec::new());
            let layout = ctx
                .render_ctx
                .text()
                .new_text_layout(line.clone()) // TODO: comment ne pas faire de clone?
                .default_attribute(TextAttribute::Weight(FONT_WEIGTH))
                .font(font.clone(), self.metrics.font_size)
                .build()
                .unwrap();

            editor.selection_on_line(line_idx, &mut ranges);

            for range in &ranges {
                match range {
                    SelectionLineRange::Range(r) => {
                        // Simple case, the selection is contain on one line
                        self.add_bounded_range_selection(
                            dy,
                            indices[r.start]..indices[r.end],
                            &layout,
                            &mut current_path,
                        )
                    }
                    SelectionLineRange::RangeFrom(r) => {
                        current_path.last_x = self.add_range_from_selection(
                            dy,
                            indices[r.start]..position::Relative::from(line.len() - 1),
                            &layout,
                            &mut current_path,
                        )
                    }
                    SelectionLineRange::RangeTo(r) => self.add_range_to_selection(
                        dy,
                        position::Relative::from(0)..indices[r.end],
                        &layout,
                        &mut current_path,
                    ),
                    SelectionLineRange::RangeFull => self.add_range_full_selection(
                        dy,
                        position::Relative::from(0)..position::Relative::from(line.len() - 1),
                        &layout,
                        &mut current_path,
                    ),
                }
                current_path.last_range = Some(range.clone());
                if let Some(PathEl::ClosePath) = current_path.last() {
                    selection_path.push(std::mem::take(&mut current_path));
                }
            }

            dy += self.metrics.font_height;
        }

        // if path is unclosed, it can only be because the lastest visible line was a RangeFull
        // We need to close it
        match current_path.last() {
            Some(PathEl::ClosePath) => (),
            _ => {
                current_path.push(PathEl::LineTo(Point::new(0.5, dy.ceil() + 0.5)));
                current_path.push(PathEl::ClosePath);
                selection_path.push(std::mem::take(&mut current_path));
            }
        }

        for path in selection_path {
            let path = BezPath::from_vec(path.elem);
            let brush = ctx.render_ctx.solid_brush(self.fg_sel_color.clone());
            ctx.render_ctx.fill(&path, &brush);
            let brush = ctx.render_ctx.solid_brush(self.bg_sel_color.clone());
            ctx.render_ctx.stroke(&path, &brush, 1.);
        }

        let mut dy = (self.delta_y / self.metrics.font_height).fract() * self.metrics.font_height;
        for line_idx in self.visible_range() {
            //editor.buffer.line(line_idx, &mut line);
            
            editor.displayable_line(position::Line::from(line_idx), &mut line, &mut indices, &mut Vec::new());
            let mut layout = ctx
                .render_ctx
                .text()
                .new_text_layout(line.clone())
                .default_attribute(TextAttribute::Weight(FONT_WEIGTH))
                .font(font.clone(), self.metrics.font_size)
                .text_color(self.fg_color.clone());
            let layout = if line_idx < editor.len_lines() {
                let highlight = self
                    .highlight_cache
                    .get_highlighted_line(editor.file.syntax, &editor.buffer, line_idx);
                for h in highlight {
                    let color = TextAttribute::TextColor(Color::rgba8(
                        h.0.foreground.r,
                        h.0.foreground.g,
                        h.0.foreground.b,
                        h.0.foreground.a,
                    ));
                    if h.0.font_style.contains(syntect::highlighting::FontStyle::ITALIC) {
                        layout = layout.range_attribute(
                            indices[h.1.start].index..indices[h.1.end].index,
                            TextAttribute::Style(FontStyle::Italic),
                        );
                    }
                    layout = layout.range_attribute(indices[h.1.start].index..indices[h.1.end].index, color);
                }
                layout
            } else {
                layout
            }
            .build()
            .unwrap();

            ctx.render_ctx.draw_text(&layout, (0.0, dy));

            self.longest_line_len = self.longest_line_len.max(layout.image_bounds().width());

            editor.carets_on_line(position::Line::from(line_idx)).for_each(|c| {
                let metrics = layout.hit_test_text_position(indices[c.relative().index].index);
                ctx.render_ctx.stroke(
                    Line::new(
                        (metrics.point.x.ceil(), (self.metrics.font_height + dy).ceil()),
                        (metrics.point.x.ceil(), dy.ceil()),
                    ),
                    &env.get(crate::theme::EDITOR_CURSOR_FOREGROUND),
                    2.0,
                );
            });

            dy += self.metrics.font_height;
        }

        false
    }

    fn pix_to_point(&self, x: f64, y: f64, ctx: &mut EventCtx, editor: &EditStack) -> (usize, usize) {
        let x = (x - self.delta_x - EDITOR_LEFT_PADDING).max(0.);
        let y = (y - self.delta_y).max(0.);
        let line = ((y / self.metrics.font_height) as usize).min(editor.len_lines() - 1);

        let mut buf = String::new();
        let mut i = Vec::new();
        editor.displayable_line(line.into(), &mut buf, &mut Vec::new(), &mut i);

        let layout = self.text_layout(ctx, buf);
        let rel = rope_utils::relative_to_column(
            i[layout.hit_test_point((x, 0.0).into()).idx],
            line.into(),
            &editor.buffer,
        )
        .index;

        (rel, line)
    }

    fn text_layout(&self, ctx: &mut EventCtx, buf: String) -> druid::piet::D2DTextLayout {
        let font = ctx.text().font_family(&self.font_name).unwrap();
        let layout = ctx
            .text()
            .new_text_layout(buf)
            .default_attribute(TextAttribute::Weight(FONT_WEIGTH))
            .font(font, self.metrics.font_size)
            .build()
            .unwrap();
        layout
    }

    fn put_caret_in_visible_range(&mut self, ctx: &mut EventCtx, editor: &EditStack) {
        if editor.has_many_carets() {
            return;
        }
        let caret = editor.main_caret();
        let y = caret.line().index as f64 * self.metrics.font_height;

        if y > -self.delta_y + self.size.height - self.metrics.font_height {
            self.delta_y = -y + self.size.height - self.metrics.font_height;
        }
        if y < -self.delta_y {
            self.delta_y = -y;
        }

        let mut buf = String::new();
        let mut i = Vec::new();
        editor.displayable_line(caret.line(), &mut buf, &mut i, &mut Vec::new());

        let layout = self.text_layout(ctx, buf);

        let hit = layout.hit_test_text_position(i[caret.relative().index].index);
        let x = hit.point.x;
        if x > -self.delta_x + self.size.width - self.metrics.font_advance - EDITOR_LEFT_PADDING {
            self.delta_x = -x + self.size.width - self.metrics.font_advance - EDITOR_LEFT_PADDING;
        }
        if x < -self.delta_x {
            self.delta_x = -x;
        }
        ctx.submit_command(Command::new(
            commands::SCROLL_TO,
            (Some(self.delta_x), Some(self.delta_y)),
            self.owner_id,
        ));
    }

    fn invalidate_highlight_cache(&mut self, editor: &EditStack) {
        let line = editor.main_caret().start_line(&editor.buffer);
        self.highlight_cache.invalidate(line.index)
    }

    fn save_as<P>(&mut self, editor: &mut EditStack, filename: P) -> anyhow::Result<()>
    where
        P: AsRef<Path>,
    {
        if filename.as_ref().exists()
            && MessageDialog::new()
                .set_level(rfd::MessageLevel::Warning)
                .set_title("File Exists")
                .set_description("The given file allready exists, are you sure you want to overwrite it?")
                .set_buttons(rfd::MessageButtons::YesNo)
                .show()
        {
            return Ok(());
        }

        editor.save(&filename)?;
        editor.filename = Some(filename.as_ref().to_path_buf());

        Ok(())
    }

    fn save(&mut self, editor: &mut EditStack) -> anyhow::Result<()> {
        anyhow::ensure!(editor.filename.is_some(), "editor.filename must not be None");
        editor.save(editor.filename.clone().as_ref().unwrap())?;
        Ok(())
    }
    fn open<P>(&mut self, editor: &mut EditStack, filename: P) -> anyhow::Result<()>
    where
        P: AsRef<Path>,
    {
        editor.open(filename)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Gutter {
    metrics: CommonMetrics,
    font_name: String,
    page_len: usize,
    size: Size,
    dy: f64,
    owner_id: WidgetId,
    is_held: bool,
}

impl Widget<EditStack> for Gutter {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EditStack, _env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(commands::SCROLL_TO) => {
                self.dy = cmd.get_unchecked(commands::SCROLL_TO).1.unwrap_or(self.dy);
                // .clamp(-self.height(&data), 0.);
                ctx.request_paint();
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(commands::RESET_HELD_STATE) => {
                self.is_held = false;
            }
            Event::MouseMove(m) => {
                //ctx.set_cursor(&Cursor::ResizeLeftRight);
                if self.is_held && ctx.is_active() && m.buttons.contains(MouseButton::Left) {
                    let y = (m.pos.y - self.dy).max(0.);
                    let line = ((y / self.metrics.font_height) as usize).min(data.len_lines() - 1);
                    ctx.submit_command(Command::new(commands::SELECT_LINE, (line, true), self.owner_id));
                    ctx.request_paint();
                    ctx.set_handled();
                }
            }
            Event::MouseDown(m) => {
                ctx.set_active(true);
                let y = (m.pos.y - self.dy).max(0.);
                let line = ((y / self.metrics.font_height) as usize).min(data.len_lines() - 1);
                ctx.submit_command(Command::new(
                    commands::SELECT_LINE,
                    (line, m.mods.shift()),
                    self.owner_id,
                ));
                ctx.request_paint();
                ctx.set_handled();
                self.is_held = true;
            }
            Event::MouseUp(_event) => {
                ctx.set_active(false);
                ctx.set_handled();
                self.is_held = false;
            }
            Event::Wheel(m) => {
                //self.dy -= m.wheel_delta.y;
                ctx.submit_command(Command::new(
                    SCROLL_TO,
                    (None, Some(self.dy - m.wheel_delta.y)),
                    self.owner_id,
                ));
                ctx.request_paint();
                ctx.set_handled();
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &EditStack, _env: &Env) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &EditStack, _data: &EditStack, _env: &Env) {}

    fn layout(&mut self, _layout_ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &EditStack, _env: &Env) -> Size {
        self.metrics = CommonMetrics::from_env(_env);
        self.size = Size::new(self.width(data), bc.max().height);

        self.page_len = (self.size.height / self.metrics.font_height).round() as usize;
        self.size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EditStack, env: &Env) {
        self.paint_gutter(data, ctx, env)
    }
}

impl Gutter {
    fn new(owner_id: WidgetId) -> Self {
        Gutter {
            metrics: Default::default(),
            font_name: FONT_NAME.to_string(),
            page_len: 0,
            size: Default::default(),
            dy: 0.0,
            owner_id,
            is_held: false,
        }
    }
    fn visible_line_range(&self) -> Range<usize> {
        let start = -(self.dy / self.metrics.font_height) as usize;
        let end = start + self.page_len + 1;
        Range { start, end }
    }

    fn width(&self, data: &EditStack) -> f64 {
        (data.len_lines().to_string().chars().count() as f64 + 3.0) * self.metrics.font_advance
    }
    fn paint_gutter(&mut self, editor: &EditStack, ctx: &mut PaintCtx, env: &Env) {
        ctx.clip(self.size.to_rect());
        ctx.render_ctx
            .fill(self.size.to_rect(), &env.get(crate::theme::EDITOR_BACKGROUND));
        // Draw line number
        let font = ctx.text().font_family(FONT_NAME).unwrap();
        let mut dy = (self.dy / self.metrics.font_height).fract() * self.metrics.font_height;
        let line_number_char_width = format!(" {}", editor.len_lines()).len();
        for line_idx in self.visible_line_range() {
            if line_idx >= editor.len_lines() {
                break;
            }
            let layout = ctx
                .render_ctx
                .text()
                .new_text_layout(format!("{:1$}", line_idx, line_number_char_width))
                .default_attribute(TextAttribute::Weight(FONT_WEIGTH))
                .font(font.clone(), self.metrics.font_size)
                .text_color(env.get(crate::theme::EDITOR_LINE_NUMBER_FOREGROUND))
                .build()
                .unwrap();
            ctx.render_ctx.draw_text(&layout, (0.0, dy));
            dy += self.metrics.font_height;
        }

        ctx.render_ctx.stroke(
            Line::new(
                ((self.width(editor) - self.metrics.font_advance).ceil() + 0.5, 0.),
                (
                    (self.width(editor) - self.metrics.font_advance).ceil() + 0.5,
                    self.size.height,
                ),
            ),
            &env.get(crate::theme::EDITOR_LINE_NUMBER_FOREGROUND),
            1.0,
        );
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ScrollBarDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
struct ScrollBar {
    owner_id: WidgetId,
    direction: ScrollBarDirection,
    len: f64,
    range: Range<f64>,
    metrics: CommonMetrics,
    mouse_delta: f64,
    is_held: bool,
    is_hovered: bool,
    delta: f64,
}

impl ScrollBar {
    fn new(owner_id: WidgetId, direction: ScrollBarDirection) -> Self {
        Self {
            owner_id,
            direction,
            len: 0.,
            range: Range { start: 0., end: 0. },
            metrics: Default::default(),
            mouse_delta: 0.,
            is_held: false,
            is_hovered: false,
            delta: 0.,
        }
    }

    fn text_len(&self, data: &EditStack) -> f64 {
        if self.is_vertical() {
            data.len_lines().saturating_sub(3) as f64 * self.metrics.font_height
        } else {
            data.buffer.max_visible_line_grapheme_len().saturating_sub(3) as f64 * self.metrics.font_advance
        }
    }

    fn handle_len(&self, data: &EditStack) -> f64 {
        (self.len.powi(2) / (self.text_len(data) + self.len)).max(self.metrics.font_height)
    }

    fn effective_len(&self, data: &EditStack) -> f64 {
        self.len - self.handle_len(data)
    }

    fn rect(&self) -> Rect {
        if self.is_vertical() {
            Rect::new(
                0.0,
                self.range.start,
                self.metrics.font_advance + SCROLLBAR_X_PADDING,
                self.range.end,
            )
        } else {
            Rect::new(
                self.range.start,
                0.0,
                self.range.end,
                self.metrics.font_advance + SCROLLBAR_X_PADDING,
            )
        }
    }

    fn is_vertical(&self) -> bool {
        self.direction == ScrollBarDirection::Vertical
    }
}
impl Widget<EditStack> for ScrollBar {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EditStack, _env: &Env) {
        // if self.text_len(data) < 0.1 {
        //     return;
        // }
        match event {
            Event::Command(cmd) if cmd.is(commands::SCROLL_TO) => {
                if self.text_len(data) < 0.1 {
                    return;
                }
                if self.is_vertical() {
                    if let Some(dy) = cmd.get_unchecked(commands::SCROLL_TO).1 {
                        self.delta = dy;
                        let len = self.effective_len(data);
                        let dy = dy * self.len / self.text_len(data);

                        self.range.start = -dy * len / self.len;
                        self.range.end = self.range.start + self.handle_len(data);

                        ctx.request_paint();
                        ctx.set_handled();
                    }
                } else if let Some(dx) = cmd.get_unchecked(commands::SCROLL_TO).0 {
                    self.delta = dx;
                    let len = self.effective_len(data);

                    let dx = dx * self.len / self.text_len(data);
                    self.range.start = -dx * len / self.len;
                    self.range.end = self.range.start + self.handle_len(data);

                    ctx.request_paint();
                    ctx.set_handled();
                }
            }
            Event::Command(cmd) if cmd.is(commands::RESET_HELD_STATE) => {
                self.is_held = false;
                self.is_hovered = false;
            }
            Event::MouseDown(m) => {
                if self.rect().contains(Point::new(m.pos.x, m.pos.y)) {
                    ctx.set_active(true);
                    self.mouse_delta = if self.is_vertical() {
                        m.pos.y - self.range.start
                    } else {
                        m.pos.x - self.range.start
                    };
                    ctx.request_paint();
                    ctx.set_handled();
                    self.is_held = true;
                }
            }
            Event::MouseUp(_) => {
                ctx.set_active(false);
                ctx.set_handled();
                self.is_held = false;
            }
            Event::MouseMove(m) => {
                if self.rect().contains(Point::new(m.pos.x, m.pos.y)) {
                    self.is_hovered = true;
                } else {
                    self.is_hovered = false;
                }
                if self.is_held && ctx.is_active() && m.buttons.contains(MouseButton::Left) {
                    if self.is_vertical() {
                        ctx.submit_command(Command::new(
                            SCROLL_TO,
                            (
                                None,
                                Some((self.mouse_delta - m.pos.y) * self.text_len(data) / self.effective_len(data)),
                            ),
                            self.owner_id,
                        ));
                    } else {
                        ctx.submit_command(Command::new(
                            SCROLL_TO,
                            (
                                Some((self.mouse_delta - m.pos.x) * self.text_len(data) / self.effective_len(data)),
                                None,
                            ),
                            self.owner_id,
                        ));
                    }

                    ctx.set_handled();
                }
                ctx.request_paint();
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &EditStack, _env: &Env) {
        //todo!()
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &EditStack, _data: &EditStack, _env: &Env) {
        //todo!()
    }

    fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &EditStack, env: &Env) -> Size {
        self.metrics = CommonMetrics::from_env(env);

        self.len = if self.is_vertical() {
            bc.max().height
        } else {
            bc.max().width
        };

        self.range = if self.text_len(data) < 0.1 {
            Range {
                start: 0.,
                end: self.len,
            }
        } else {
            let start = -self.delta * (self.effective_len(data)) / self.text_len(data);
            let end = start + self.handle_len(data);
            Range { start, end }
        };
        // let weight = if self.text_len(data) <= 0.1 {
        //     0.
        // } else {
        //     self.metrics.font_advance
        // };
        if self.is_vertical() {
            Size::new(self.metrics.font_advance + SCROLLBAR_X_PADDING, self.len)
        } else {
            Size::new(self.len, self.metrics.font_advance + SCROLLBAR_X_PADDING)
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &EditStack, env: &Env) {
        //if self.is_vertical() {
        let r = ctx.size().to_rect();
        ctx.clip(r);
        ctx.fill(r, &env.get(crate::theme::EDITOR_BACKGROUND));
        if self.is_held {
            ctx.fill(
                self.rect().inflate(-1.0, -1.0).to_rounded_rect(3.),
                &env.get(crate::theme::SCROLLBAR_SLIDER_ACTIVE_BACKGROUND),
            );
        } else if self.is_hovered {
            ctx.fill(
                self.rect().inflate(-1.0, -1.0).to_rounded_rect(3.),
                &env.get(crate::theme::SCROLLBAR_SLIDER_HOVER_BACKGROUND),
            );
        } else {
            ctx.fill(
                self.rect().inflate(-1.0, -1.0).to_rounded_rect(3.),
                &env.get(crate::theme::SCROLLBAR_SLIDER_BACKGROUND),
            );
        }
        //}
    }
}

#[derive(Debug, Default)]
struct ScrollBarSpacer {
    metrics: CommonMetrics,
}

impl Widget<EditStack> for ScrollBarSpacer {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut EditStack, _env: &Env) {}

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &EditStack, _env: &Env) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &EditStack, _data: &EditStack, _env: &Env) {}

    fn layout(&mut self, _ctx: &mut LayoutCtx, _bc: &BoxConstraints, _data: &EditStack, env: &Env) -> Size {
        self.metrics = CommonMetrics::from_env(env);
        Size::new(
            self.metrics.font_advance + SCROLLBAR_X_PADDING,
            self.metrics.font_advance + SCROLLBAR_X_PADDING,
        )
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &EditStack, env: &Env) {
        let r = ctx.size().to_rect();
        ctx.clip(r);
        ctx.fill(r, &env.get(crate::theme::EDITOR_BACKGROUND));
    }
}

struct TextEditor {
    gutter_id: WidgetId,
    editor_id: WidgetId,
    vscroll_id: WidgetId,
    hscroll_id: WidgetId,
    id: WidgetId,
    inner: Flex<EditStack>,
    metrics: CommonMetrics,
}

impl TextEditor {
    pub fn text_height(&self, data: &EditStack) -> f64 {
        data.len_lines().saturating_sub(3) as f64 * self.metrics.font_height
    }
    pub fn text_width(&self, data: &EditStack) -> f64 {
        data.buffer.max_visible_line_grapheme_len().saturating_sub(3) as f64 * self.metrics.font_advance
    }
}

impl Default for TextEditor {
    fn default() -> Self {
        let id = WidgetId::next();
        let gutter_id = WidgetId::next();
        let editor_id = WidgetId::next();
        let vscroll_id = WidgetId::next();
        let hscroll_id = WidgetId::next();
        TextEditor {
            gutter_id,
            editor_id,
            vscroll_id,
            hscroll_id,
            id,
            inner: Flex::row()
                .with_child(Gutter::new(id).with_id(gutter_id))
                .with_flex_child(
                    Flex::column()
                        .with_flex_child(EditorView::new(id).with_id(editor_id), 1.0)
                        .with_child(ScrollBar::new(id, ScrollBarDirection::Horizontal).with_id(hscroll_id)),
                    1.0,
                )
                .must_fill_main_axis(true)
                .with_child(
                    Flex::column()
                        .with_flex_child(
                            ScrollBar::new(id, ScrollBarDirection::Vertical).with_id(vscroll_id),
                            1.0,
                        )
                        .with_child(ScrollBarSpacer::default()),
                ),
            metrics: Default::default(),
        }
    }
}

impl Widget<EditStack> for TextEditor {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EditStack, env: &Env) {
        let mut new_env = env.clone();
        self.metrics.to_env(&mut new_env);
        match event {
            Event::Command(cmd) if cmd.is(commands::SCROLL_TO) => {
                // clamp to size
                let d = *cmd.get_unchecked(commands::SCROLL_TO);
                let x = d.0.map(|x| x.clamp(-self.text_width(&data), 0.0));
                let y = d.1.map(|y| y.clamp(-self.text_height(&data), 0.0));
                // dbg!(x,y);
                ctx.submit_command(Command::new(SCROLL_TO, (x, y), self.editor_id));
                ctx.submit_command(Command::new(SCROLL_TO, (x, y), self.gutter_id));
                ctx.submit_command(Command::new(SCROLL_TO, (x, y), self.vscroll_id));
                ctx.submit_command(Command::new(SCROLL_TO, (x, y), self.hscroll_id));
                ctx.is_handled();
            }

            _ => self.inner.event(ctx, event, data, &new_env),
        }
        //self.inner.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &EditStack, env: &Env) {
        let mut new_env = env.clone();
        self.metrics.to_env(&mut new_env);

        self.inner.lifecycle(ctx, event, data, &new_env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &EditStack, data: &EditStack, env: &Env) {
        let mut new_env = env.clone();
        self.metrics.to_env(&mut new_env);
        self.inner.update(ctx, old_data, data, &new_env)
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &EditStack, env: &Env) -> Size {
        self.metrics = CommonMetrics::new(ctx.text(), FONT_NAME, bc.max());
        let mut new_env = env.clone();
        self.metrics.to_env(&mut new_env);
        self.inner.layout(ctx, bc, data, &new_env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EditStack, env: &Env) {
        let mut new_env = env.clone();
        self.metrics.to_env(&mut new_env);
        self.inner.paint(ctx, data, &new_env)
    }
}

pub fn new() -> impl Widget<EditStack> {
    let t = TextEditor::default();
    let id = t.id;
    t.with_id(id)
}
