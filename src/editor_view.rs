use std::io::Result;
use std::ops::Range;
use std::path::Path;

use druid_shell::kurbo::{BezPath, Line, PathEl, Point, Rect, Size, Vec2};
use druid_shell::piet::{FontBuilder, Piet, RenderContext, Text, TextLayout, TextLayoutBuilder};
use druid_shell::{FileDialogOptions, HotKey, KeyCode, KeyEvent, KeyModifiers, SysMods, WinCtx};

use crate::app_context::AppContext;
use crate::dialog;
use crate::text_buffer::{EditStack, OpenRange};
use crate::{BG_COLOR, BG_SEL_COLOR, FG_COLOR, FG_SEL_COLOR, FONT_HEIGHT};

#[derive(Debug, Default)]
pub struct EditorView {
    editor: EditStack,
    delta_y: f64,
    page_len: usize,
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
        (-self.delta_y / FONT_HEIGHT) as usize
            ..((-self.delta_y + self.size.height) / FONT_HEIGHT) as usize
    }

    pub fn paint(&mut self, piet: &mut Piet, _ctx: &mut dyn WinCtx) -> bool {
        let font = piet
            .text()
            .new_font_by_name("Consolas", FONT_HEIGHT)
            .build()
            .unwrap();

        let advance = piet
            .text()
            .new_text_layout(&font, " ")
            .build()
            .unwrap()
            .width();

        let rect = Rect::new(0.0, 0.0, self.size.width, self.size.height);
        piet.fill(rect, &BG_COLOR);
        // piet.stroke(Line::new((10.0, 50.0), (90.0, 90.0)), &FG_COLOR, 1.0);
        let visible_range = self.visible_range();
        let mut dy = (self.delta_y / FONT_HEIGHT).fract() * FONT_HEIGHT;
        //for line in self.text.lines().skip(r.start).take(r.end - r.start) {
        let mut line = String::new();
        let mut ranges = Vec::new();
        let mut selection_path = Vec::new();

        for line_idx in dbg!(visible_range.clone()) {
            self.editor.buffer.line(line_idx, &mut line);
            let layout = piet.text().new_text_layout(&font, &line).build().unwrap();

            self.editor.buffer.selection_on_line(line_idx, &mut ranges);

            for r in &ranges {
                dbg!(&line_idx, &r);
                match r {
                    OpenRange::Range(r) => {
                        // Simple case, the selection is contain on one line

                        let s = layout.hit_test_text_position(r.start);
                        let e = layout.hit_test_text_position(r.end);

                        match (s, e) {
                            (Some(s), Some(e)) => {
                                selection_path.push(Vec::new());
                                let p = selection_path.last_mut().unwrap();
                                p.push(PathEl::MoveTo(Point::new(s.point.x, dy + 2.2)));
                                p.push(PathEl::LineTo(Point::new(e.point.x, dy + 2.2)));
                                p.push(PathEl::LineTo(Point::new(
                                    e.point.x,
                                    FONT_HEIGHT + dy + 2.2,
                                )));
                                p.push(PathEl::LineTo(Point::new(
                                    s.point.x,
                                    FONT_HEIGHT + dy + 2.2,
                                )));
                                p.push(PathEl::ClosePath);
                            }
                            _ => (),
                        }
                    }
                    OpenRange::RangeFrom(r) => {
                        let s = layout.hit_test_text_position(r.start);
                        let e = layout.hit_test_text_position(line.len() - 1);

                        match (s, e) {
                            (Some(s), Some(e)) => {
                                selection_path.push(Vec::new());
                                let p = selection_path.last_mut().unwrap();
                                p.push(PathEl::MoveTo(Point::new(
                                    s.point.x,
                                    FONT_HEIGHT + dy + 2.2,
                                )));
                                p.push(PathEl::LineTo(Point::new(s.point.x, dy + 2.2)));
                                p.push(PathEl::LineTo(Point::new(e.point.x + advance, dy + 2.2)));
                                p.push(PathEl::LineTo(Point::new(
                                    e.point.x + advance,
                                    FONT_HEIGHT + dy + 2.2,
                                )));
                            }
                            _ => (),
                        }
                    }
                    OpenRange::RangeTo(r) => {
                        if let Some(e) = layout.hit_test_text_position(r.end) {
                            if selection_path.len() > 0 {
                                let p = selection_path.last_mut().unwrap();

                                // todo: finish
                                // match (p[0],p.last().clone()) {
                                //     (PathEl::MoveTo(Point{x,y:_}),Some(PathEl::LineTo(Point{x:_,y}))) if x>e.point.x => {
                                //         p.push(PathEl::LineTo(Point::new(x,*y)));
                                //         p.push(PathEl::ClosePath);
                                //     }
                                //     _ => ()
                                // }

                                p.push(PathEl::LineTo(Point::new(e.point.x, dy + 2.2)));
                                p.push(PathEl::LineTo(Point::new(
                                    e.point.x,
                                    FONT_HEIGHT + dy + 2.2,
                                )));
                                p.push(PathEl::LineTo(Point::new(0., FONT_HEIGHT + dy + 2.2)));
                                p.push(PathEl::LineTo(Point::new(0., dy + 2.2)));
                                p.push(PathEl::ClosePath);
                            } else {
                                selection_path.push(Vec::new());
                                let p = selection_path.last_mut().unwrap();
                                p.push(PathEl::MoveTo(Point::new(0., dy + 2.2)));
                                p.push(PathEl::LineTo(Point::new(e.point.x, dy + 2.2)));
                                p.push(PathEl::LineTo(Point::new(
                                    e.point.x,
                                    FONT_HEIGHT + dy + 2.2,
                                )));
                                p.push(PathEl::LineTo(Point::new(0., FONT_HEIGHT + dy + 2.2)));
                                p.push(PathEl::ClosePath);
                            }
                        }
                    }
                    OpenRange::RangeFull => {
                        if let Some(e) = layout.hit_test_text_position(line.len() - 1) {
                            if selection_path.len() > 0 {
                                let p = selection_path.last_mut().unwrap();
                                if let PathEl::MoveTo(point) = p[0] {
                                    if point.x > 0.1 {
                                        p[0] = PathEl::LineTo(point);
                                        p.insert(0, PathEl::MoveTo(Point::new(0., point.y)));
                                    }
                                }
                                p.push(PathEl::LineTo(Point::new(e.point.x + advance, dy + 2.2)));
                                p.push(PathEl::LineTo(Point::new(
                                    e.point.x + advance,
                                    FONT_HEIGHT + dy + 2.2,
                                )));
                            } else {
                                selection_path.push(Vec::new());
                                let p = selection_path.last_mut().unwrap();
                                p.push(PathEl::MoveTo(Point::new(0., dy + 2.2)));
                                p.push(PathEl::LineTo(Point::new(e.point.x + advance, dy + 2.2)));
                                p.push(PathEl::LineTo(Point::new(
                                    e.point.x + advance,
                                    FONT_HEIGHT + dy + 2.2,
                                )));
                            }
                        }
                    }
                    _ => (),
                }
            }
            dy += FONT_HEIGHT;
        }

        for path in selection_path {
            let path = BezPath::from_vec(path);
            let brush = piet.solid_brush(FG_SEL_COLOR);
            piet.fill(&path, &brush);
            let brush = piet.solid_brush(BG_SEL_COLOR);
            piet.stroke(&path, &brush, 0.5);
        }

        let mut dy = (self.delta_y / FONT_HEIGHT).fract() * FONT_HEIGHT;
        for line_idx in visible_range {
            self.editor.buffer.line(line_idx, &mut line);
            let layout = piet.text().new_text_layout(&font, &line).build().unwrap();

            piet.draw_text(&layout, (0.0, FONT_HEIGHT + dy), &FG_COLOR);

            self.editor.buffer.carrets_on_line(line_idx).for_each(|c| {
                println!("carret {:?} on line {}", c, line_idx);
                // println!("{:?}", layout.hit_test_text_position(c.col_index));
                if let Some(metrics) = layout.hit_test_text_position(c.col_index) {
                    piet.stroke(
                        Line::new(
                            (metrics.point.x, FONT_HEIGHT + dy + 2.2),
                            (metrics.point.x, dy + 2.2),
                        ),
                        &FG_COLOR,
                        2.0,
                    );
                }
            });

            dy += FONT_HEIGHT;
        }

        false
    }

    pub fn key_down(
        &mut self,
        event: KeyEvent,
        ctx: &mut dyn WinCtx,
        app_ctx: &mut AppContext,
    ) -> bool {
        if let Some(text) = event.text() {
            if !(text.chars().count() == 1 && text.chars().nth(0).unwrap().is_ascii_control()) {
                self.editor.insert(text);
                println!("keydown: {:?}", text);
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
                ctx.invalidate();
                return true;
            }
            KeyEvent {
                key_code: KeyCode::ArrowDown,
                mods,
                ..
            } => {
                self.editor.down(mods.shift);
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

        if HotKey::new(None, KeyCode::NumpadEnter).matches(event)
            || HotKey::new(None, KeyCode::Return).matches(event)
        {
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

        //println!("keydown: {:?}, timer id = {:?}", event, id);
        return false;
    }

    pub fn wheel(&mut self, delta: Vec2, _mods: KeyModifiers, ctx: &mut dyn WinCtx) {
        self.delta_y -= delta.y;
        if self.delta_y > 0. {
            self.delta_y = 0.
        }
        if -self.delta_y
            > self.editor.buffer.rope.len_lines() as f64 * FONT_HEIGHT - 4. * FONT_HEIGHT
        {
            self.delta_y =
                -((self.editor.buffer.rope.len_lines() as f64) * FONT_HEIGHT - 4. * FONT_HEIGHT)
        }
        ctx.invalidate();
    }

    pub fn size(&mut self, width: u32, height: u32, dpi: f32, _ctx: &mut dyn WinCtx) {
        let dpi_scale = dpi as f64 / 96.0;
        let width_f = (width as f64) / dpi_scale;
        let height_f = (height as f64) / dpi_scale;
        self.size = Size::new(width_f, height_f);

        self.page_len = (height_f / FONT_HEIGHT).round() as usize;
    }
}
