use std::ops::{Deref, DerefMut, Range};
use std::path::Path;

use crate::commands::{NEW_METRICS, SCROLL_TO};
use crate::text_buffer::{position, rope_utils};
use crate::text_buffer::{EditStack, SelectionLineRange};
use crate::{commands, MainWindowState};
use druid::keyboard_types::CompositionEvent;
use druid::widget::{Controller, Flex, IdentityWrapper, Scroll};
use druid::{
    kurbo::{BezPath, Line, PathEl, Point, Rect, Size},
    FontDescriptor, WidgetExt,
};
use druid::{
    piet::{PietText, RenderContext, Text, TextLayout, TextLayoutBuilder},
    Affine, Application, BoxConstraints, ClipboardFormat, Color, Command, Env, Event, EventCtx, FileDialogOptions,
    HotKey, Key, KeyEvent, LayoutCtx, LifeCycle, LifeCycleCtx, MouseButton, PaintCtx, SysMods, Target, UpdateCtx,
    Widget, WidgetId,
};
use druid::{Data, Vec2, WindowConfig};
use rfd::MessageDialog;

pub const FONT_SIZE: Key<f64> = Key::new("nonepad.editor.font_height");
pub const FONT_ADVANCE: Key<f64> = Key::new("nonepad.editor.font_advance");
pub const FONT_BASELINE: Key<f64> = Key::new("nonepad.editor.font_baseline");
pub const FONT_DESCENT: Key<f64> = Key::new("nonepad.editor.font_descent");
pub const FONT_HEIGHT: Key<f64> = Key::new("nonepad.editor.fonth_height");
pub const PAGE_LEN: Key<u64> = Key::new("nonepad.editor.page_len");


//pub const FONT_NAME: Key<druid::Value> = Key::new("nonepad.editor.font_name");
pub const FONT_NAME: &'static str = "Consolas";

pub const FONT_DESCRIPTOR: Key<FontDescriptor> = Key::new("nonepad.editor.font_descriptor");
pub const BG_COLOR: Key<Color> = Key::new("nondepad.editor.fg_color");
pub const FG_COLOR: Key<Color> = Key::new("nondepad.editor.bg_color");
pub const FG_SEL_COLOR: Key<Color> = Key::new("nondepad.editor.fg_selection_color");
pub const BG_SEL_COLOR: Key<Color> = Key::new("nondepad.editor.bg_selection_color");

pub const EDITOR_WIDGET_ID: WidgetId = WidgetId::reserved(0xED17);
pub const GUTTER_WIDGET_ID: WidgetId = WidgetId::reserved(0x8077);

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
            last_x: 0.,
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
    pub fn new(text_ctx: &mut PietText, font_name: &str,size: Size) -> Self {
        let mut metrics = CommonMetrics::default();
        let font = text_ctx.font_family(font_name).unwrap();
        let layout = text_ctx
            .new_text_layout("8")
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
            font_baseline : env.get(FONT_BASELINE),
            font_advance : env.get(FONT_ADVANCE),
            font_descent : env.get(FONT_DESCENT),
            font_height : env.get(FONT_HEIGHT),
            font_size : env.get(FONT_SIZE),
            page_len : env.get(PAGE_LEN),
        }
    }

    pub fn to_env(&self, env: &mut Env) {
        env.set(FONT_BASELINE, self.font_baseline);
        env.set(FONT_ADVANCE, self.font_advance);
        env.set(FONT_DESCENT, self.font_descent);
        env.set(FONT_HEIGHT, self.font_height);
        env.set(FONT_SIZE, self.font_size);
        env.set(PAGE_LEN, self.page_len);
    }
}

impl Default for CommonMetrics {
    fn default() -> Self {
        CommonMetrics {
            font_advance: 0.0,
            font_baseline: 0.0,
            font_descent: 0.0,
            font_height: 0.0,
            font_size: 12.0,
            page_len: 0,
        }
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
    //handle: WindowHandle,
}

impl Widget<EditStack> for EditorView {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, editor: &mut EditStack, _env: &Env) {
        match event {
            Event::WindowConnected => {
                ctx.request_focus();
            }
            Event::KeyDown(event) => {
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
                    self.put_caret_in_visible_range(ctx, editor);
                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }
                if HotKey::new(None, druid::keyboard_types::Key::Delete).matches(event) {
                    editor.delete();
                    self.put_caret_in_visible_range(ctx, editor);
                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }

                if HotKey::new(None, druid::keyboard_types::Key::Enter).matches(event) {
                    // || HotKey::new(None, druid::keyboard_types::Key::Return).matches(event) {
                    editor.insert(editor.file.linefeed.to_str());
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
                    self.put_caret_in_visible_range(ctx, editor);
                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }
                if HotKey::new(SysMods::Cmd, "y").matches(event) {
                    editor.redo();
                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }
                if HotKey::new(SysMods::Cmd, "o").matches(event) {
                    if editor.is_dirty()
                        && MessageDialog::new()
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
                    ctx.submit_command(Command::new(crate::commands::SHOW_SEARCH_PANEL, (), Target::Global));

                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }

                if let druid::keyboard_types::Key::Character(text) = event.key.clone() {
                    if event.mods.ctrl() || event.mods.alt() || event.mods.meta() {
                        return;
                    }
                    if text.chars().count() == 1 && text.chars().next().unwrap().is_ascii_control() {
                        return;
                    }
                    editor.insert(&text);
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
                    dbg!((Some(self.delta_x - event.wheel_delta.x), Some(self.delta_y - event.wheel_delta.y))),
                    self.owner_id,
                ));

                // self.delta_x -= event.wheel_delta.x;
                // if self.delta_x > 0. {
                //     self.delta_x = 0.;
                // }
                if ctx.is_active() && event.buttons.contains(MouseButton::Left) {
                    let (x, y) = self.pix_to_point(event.pos.x, event.pos.y, ctx, editor);
                    let p = editor.point(x, y);
                    editor.move_main_caret_to(p, true);
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
                    editor.move_main_caret_to(p, event.mods.shift());
                    ctx.set_active(true);
                }
                ctx.request_focus();
                ctx.request_paint();
                ctx.set_handled();
            }
            Event::MouseUp(_event) => {
                ctx.set_active(false);
                ctx.set_handled();
            }
            Event::MouseMove(event) => {
                if ctx.is_active() && event.buttons.contains(MouseButton::Left) {
                    let (x, y) = self.pix_to_point(event.pos.x, event.pos.y, ctx, editor);
                    let p = editor.point(x, y);
                    editor.move_main_caret_to(p, true);
                    self.put_caret_in_visible_range(ctx, editor);
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
            Event::Command(cmd) if cmd.is(crate::commands::REQUEST_NEXT_SEARCH) => {
                if let Some(data) = cmd.get(crate::commands::REQUEST_NEXT_SEARCH) {
                    editor.search_next(data);
                    self.put_caret_in_visible_range(ctx, editor);
                }
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(crate::commands::GIVE_FOCUS) => ctx.request_focus(),
            Event::Command(cmd) if cmd.is(crate::commands::SELECT_LINE) => {
                let (line, expand) = *cmd.get_unchecked(crate::commands::SELECT_LINE);
                editor.buffer.select_line(line.into(), expand);
                self.put_caret_in_visible_range(ctx, editor);
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(crate::commands::SCROLL_TO) => {
                let d = dbg!(*cmd.get_unchecked(crate::commands::SCROLL_TO));
                self.delta_x = d.0.unwrap_or(self.delta_x);
                self.delta_y = d.1.unwrap_or(self.delta_y);
                ctx.set_handled();
                ctx.request_paint();
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _data: &EditStack, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            self.bg_color = env.get(BG_COLOR);
            self.fg_color = env.get(FG_COLOR);
            self.fg_sel_color = env.get(FG_SEL_COLOR);
            self.bg_sel_color = env.get(BG_SEL_COLOR);

            ctx.register_for_focus();
        }
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &EditStack, _data: &EditStack, _env: &Env) {}

    fn layout(&mut self, layout_ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &EditStack, _env: &Env) -> Size {
        self.size = bc.max();

        self.metrics = CommonMetrics::new(layout_ctx.text(), &self.font_name, self.size);
        self.page_len = (self.size.height / self.metrics.font_height).round() as usize;

        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EditStack, env: &Env) {
        dbg!(self.delta_x, self.delta_y);
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
        path.push(PathEl::MoveTo(Point::new(s.point.x, y)));
        path.push(PathEl::LineTo(Point::new(e.point.x, y)));
        path.push(PathEl::LineTo(Point::new(e.point.x, self.metrics.font_height + y)));
        path.push(PathEl::LineTo(Point::new(s.point.x, self.metrics.font_height + y)));
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
        path.push(PathEl::MoveTo(Point::new(s.point.x, self.metrics.font_height + y)));
        path.push(PathEl::LineTo(Point::new(s.point.x, y)));
        path.push(PathEl::LineTo(Point::new(e.point.x + self.metrics.font_advance, y)));
        path.push(PathEl::LineTo(Point::new(
            e.point.x + self.metrics.font_advance,
            self.metrics.font_height + y,
        )));
        s.point.x
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
                path.push(PathEl::LineTo(Point::new(0., y)));
                path.push(PathEl::ClosePath);
            }
            Some(SelectionLineRange::RangeFrom(_)) if path.last_x > e.point.x => {
                path.push(PathEl::ClosePath);
                path.push(PathEl::MoveTo(Point::new(e.point.x, y)));
                path.push(PathEl::LineTo(Point::new(e.point.x, self.metrics.font_height + y)));
                path.push(PathEl::LineTo(Point::new(0., self.metrics.font_height + y)));
                path.push(PathEl::LineTo(Point::new(0., y)));
                path.push(PathEl::ClosePath);
            }
            Some(SelectionLineRange::RangeFrom(_)) | Some(SelectionLineRange::RangeFull) => {
                path.push(PathEl::LineTo(Point::new(e.point.x, y)));
                path.push(PathEl::LineTo(Point::new(e.point.x, self.metrics.font_height + y)));
                path.push(PathEl::LineTo(Point::new(0., self.metrics.font_height + y)));
                path.push(PathEl::LineTo(Point::new(0., y)));
                path.push(PathEl::ClosePath);
            }
            None => {
                path.clear();
                path.push(PathEl::MoveTo(Point::new(0., y)));
                path.push(PathEl::LineTo(Point::new(e.point.x, y)));
                path.push(PathEl::LineTo(Point::new(e.point.x, self.metrics.font_height + y)));
                path.push(PathEl::LineTo(Point::new(0., self.metrics.font_height + y)));
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
                path.push(PathEl::MoveTo(Point::new(0., y)));
                path.push(PathEl::LineTo(Point::new(e.point.x + self.metrics.font_advance, y)));
                path.push(PathEl::LineTo(Point::new(
                    e.point.x + self.metrics.font_advance,
                    self.metrics.font_height + y,
                )));
            }
            Some(SelectionLineRange::RangeFrom(_)) if path.last_x <= e.point.x => {
                // insert a point at the begining of the line
                path[0] = PathEl::LineTo(Point::new(path.last_x, y));
                path.insert(0, PathEl::MoveTo(Point::new(0., y)));
                path.push(PathEl::LineTo(Point::new(e.point.x + self.metrics.font_advance, y)));
                path.push(PathEl::LineTo(Point::new(
                    e.point.x + self.metrics.font_advance,
                    self.metrics.font_height + y,
                )));
            }
            None => {
                // the precedent line was outside the visible range
                path.clear();
                path.push(PathEl::MoveTo(Point::new(0., y)));
                path.push(PathEl::LineTo(Point::new(e.point.x + self.metrics.font_advance, y)));
                path.push(PathEl::LineTo(Point::new(
                    e.point.x + self.metrics.font_advance,
                    self.metrics.font_height + y,
                )));
            }
            _ => {
                path.push(PathEl::LineTo(Point::new(e.point.x + self.metrics.font_advance, y)));
                path.push(PathEl::LineTo(Point::new(
                    e.point.x + self.metrics.font_advance,
                    self.metrics.font_height + y,
                )));
            }
        }
    }

    fn paint_editor(&mut self, editor: &EditStack, ctx: &mut PaintCtx, _env: &Env) -> bool {
        let font = ctx.render_ctx.text().font_family(&self.font_name).unwrap();
        // self.size.to_rect();
        let rect = Rect::new(0.0, 0.0, self.size.width, self.size.height);
        ctx.render_ctx.fill(rect, &self.bg_color);

        let clip_rect = ctx.size().to_rect();
        ctx.render_ctx.clip(clip_rect);
        ctx.render_ctx.transform(Affine::translate((self.delta_x, 0.0)));

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
                current_path.push(PathEl::LineTo(Point::new(0., dy)));
                current_path.push(PathEl::ClosePath);
                selection_path.push(std::mem::take(&mut current_path));
            }
        }

        for path in selection_path {
            let path = BezPath::from_vec(path.elem);
            let brush = ctx.render_ctx.solid_brush(self.fg_sel_color.clone());
            ctx.render_ctx.fill(&path, &brush);
            let brush = ctx.render_ctx.solid_brush(self.bg_sel_color.clone());
            ctx.render_ctx.stroke(&path, &brush, 0.5);
        }

        let mut dy = (self.delta_y / self.metrics.font_height).fract() * self.metrics.font_height;
        for line_idx in self.visible_range() {
            //editor.buffer.line(line_idx, &mut line);
            editor.displayable_line(position::Line::from(line_idx), &mut line, &mut indices, &mut Vec::new());
            let layout = ctx
                .render_ctx
                .text()
                .new_text_layout(line.clone())
                .font(font.clone(), self.metrics.font_size)
                .text_color(self.fg_color.clone())
                .build()
                .unwrap();

            ctx.render_ctx.draw_text(&layout, (0.0, dy));

            editor.carets_on_line(position::Line::from(line_idx)).for_each(|c| {
                let metrics = layout.hit_test_text_position(indices[c.relative().index].index);
                ctx.render_ctx.stroke(
                    Line::new(
                        (metrics.point.x + 1.0, self.metrics.font_height + dy),
                        (metrics.point.x + 1.0, dy),
                    ),
                    &self.fg_color,
                    2.0,
                );
            });

            dy += self.metrics.font_height;
        }

        false
    }

    fn pix_to_point(&self, x: f64, y: f64, ctx: &mut EventCtx, editor: &EditStack) -> (usize, usize) {
        let x = (x - self.delta_x).max(0.);
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
        if x > -self.delta_x + self.size.width - self.metrics.font_advance {
            self.delta_x = -x + self.size.width - self.metrics.font_advance;
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
}

impl Widget<EditStack> for Gutter {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EditStack, _env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(commands::SCROLL_TO) => {
                self.dy = dbg!(cmd.get_unchecked(commands::SCROLL_TO))
                    .1
                    .unwrap_or(self.dy);
                   // .clamp(-self.height(&data), 0.);
                ctx.request_paint();
                ctx.set_handled();
            }
            Event::MouseMove(m) => {
                //ctx.set_cursor(&Cursor::ResizeLeftRight);
                if ctx.is_active() && m.buttons.contains(MouseButton::Left) {
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
            }
            Event::MouseUp(_event) => {
                ctx.set_active(false);
                ctx.set_handled();
            }
            Event::Wheel(m) => {
                //self.dy -= m.wheel_delta.y;
                ctx.submit_command(Command::new(SCROLL_TO, (None, Some(self.dy -  m.wheel_delta.y)), self.owner_id));
                ctx.request_paint();
                ctx.set_handled();
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &EditStack, _env: &Env) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &EditStack, _data: &EditStack, _env: &Env) {}

    fn layout(&mut self, layout_ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &EditStack, _env: &Env) -> Size {
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
        }
    }
    fn visible_line_range(&self) -> Range<usize> {
        let start = -(self.dy / self.metrics.font_height) as usize;
        let end = start + self.page_len + 1;
        Range { start, end }
    }
    fn visible_width(&self) -> f64 {
        self.size.width
    }
    fn visible_height(&self) -> f64 {
        self.size.height
    }
    
    fn height(&self, data: &EditStack) -> f64 {
        // Reserve 3 visibles lines
        (data.len_lines() - 3) as f64 * self.metrics.font_height
    }
    fn width(&self, data: &EditStack) -> f64 {
        (data.len_lines().to_string().chars().count() as f64 + 3.0) * self.metrics.font_advance
    }
    fn paint_gutter(&mut self, editor: &EditStack, ctx: &mut PaintCtx, env: &Env) {
        ctx.clip(self.size.to_rect());
        ctx.render_ctx.fill(self.size.to_rect(), &env.get(BG_COLOR));
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
                .font(font.clone(), self.metrics.font_size)
                .text_color(env.get(FG_COLOR))
                .build()
                .unwrap();
            ctx.render_ctx.draw_text(&layout, (0.0, dy));
            dy += self.metrics.font_height;
        }

        ctx.render_ctx.stroke(
            Line::new(
                (self.width(editor) - self.metrics.font_advance, 0.0),
                (self.width(editor), self.size.height),
            ),
            &env.get(FG_COLOR),
            1.0,
        );
    }
}

#[derive(Debug)]
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
}

impl ScrollBar {
    fn new(owner_id: WidgetId, direction: ScrollBarDirection) -> Self {
        Self {
            owner_id,
            direction,
            len: 0.,
            range: Range { start: 0., end: 0. },
        }
    }
}

impl Widget<EditStack> for ScrollBar {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut EditStack, env: &Env) {
        todo!()
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &EditStack, env: &Env) {
        todo!()
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &EditStack, data: &EditStack, env: &Env) {
        todo!()
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &EditStack, env: &Env) -> Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EditStack, env: &Env) {}
}

struct TextEditor {
    gutter_id: WidgetId,
    editor_id: WidgetId,
    id: WidgetId,
    inner: Flex<EditStack>,
    metrics: CommonMetrics,
}

impl TextEditor {
    pub fn text_height(&self, data: &EditStack) -> f64 {
        (data.len_lines() - 3) as f64 * self.metrics.font_height
    }
}

impl Default for TextEditor {
    fn default() -> Self {
        let id = WidgetId::next();
        let gutter_id = WidgetId::next();
        let editor_id = WidgetId::next();
        TextEditor {
            gutter_id,
            editor_id,
            id,
            inner: Flex::row()
                .with_child(Gutter::new(id).with_id(gutter_id))
                .with_flex_child(EditorView::new(id).with_id(editor_id), 1.0)
                .must_fill_main_axis(true),
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
                dbg!(d);
                let x = d.0.map(|x| x.clamp(-ctx.size().width, 0.0));
                let y = d.1.map(|y| y.clamp(-self.text_height(&data), 0.0));
                // dbg!(x,y);
                ctx.submit_command(Command::new(SCROLL_TO, (x,y), self.editor_id));
                ctx.submit_command(Command::new(SCROLL_TO, (x,y), self.gutter_id));
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
    let id = t.id.clone();
    t.with_id(id)
}
