use std::io::Result;
use std::ops::{Deref, DerefMut, Range};
use std::path::{Path, PathBuf};

use druid::kurbo::{BezPath, Line, PathEl, Point, Rect, Size, Vec2};
use druid::piet::{FontBuilder, Piet, RenderContext, Text, TextLayout, TextLayoutBuilder};
use druid::{
    Affine, BoxConstraints, Color, Command, Env, Event, EventCtx, FileDialogOptions, FileInfo, HotKey, Key, KeyCode,
    KeyEvent, KeyModifiers, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Selector, SysMods, UpdateCtx, Widget,
    WindowHandle,
};

use crate::dialog;
use crate::position;
use crate::text_buffer::{EditStack, SelectionLineRange};
use position::Relative;

pub const FONT_SIZE: Key<f64> = Key::new("nonepad.editor.font_height");
pub const FONT_NAME: Key<&str> = Key::new("nonepad.editor.font_name");
pub const BG_COLOR: Key<Color> = Key::new("nondepad.editor.fg_color");
pub const FG_COLOR: Key<Color> = Key::new("nondepad.editor.bg_color");
pub const FG_SEL_COLOR: Key<Color> = Key::new("nondepad.editor.fg_selection_color");
pub const BG_SEL_COLOR: Key<Color> = Key::new("nondepad.editor.bg_selection_color");

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

impl Widget<EditStack> for EditorView {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, editor: &mut EditStack, _env: &Env) {
        match event {
            Event::WindowConnected => {
                ctx.request_focus();
            }
            Event::MouseDown(mouse) => {
                ctx.request_focus();
            }
            Event::KeyDown(event) => {
                if let Some(text) = event.text() {
                    if !(text.chars().count() == 1 && text.chars().nth(0).unwrap().is_ascii_control()) {
                        editor.insert(text);
                        ctx.request_paint();
                        ctx.set_handled();
                        return;
                    }
                }

                // if HotKey::new(SysMods::CmdShift, KeyCode::KeyP).matches(event) {
                //     handle.app_ctx().open_palette(vec![], |u| println!("Palette result {}", u));
                //     _ctx.request_paint();
                //     return true;
                // }

                if HotKey::new(SysMods::AltCmd, KeyCode::ArrowDown).matches(event) {
                    editor.duplicate_down();
                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }
                if HotKey::new(SysMods::AltCmd, KeyCode::ArrowUp).matches(event) {
                    editor.duplicate_up();
                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }
                match event {
                    KeyEvent {
                        key_code: KeyCode::ArrowRight,
                        mods,
                        ..
                    } => {
                        editor.forward(mods.shift);
                        ctx.request_paint();
                        ctx.set_handled();
                        return;
                    }
                    KeyEvent {
                        key_code: KeyCode::ArrowLeft,
                        mods,
                        ..
                    } => {
                        editor.backward(mods.shift);
                        ctx.request_paint();
                        ctx.set_handled();
                        return;
                    }
                    KeyEvent {
                        key_code: KeyCode::ArrowUp,
                        mods,
                        ..
                    } => {
                        editor.up(mods.shift);
                        self.put_carret_in_visible_range(editor);
                        ctx.request_paint();
                        ctx.set_handled();
                        return;
                    }
                    KeyEvent {
                        key_code: KeyCode::ArrowDown,
                        mods,
                        ..
                    } => {
                        editor.down(mods.shift);
                        self.put_carret_in_visible_range(editor);
                        ctx.request_paint();
                        ctx.set_handled();
                        return;
                    }
                    KeyEvent {
                        key_code: KeyCode::PageUp,
                        mods,
                        ..
                    } => {
                        for _ in 0..self.page_len {
                            editor.up(mods.shift);
                        }
                        self.put_carret_in_visible_range(editor);
                        ctx.request_paint();
                        ctx.set_handled();
                        return;
                    }
                    KeyEvent {
                        key_code: KeyCode::PageDown,
                        mods,
                        ..
                    } => {
                        for _ in 0..self.page_len {
                            editor.down(mods.shift)
                        }
                        self.put_carret_in_visible_range(editor);
                        ctx.request_paint();
                        ctx.set_handled();
                        return;
                    }
                    KeyEvent {
                        key_code: KeyCode::End,
                        mods,
                        ..
                    } => {
                        editor.end(mods.shift);
                        ctx.request_paint();
                        ctx.set_handled();
                        return;
                    }
                    KeyEvent {
                        key_code: KeyCode::Home,
                        mods,
                        ..
                    } => {
                        editor.home(mods.shift);
                        ctx.request_paint();
                        ctx.set_handled();
                        return;
                    }
                    KeyEvent {
                        key_code: KeyCode::Tab, ..
                    } => {
                        editor.tab();
                        ctx.request_paint();
                        ctx.set_handled();
                        return;
                    }
                    _ => (),
                }

                if HotKey::new(None, KeyCode::Escape).matches(event) {
                    editor.revert_to_single_carrets();
                    editor.cancel_selection();
                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }

                if HotKey::new(None, KeyCode::Backspace).matches(event) {
                    editor.backspace();
                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }
                if HotKey::new(None, KeyCode::Delete).matches(event) {
                    editor.delete();
                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }

                if HotKey::new(None, KeyCode::NumpadEnter).matches(event)
                    || HotKey::new(None, KeyCode::Return).matches(event)
                {
                    editor.insert(editor.file.linefeed.to_str());
                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }
                if HotKey::new(SysMods::Cmd, KeyCode::KeyZ).matches(event) {
                    editor.undo();
                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }
                if HotKey::new(SysMods::Cmd, KeyCode::KeyY).matches(event) {
                    editor.redo();
                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }
                if HotKey::new(SysMods::Cmd, KeyCode::KeyS).matches(event) {
                    //self.save(editor, ctx);
                    if editor.filename.is_some() {
                        ctx.submit_command(Command::new(druid::commands::SAVE_FILE, None), None);
                    } else {
                        let options = FileDialogOptions::new().show_hidden();
                        ctx.submit_command(Command::new(druid::commands::SHOW_SAVE_PANEL, options), None)
                    }
                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }
                if HotKey::new(SysMods::CmdShift, KeyCode::KeyS).matches(event) {
                    let options = FileDialogOptions::new().show_hidden();
                    ctx.submit_command(Command::new(druid::commands::SHOW_SAVE_PANEL, options), None);

                    ctx.request_paint();
                    ctx.set_handled();
                    return;
                }
            }
            Event::Wheel(event) => {
                self.delta_y -= event.wheel_delta.y;
                if self.delta_y > 0. {
                    self.delta_y = 0.
                }
                if -self.delta_y > editor.buffer.rope.len_lines() as f64 * self.font_height - 4. * self.font_height {
                    self.delta_y = -((editor.buffer.rope.len_lines() as f64) * self.font_height - 4. * self.font_height)
                }
                ctx.request_paint();
                ctx.set_handled();
                return;
            }
            Event::Command(cmd) if cmd.is(druid::commands::SAVE_FILE) => {
                if let Some(file_info) = cmd.get_unchecked(druid::commands::SAVE_FILE) {
                    if let Err(e) = self.save_as(editor, file_info.path()) {
                        println!("Error writing file: {}", e);
                    }
                } else {
                    if let Err(e) = self.save(editor) {
                        println!("Error writing file: {}", e);
                    }
                }
                ctx.set_handled();
                return;
            }
            Event::Command(cmd) if cmd.is(druid::commands::OPEN_FILE) => {
                
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _data: &EditStack, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                self.font_size = env.get(FONT_SIZE);
                self.font_name = env.get(FONT_NAME).to_owned();
                self.bg_color = env.get(BG_COLOR);
                self.fg_color = env.get(FG_COLOR);
                self.fg_sel_color = env.get(FG_SEL_COLOR);
                self.bg_sel_color = env.get(BG_SEL_COLOR);

                ctx.register_for_focus();
            }
            _ => (),
        }
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &EditStack, _data: &EditStack, _env: &Env) {
        println!("update");
    }

    fn layout(&mut self, layout_ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &EditStack, env: &Env) -> Size {
        self.size = bc.max();

        // self.font_height = env.get(FONT_HEIGHT);
        // self.font_name = env.get(FONT_NAME).to_owned();

        let font = layout_ctx
            .text()
            .new_font_by_name(&self.font_name, self.font_size)
            .build()
            .unwrap();

        let layout = layout_ctx.text().new_text_layout(&font, "8", None).build().unwrap();
        self.font_advance = layout.width();
        self.font_baseline = layout.line_metric(0).unwrap().baseline;
        self.font_height = layout.line_metric(0).unwrap().height;
        self.font_descent = self.font_height - self.font_baseline;

        self.page_len = (self.size.height / self.font_height).round() as usize;

        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &EditStack, env: &Env) {
        let clip_rect = ctx.size().to_rect();
        ctx.clip(clip_rect);

        self.paint_editor(data, ctx.render_ctx, env);
    }
}

pub struct EditorView {
    //editor: EditStack,
    delta_y: f64,
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

impl Default for EditorView {
    fn default() -> Self {
        EditorView {
            bg_color: Color::BLACK,
            fg_color: Color::WHITE,
            fg_sel_color: Color::BLACK,
            bg_sel_color: Color::WHITE,

            delta_y: 0.0,
            page_len: 0,
            font_advance: 0.0,

            font_baseline: 0.0,
            font_descent: 0.0,
            font_height: 0.0,
            font_size: 0.0,
            font_name: "".to_owned(),
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
        match (
            layout.hit_test_text_position(range.start.into()),
            layout.hit_test_text_position(range.end.into()),
        ) {
            (Some(s), Some(e)) => {
                path.clear();
                path.push(PathEl::MoveTo(Point::new(s.point.x, y)));
                path.push(PathEl::LineTo(Point::new(e.point.x, y)));
                path.push(PathEl::LineTo(Point::new(e.point.x, self.font_height + y)));
                path.push(PathEl::LineTo(Point::new(s.point.x, self.font_height + y)));
                path.push(PathEl::ClosePath);
            }
            _ => (),
        }
    }

    fn add_range_from_selection<L: TextLayout>(
        &mut self,
        y: f64,
        range: Range<position::Relative>,
        layout: &L,
        path: &mut Vec<PathEl>,
    ) -> f64 {
        match (
            layout.hit_test_text_position(range.start.into()),
            layout.hit_test_text_position(range.end.into()),
        ) {
            (Some(s), Some(e)) => {
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
            _ => 0.,
        }
    }

    fn add_range_to_selection<L: TextLayout>(
        &mut self,
        y: f64,
        range: Range<position::Relative>,
        layout: &L,
        path: &mut SelectionPath,
    ) {
        if let Some(e) = layout.hit_test_text_position(range.end.into()) {
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
    }

    fn add_range_full_selection<L: TextLayout>(
        &mut self,
        y: f64,
        range: Range<position::Relative>,
        layout: &L,
        path: &mut SelectionPath,
    ) {
        if let Some(e) = layout.hit_test_text_position(range.end.into()) {
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
    }

    fn paint_editor(&mut self, editor: &EditStack, piet: &mut Piet, env: &Env) -> bool {
        let font = piet
            .text()
            .new_font_by_name(&self.font_name, self.font_size)
            .build()
            .unwrap();

        let rect = Rect::new(0.0, 0.0, self.size.width, self.size.height);
        piet.fill(rect, &self.bg_color);

        let visible_range = self.visible_range();
        let mut dy = (self.delta_y / self.font_height).fract() * self.font_height;

        let line_number_char_width = format!(" {}", editor.len_lines()).len();
        let line_number_width = self.font_advance * line_number_char_width as f64; // piet.text().new_text_layout(&font, &format!("{} ",editor.len_lines()),None).build().unwrap().width();

        for line_idx in visible_range.clone() {
            let layout = piet
                .text()
                .new_text_layout(&font, &format!("{:1$}", line_idx, line_number_char_width), None)
                .build()
                .unwrap();
            piet.draw_text(&layout, (0.0, self.font_baseline + dy), &self.fg_color);
            dy += self.font_height;
        }
        piet.transform(Affine::translate((line_number_width + self.font_advance + 4.0, 0.0)));
        piet.stroke(Line::new((-2.0, 0.0), (-2.0, self.size.height)), &self.fg_color, 1.0);
        let mut line = String::new();
        let mut indices = Vec::new();
        let mut ranges = Vec::new();
        let mut selection_path = Vec::new();
        //let mut current_path: Vec<PathEl> = Vec::new();
        let mut current_path = SelectionPath::new();

        // Draw selection first
        // TODO: cache layout to reuse it when we will draw the text
        let mut dy = (self.delta_y / self.font_height).fract() * self.font_height;
        for line_idx in visible_range.clone() {
            //editor.buffer.line(line_idx, &mut line);
            editor.displayable_line(position::Line::from(line_idx), &mut line, &mut indices);
            let layout = piet.text().new_text_layout(&font, &line, None).build().unwrap();

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
                            indices[r.start]..Relative::from(line.len() - 1),
                            &layout,
                            &mut current_path,
                        )
                    }
                    SelectionLineRange::RangeTo(r) => {
                        self.add_range_to_selection(dy, Relative::from(0)..indices[r.end], &layout, &mut current_path)
                    }
                    SelectionLineRange::RangeFull => self.add_range_full_selection(
                        dy,
                        Relative::from(0)..Relative::from(line.len() - 1),
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
            let brush = piet.solid_brush(self.fg_sel_color.clone());
            piet.fill(&path, &brush);
            let brush = piet.solid_brush(self.bg_sel_color.clone());
            piet.stroke(&path, &brush, 0.5);
        }

        let mut dy = (self.delta_y / self.font_height).fract() * self.font_height;
        for line_idx in visible_range {
            //editor.buffer.line(line_idx, &mut line);
            editor.displayable_line(position::Line::from(line_idx), &mut line, &mut indices);
            let layout = piet.text().new_text_layout(&font, &line, None).build().unwrap();

            piet.draw_text(&layout, (0.0, self.font_baseline + dy), &self.fg_color);

            editor.carrets_on_line(position::Line::from(line_idx)).for_each(|c| {
                if let Some(metrics) = layout.hit_test_text_position(indices[c.relative().index].index) {
                    piet.stroke(
                        Line::new(
                            (metrics.point.x + 1.0, self.font_height + dy),
                            (metrics.point.x + 1.0, dy),
                        ),
                        &self.fg_color,
                        2.0,
                    );
                }
            });

            dy += self.font_height;
        }

        false
    }

    fn put_carret_in_visible_range(&mut self, editor: &EditStack) {
        if editor.buffer.carrets.len() > 1 {
            return;
        }
        if let Some(carret) = editor.buffer.carrets.first() {
            let y = carret.line().index as f64 * self.font_height;

            if y > -self.delta_y + self.size.height - self.font_height {
                self.delta_y = -y + self.size.height - self.font_height;
            }
            if y < -self.delta_y {
                self.delta_y = -y;
            }
        }
    }

    fn save_as<P>(&mut self, editor: &mut EditStack, filename: P) -> anyhow::Result<()> where P: AsRef<Path>  {
        if filename.as_ref().exists() {
            if let Some(result) = dialog::messagebox(
                "The given file allready exists, are you sure you want to overwrite it?",
                "Are you sure?",
                dialog::Icon::Question,
                dialog::Buttons::OkCancel,
            ) {
                if result != dialog::Button::Ok {
                    return Ok(());
                }
            }
        }

        editor.save(&filename)?;
        editor.filename = Some(filename.as_ref().to_path_buf());
        
        Ok(())
    }

    fn save(&mut self, editor: &mut EditStack) -> anyhow::Result<()> {
        anyhow::ensure!(editor.filename.is_some(), "editor.filename must not be None" );
        editor.save(editor.filename.clone().as_ref().unwrap())?;
        Ok(())
    }
}
