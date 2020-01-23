use std::io::Result;
use std::ops::{Deref, DerefMut, Range};
use std::path::Path;

use druid_shell::kurbo::{BezPath, Line, PathEl, Point, Rect, Size, Vec2};
use druid_shell::piet::{FontBuilder, Piet, RenderContext, Text, TextLayout, TextLayoutBuilder};
use druid_shell::{FileDialogOptions, HotKey, KeyCode, KeyEvent, KeyModifiers, SysMods, WinCtx};

use crate::app_context::AppContext;
use crate::dialog;
use crate::text_buffer::{EditStack, SelectionLineRange};
use crate::{BG_COLOR, BG_SEL_COLOR, FG_COLOR, FG_SEL_COLOR, FONT_HEIGHT};

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




#[derive(Debug, Default)]
pub struct EditorView {
    editor: EditStack,
    delta_y: f64,
    page_len: usize,
    font_advance: f64,
    font_ascent: f64,
    font_descent: f64,
    font_height: f64,
    size: Size,
}

impl EditorView {
    pub fn from_file<'a, P: AsRef<Path>>(path: P) -> Result<Self> {
        let editor = EditStack::from_file(path)?;
        Ok(Self {
            editor,
            ..Default::default()
        })
    }

    fn visible_range(&self) -> Range<usize> {
        (-self.delta_y / self.font_height) as usize
            ..((-self.delta_y + self.size.height) / self.font_height) as usize + 1
    }

    fn add_bounded_range_selection(
        &mut self,
        y: f64,
        range: Range<usize>,
        layout: &dyn TextLayout,
        path: &mut SelectionPath,
    ) {
        match (
            layout.hit_test_text_position(range.start),
            layout.hit_test_text_position(range.end),
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

    fn add_range_from_selection(
        &mut self,
        y: f64,
        range: Range<usize>,
        layout: &dyn TextLayout,
        path: &mut Vec<PathEl>,
    ) -> f64 {
        match (
            layout.hit_test_text_position(range.start),
            layout.hit_test_text_position(range.end),
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

    fn add_range_to_selection(
        &mut self,
        y: f64,
        range: Range<usize>,
        layout: &dyn TextLayout,
        path: &mut SelectionPath,
    ) {
        if let Some(e) = layout.hit_test_text_position(range.end) {
            match &path.last_range {
                Some(SelectionLineRange::RangeFrom(_)) if range.end == 0 => {
                    path.push(PathEl::ClosePath);
                }
                Some(SelectionLineRange::RangeFull) if range.end == 0 => {
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

    fn add_range_full_selection(
        &mut self,
        y: f64,
        range: Range<usize>,
        layout: &dyn TextLayout,
        path: &mut SelectionPath,
    ) {
        if let Some(e) = layout.hit_test_text_position(range.end) {
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

    

    pub fn paint(&mut self, piet: &mut Piet, _ctx: &mut dyn WinCtx) -> bool {
        let font = piet.text().new_font_by_name("Consolas", FONT_HEIGHT).build().unwrap();

        let rect = Rect::new(0.0, 0.0, self.size.width, self.size.height);
        piet.fill(rect, &BG_COLOR);

        let visible_range = self.visible_range();
        let mut dy = (self.delta_y / self.font_height).fract() * self.font_height;

        let mut line = String::new();
        let mut indices = Vec::new();
        let mut ranges = Vec::new();
        let mut selection_path = Vec::new();
        //let mut current_path: Vec<PathEl> = Vec::new();
        let mut current_path = SelectionPath::new();

        // Draw selection first
        // TODO: cache layout to reuse it when we will draw the text
        for line_idx in visible_range.clone() {
            //self.editor.buffer.line(line_idx, &mut line);
            self.editor.buffer.displayable_line(line_idx, self.editor.file.indentation.visible_len(), &mut line, &mut indices);
            let layout = piet.text().new_text_layout(&font, &line).build().unwrap();

            self.editor.buffer.selection_on_line(line_idx, &mut ranges);

            for range in &ranges {
                match range {
                    SelectionLineRange::Range(r) => {
                        // Simple case, the selection is contain on one line
                        self.add_bounded_range_selection(dy, indices[r.start]..indices[r.end], &layout, &mut current_path)
                    }
                    SelectionLineRange::RangeFrom(r) => {
                        current_path.last_x =
                            self.add_range_from_selection(dy, indices[r.start]..line.len() - 1, &layout, &mut current_path)
                    }
                    SelectionLineRange::RangeTo(r) => {
                        self.add_range_to_selection(dy, 0..indices[r.end], &layout, &mut current_path)
                    }
                    SelectionLineRange::RangeFull => {
                        self.add_range_full_selection(dy, 0..line.len() - 1, &layout, &mut current_path)
                    }
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
            let brush = piet.solid_brush(FG_SEL_COLOR);
            piet.fill(&path, &brush);
            let brush = piet.solid_brush(BG_SEL_COLOR);
            piet.stroke(&path, &brush, 0.5);
        }

        let mut dy = (self.delta_y / self.font_height).fract() * self.font_height;
        for line_idx in visible_range {
            
            //self.editor.buffer.line(line_idx, &mut line);
            self.editor.buffer.displayable_line(line_idx, self.editor.file.indentation.visible_len(), &mut line, &mut indices);
            let layout = piet.text().new_text_layout(&font, &line).build().unwrap();

            piet.draw_text(&layout, (0.0, self.font_ascent + dy), &FG_COLOR);

            self.editor.buffer.carrets_on_line(line_idx).for_each(|c| {
                if let Some(metrics) = layout.hit_test_text_position(indices[c.index_column()]) {
                    piet.stroke(
                        Line::new(
                            (metrics.point.x + 1.0, self.font_height + dy),
                            (metrics.point.x + 1.0, dy),
                        ),
                        &FG_COLOR,
                        2.0,
                    );
                }
            });

            dy += self.font_height;
        }

        false
    }

    fn put_carret_in_visible_range(&mut self) {
        if self.editor.buffer.carrets.len() > 1 {
            return;
        }
        if let Some(carret) = self.editor.buffer.carrets.first() {
            let y = self.editor.buffer.byte_to_line(carret.index) as f64 * self.font_height;

            if y > -self.delta_y + self.size.height - self.font_height {
                self.delta_y = -y + self.size.height - self.font_height;
            }
            if y < -self.delta_y {
                self.delta_y = -y;
            }
        }
    }

    pub fn key_down(&mut self, event: KeyEvent, ctx: &mut dyn WinCtx, app_ctx: &mut AppContext) -> bool {
        if let Some(text) = event.text() {
            if !(text.chars().count() == 1 && text.chars().nth(0).unwrap().is_ascii_control()) {
                self.editor.insert(text);
                ctx.invalidate();
                return true;
            }
        }

        if HotKey::new(SysMods::CmdShift, KeyCode::KeyP).matches(event) {
            app_ctx.open_palette(vec![], |u| println!("Palette result {}", u));
            ctx.invalidate();
            return true;
        }

        if HotKey::new(SysMods::AltCmd, KeyCode::ArrowDown).matches(event) {
            self.editor.duplicate_down();
            ctx.invalidate();
            return true;
        }

        match event {
            KeyEvent {
                key_code: KeyCode::Tab,
                mods,
                ..} => {
                    
                }
            KeyEvent {
                key_code: KeyCode::ArrowRight,
                mods,
                ..
            } => {
                self.editor.forward(mods.shift);
                ctx.invalidate();
                return true;
            }
            KeyEvent {
                key_code: KeyCode::ArrowLeft,
                mods,
                ..
            } => {
                self.editor.backward(mods.shift);
                ctx.invalidate();
                return true;
            }
            KeyEvent {
                key_code: KeyCode::ArrowUp,
                mods,
                ..
            } => {
                self.editor.up(mods.shift);
                self.put_carret_in_visible_range();
                ctx.invalidate();
                return true;
            }
            KeyEvent {
                key_code: KeyCode::ArrowDown,
                mods,
                ..
            } => {
                self.editor.down(mods.shift);
                self.put_carret_in_visible_range();
                ctx.invalidate();
                return true;
            }
            KeyEvent {
                key_code: KeyCode::PageUp,
                mods,
                ..
            } => {
                for _ in 0..self.page_len {
                    self.editor.up(mods.shift);
                }
                self.put_carret_in_visible_range();
                ctx.invalidate();
                return true;
            }
            KeyEvent {
                key_code: KeyCode::PageDown,
                mods,
                ..
            } => {
                for _ in 0..self.page_len {
                    self.editor.down(mods.shift)
                }
                self.put_carret_in_visible_range();
                ctx.invalidate();
                return true;
            }
            KeyEvent {
                key_code: KeyCode::End,
                mods,
                ..
            } => {
                self.editor.end(mods.shift);
                ctx.invalidate();
                return true;
            }
            KeyEvent {
                key_code: KeyCode::Home,
                mods,
                ..
            } => {
                self.editor.home(mods.shift);
                ctx.invalidate();
                return true;
            }

            _ => (),
        }

        if HotKey::new(None, KeyCode::Escape).matches(event) {
            self.editor.revert_to_single_carrets();
            ctx.invalidate();
            return true;
        }

        if HotKey::new(None, KeyCode::Backspace).matches(event) {
            self.editor.backspace();
            ctx.invalidate();
            return true;
        }
        if HotKey::new(None, KeyCode::Delete).matches(event) {
            self.editor.delete();
            ctx.invalidate();
            return true;
        }

        if HotKey::new(None, KeyCode::NumpadEnter).matches(event) || HotKey::new(None, KeyCode::Return).matches(event) {
            self.editor.insert(self.editor.file.linefeed.to_str());
            ctx.invalidate();
            return true;
        }
        if HotKey::new(SysMods::Cmd, KeyCode::KeyZ).matches(event) {
            self.editor.undo();
            ctx.invalidate();
            return true;
        }
        if HotKey::new(SysMods::Cmd, KeyCode::KeyY).matches(event) {
            self.editor.redo();
            ctx.invalidate();
            return true;
        }
        if HotKey::new(SysMods::Cmd, KeyCode::KeyS).matches(event) {
            self.editor.save().unwrap();
            ctx.invalidate();
            return true;
        }
        if HotKey::new(SysMods::CmdShift, KeyCode::KeyS).matches(event) {
            let options = FileDialogOptions::new().show_hidden();
            let filename = ctx.save_as_sync(options);
            if let Some(filename) = filename {
                // TODO: test if file don't already exist!
                if filename.path().exists() {
                    if let Some(result) = dialog::messagebox(
                        "The given file allready exists, are you sure you want to overwrite it?",
                        "Are you sure?",
                        dialog::Icon::Question,
                        dialog::Buttons::OkCancel,
                    ) {
                        if result != dialog::Button::Ok {
                            return true;
                        }
                    }
                }
                self.editor.save_as(filename.path()).unwrap();
            }
            ctx.invalidate();
            return true;
        }

        return false;
    }

    pub fn wheel(&mut self, delta: Vec2, _mods: KeyModifiers, ctx: &mut dyn WinCtx) {
        self.delta_y -= delta.y;
        if self.delta_y > 0. {
            self.delta_y = 0.
        }
        if -self.delta_y > self.editor.buffer.rope.len_lines() as f64 * self.font_height - 4. * self.font_height {
            self.delta_y = -((self.editor.buffer.rope.len_lines() as f64) * self.font_height - 4. * self.font_height)
        }
        ctx.invalidate();
    }

    pub fn size(&mut self, width: u32, height: u32, dpi: f32, ctx: &mut dyn WinCtx) {
        let dpi_scale = dpi as f64 / 96.0;
        let width_f = (width as f64) / dpi_scale;
        let height_f = (height as f64) / dpi_scale;
        self.size = Size::new(width_f, height_f);

        let font = ctx
            .text_factory()
            .new_font_by_name("Consolas", FONT_HEIGHT)
            .build()
            .unwrap();
        self.font_advance = ctx.text_factory().new_text_layout(&font, " ").build().unwrap().width();
        // Calculated with font_kit
        self.font_descent = -3.2626953;
        self.font_ascent = 11.958984;
        self.font_height = 15.22168;

        self.page_len = (height_f / self.font_height).round() as usize;
    }
}
