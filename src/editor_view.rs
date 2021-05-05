use std::ops::{Deref, DerefMut, Range};
use std::path::Path;

use druid::piet::{RenderContext, Text, TextLayout, TextLayoutBuilder};
use druid::{
    kurbo::{BezPath, Line, PathEl, Point, Rect, Size},
    FontDescriptor,
};
use druid::{
    Affine, Application, BoxConstraints, ClipboardFormat, Color, Command, Env, Event, EventCtx, FileDialogOptions,
    HotKey, Key, KeyEvent, LayoutCtx, LifeCycle, LifeCycleCtx, MouseButton, PaintCtx, SysMods, Target, UpdateCtx,
    Widget, WidgetId,
};
use rfd::MessageDialog;
use crate::text_buffer::position;
use crate::text_buffer::{EditStack, SelectionLineRange};

pub const FONT_SIZE: Key<f64> = Key::new("nonepad.editor.font_height");
//pub const FONT_NAME: Key<druid::Value> = Key::new("nonepad.editor.font_name");

pub const FONT_DESCRIPTOR: Key<FontDescriptor> = Key::new("nonepad.editor.font_descriptor");
pub const BG_COLOR: Key<Color> = Key::new("nondepad.editor.fg_color");
pub const FG_COLOR: Key<Color> = Key::new("nondepad.editor.bg_color");
pub const FG_SEL_COLOR: Key<Color> = Key::new("nondepad.editor.fg_selection_color");
pub const BG_SEL_COLOR: Key<Color> = Key::new("nondepad.editor.bg_selection_color");

pub const WIDGET_ID: WidgetId = WidgetId::reserved(0xED17);

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
#[derive(Debug)]
pub struct EditorView {
    //editor: EditStack,
    delta_y: f64,
    delta_x: f64,
    page_len: usize,
    font_advance: f64,
    font_baseline: f64,
    font_descent: f64,
    font_height: f64,
    font_name: String,
    font_size: f64,

    bg_color: Color,
    fg_color: Color,
    fg_sel_color: Color,
    bg_sel_color: Color,

    size: Size,
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
                    if editor.is_dirty() {
                        if MessageDialog::new()
                            .set_level(rfd::MessageLevel::Warning)
                            .set_title("Are you sure?")
                            .set_description("Discard unsaved change?")
                            .set_buttons(rfd::MessageButtons::YesNo)
                            .show()
                        {
                            ctx.set_handled();
                            return;
                        }
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
                    if text.chars().count() == 1 && text.chars().nth(0).unwrap().is_ascii_control() {
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
                if editor.len_lines() as f64 * self.font_height >= ctx.size().height {
                    self.delta_y -= event.wheel_delta.y;
                    if self.delta_y > 0. {
                        self.delta_y = 0.;
                    }
                    if -self.delta_y > editor.len_lines() as f64 * self.font_height - 4. * self.font_height {
                        self.delta_y = -((editor.len_lines() as f64) * self.font_height - 4. * self.font_height)
                    }
                }
                self.delta_x -= event.wheel_delta.x;
                if self.delta_x > 0. {
                    self.delta_x = 0.;
                }
                if ctx.is_active() && event.buttons.contains(MouseButton::Left) {
                    let (x, y) = self.pix_to_point(event.window_pos.x, event.window_pos.y, ctx, editor);
                    let p = editor.point(x, y);
                    editor.move_main_caret_to(p, true);
                    self.put_caret_in_visible_range(ctx, editor);
                }
                ctx.request_paint();
                ctx.set_handled();
                return;
            }
            Event::MouseDown(event) => {
                if matches!(event.button, MouseButton::Left) {
                    let (x, y) = self.pix_to_point(event.window_pos.x, event.window_pos.y, ctx, editor);
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
                    let (x, y) = self.pix_to_point(event.window_pos.x, event.window_pos.y, ctx, editor);
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
                return;
            }
            Event::Command(cmd) if cmd.is(druid::commands::SAVE_FILE) => {
                if let Err(e) = self.save(editor) {
                    println!("Error writing file: {}", e);
                }

                ctx.set_handled();
                return;
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
            }
            Event::Command(cmd) if cmd.is(crate::commands::GIVE_FOCUS) => ctx.request_focus(),
            _ => (),
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _data: &EditStack, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                self.font_size = env.get(FONT_SIZE);
                //self.font_name = env.get(FONT_NAME).to_owned();
                self.bg_color = env.get(BG_COLOR);
                self.fg_color = env.get(FG_COLOR);
                self.fg_sel_color = env.get(FG_SEL_COLOR);
                self.bg_sel_color = env.get(BG_SEL_COLOR);

                ctx.register_for_focus();
            }
            _ => (),
        }
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &EditStack, _data: &EditStack, _env: &Env) {}

    fn layout(&mut self, layout_ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &EditStack, _env: &Env) -> Size {
        self.size = bc.max();

        let font = layout_ctx.text().font_family(&self.font_name).unwrap();

        let layout = layout_ctx
            .text()
            .new_text_layout("8")
            .font(font, self.font_size)
            .build()
            .unwrap();
        self.font_advance = layout.size().width;
        self.font_baseline = layout.line_metric(0).unwrap().baseline;
        self.font_height = layout.line_metric(0).unwrap().height;
        self.font_descent = self.font_height - self.font_baseline;

        self.page_len = (self.size.height / self.font_height).round() as usize;

        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EditStack, env: &Env) {
        let clip_rect = ctx.size().to_rect();
        ctx.clip(clip_rect);

        self.paint_editor(data, ctx, env);
    }
}

impl Default for EditorView {
    fn default() -> Self {
        EditorView {
            bg_color: Color::BLACK,
            fg_color: Color::WHITE,
            fg_sel_color: Color::BLACK,
            bg_sel_color: Color::WHITE,

            delta_x: 0.0,
            delta_y: 0.0,
            page_len: 0,
            font_advance: 0.0,

            font_baseline: 0.0,
            font_descent: 0.0,
            font_height: 0.0,
            font_size: 0.0,
            font_name: "consolas".to_owned(),
            size: Default::default(),
        }
    }
}

impl EditorView {
    fn visible_range(&self) -> Range<usize> {
        (-self.delta_y / self.font_height) as usize
            ..((-self.delta_y + self.size.height) / self.font_height) as usize + 1
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
        path.push(PathEl::LineTo(Point::new(e.point.x, self.font_height + y)));
        path.push(PathEl::LineTo(Point::new(s.point.x, self.font_height + y)));
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
        path.push(PathEl::MoveTo(Point::new(s.point.x, self.font_height + y)));
        path.push(PathEl::LineTo(Point::new(s.point.x, y)));
        path.push(PathEl::LineTo(Point::new(e.point.x + self.font_advance, y)));
        path.push(PathEl::LineTo(Point::new(
            e.point.x + self.font_advance,
            self.font_height + y,
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
                path.push(PathEl::LineTo(Point::new(e.point.x, self.font_height + y)));
                path.push(PathEl::LineTo(Point::new(0., self.font_height + y)));
                path.push(PathEl::LineTo(Point::new(0., y)));
                path.push(PathEl::ClosePath);
            }
            Some(SelectionLineRange::RangeFrom(_)) | Some(SelectionLineRange::RangeFull) => {
                path.push(PathEl::LineTo(Point::new(e.point.x, y)));
                path.push(PathEl::LineTo(Point::new(e.point.x, self.font_height + y)));
                path.push(PathEl::LineTo(Point::new(0., self.font_height + y)));
                path.push(PathEl::LineTo(Point::new(0., y)));
                path.push(PathEl::ClosePath);
            }
            None => {
                path.clear();
                path.push(PathEl::MoveTo(Point::new(0., y)));
                path.push(PathEl::LineTo(Point::new(e.point.x, y)));
                path.push(PathEl::LineTo(Point::new(e.point.x, self.font_height + y)));
                path.push(PathEl::LineTo(Point::new(0., self.font_height + y)));
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
                path.push(PathEl::LineTo(Point::new(e.point.x + self.font_advance, y)));
                path.push(PathEl::LineTo(Point::new(
                    e.point.x + self.font_advance,
                    self.font_height + y,
                )));
            }
            Some(SelectionLineRange::RangeFrom(_)) if path.last_x <= e.point.x => {
                // insert a point at the begining of the line
                path[0] = PathEl::LineTo(Point::new(path.last_x, y));
                path.insert(0, PathEl::MoveTo(Point::new(0., y)));
                path.push(PathEl::LineTo(Point::new(e.point.x + self.font_advance, y)));
                path.push(PathEl::LineTo(Point::new(
                    e.point.x + self.font_advance,
                    self.font_height + y,
                )));
            }
            None => {
                // the precedent line was outside the visible range
                path.clear();
                path.push(PathEl::MoveTo(Point::new(0., y)));
                path.push(PathEl::LineTo(Point::new(e.point.x + self.font_advance, y)));
                path.push(PathEl::LineTo(Point::new(
                    e.point.x + self.font_advance,
                    self.font_height + y,
                )));
            }
            _ => {
                path.push(PathEl::LineTo(Point::new(e.point.x + self.font_advance, y)));
                path.push(PathEl::LineTo(Point::new(
                    e.point.x + self.font_advance,
                    self.font_height + y,
                )));
            }
        }
    }

    fn paint_editor(&mut self, editor: &EditStack, ctx: &mut PaintCtx, _env: &Env) -> bool {
        let font = ctx
            .render_ctx
            .text()
            .font_family(&self.font_name)
            //.new_font_by_name(&self.font_name, self.font_size)
            //.build()
            .unwrap();

        let rect = Rect::new(0.0, 0.0, self.size.width, self.size.height);
        ctx.render_ctx.fill(rect, &self.bg_color);

        let visible_range = self.visible_range();

        // Draw line number
        let mut dy = (self.delta_y / self.font_height).fract() * self.font_height;

        let line_number_char_width = format!(" {}", editor.len_lines()).len();

        for line_idx in visible_range.clone() {
            if line_idx >= editor.len_lines() {
                break;
            }
            let layout = ctx
                .render_ctx
                .text()
                .new_text_layout(format!("{:1$}", line_idx, line_number_char_width))
                .font(font.clone(), self.font_size)
                .text_color(self.fg_color.clone())
                .build()
                .unwrap();
            ctx.render_ctx.draw_text(&layout, (0.0, /*self.font_baseline + */ dy));
            dy += self.font_height;
        }

        let mut clip_rect = ctx.size().to_rect();
        clip_rect.x0 += self.gutter_width(editor) - 2.0;
        ctx.render_ctx.clip(clip_rect);

        ctx.render_ctx
            .transform(Affine::translate((self.gutter_width(editor), 0.0)));
        ctx.render_ctx
            .stroke(Line::new((-2.0, 0.0), (-2.0, self.size.height)), &self.fg_color, 1.0);
        ctx.render_ctx.transform(Affine::translate((self.delta_x, 0.0)));

        let mut line = String::new();
        let mut indices = Vec::new();
        let mut ranges = Vec::new();
        let mut selection_path = Vec::new();
        let mut current_path = SelectionPath::new();

        // Draw selection first
        // TODO: cache layout to reuse it when we will draw the text
        let mut dy = (self.delta_y / self.font_height).fract() * self.font_height;
        for line_idx in visible_range.clone() {
            //editor.buffer.line(line_idx, &mut line);
            editor.displayable_line(position::Line::from(line_idx), &mut line, &mut indices, &mut Vec::new());
            let layout = ctx
                .render_ctx
                .text()
                .new_text_layout(line.clone()) // TODO: comment ne pas faire de clone?
                .font(font.clone(), self.font_size)
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

            dy += self.font_height;
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

        let mut dy = (self.delta_y / self.font_height).fract() * self.font_height;
        for line_idx in visible_range {
            //editor.buffer.line(line_idx, &mut line);
            editor.displayable_line(position::Line::from(line_idx), &mut line, &mut indices, &mut Vec::new());
            let layout = ctx
                .render_ctx
                .text()
                .new_text_layout(line.clone())
                .font(font.clone(), self.font_size)
                .text_color(self.fg_color.clone())
                .build()
                .unwrap();

            ctx.render_ctx.draw_text(&layout, (0.0, /*self.font_baseline + */ dy));

            editor.carets_on_line(position::Line::from(line_idx)).for_each(|c| {
                let metrics = layout.hit_test_text_position(indices[c.relative().index].index);
                ctx.render_ctx.stroke(
                    Line::new(
                        (metrics.point.x + 1.0, self.font_height + dy),
                        (metrics.point.x + 1.0, dy),
                    ),
                    &self.fg_color,
                    2.0,
                );
            });

            dy += self.font_height;
        }

        false
    }

    fn pix_to_point(&self, x: f64, y: f64, ctx: &mut EventCtx, editor: &EditStack) -> (usize, usize) {
        let x = ((x - self.delta_x) - self.gutter_width(editor)).max(0.);
        let y = (y - self.delta_y).max(0.);
        let line = ((y / self.font_height) as usize).min(editor.len_lines() - 1);

        let mut buf = String::new();
        let mut i = Vec::new();
        editor.displayable_line(line.into(), &mut buf, &mut Vec::new(), &mut i);

        let layout = self.text_layout(ctx, buf);
        let rel = i[layout.hit_test_point((x, 0.0).into()).idx].index;

        (rel, line)
    }

    fn text_layout(&self, ctx: &mut EventCtx, buf: String) -> druid::piet::D2DTextLayout {
        let font = ctx.text().font_family(&self.font_name).unwrap();
        let layout = ctx
            .text()
            .new_text_layout(buf)
            .font(font, self.font_size)
            .build()
            .unwrap();
        layout
    }

    fn gutter_width(&self, editor: &EditStack) -> f64 {
        let line_number_char_width = format!(" {}", editor.len_lines()).len();
        let line_number_width = self.font_advance * line_number_char_width as f64;
        line_number_width + self.font_advance + 4.0
    }
    fn editor_width(&self, editor: &EditStack) -> f64 {
        self.size.width - self.gutter_width(editor)
    }

    fn put_caret_in_visible_range(&mut self, ctx: &mut EventCtx, editor: &EditStack) {
        if editor.has_many_carets() {
            return;
        }
        let caret = editor.main_caret();
        let y = caret.line().index as f64 * self.font_height;

        if y > -self.delta_y + self.size.height - self.font_height {
            self.delta_y = -y + self.size.height - self.font_height;
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
        if x > -self.delta_x + self.editor_width(editor) - self.font_advance {
            self.delta_x = -x + self.editor_width(editor) - self.font_advance;
        }
        if x < -self.delta_x {
            self.delta_x = -x;
        }
    }

    fn save_as<P>(&mut self, editor: &mut EditStack, filename: P) -> anyhow::Result<()>
    where
        P: AsRef<Path>,
    {
        if filename.as_ref().exists() {
            if MessageDialog::new()
                .set_level(rfd::MessageLevel::Warning)
                .set_title("File Exists")
                .set_description("The given file allready exists, are you sure you want to overwrite it?")
                .set_buttons(rfd::MessageButtons::YesNo)
                .show()
            {
                return Ok(());
            }
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
