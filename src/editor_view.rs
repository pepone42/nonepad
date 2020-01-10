use std::io::Result;
use std::ops::Range;
use std::path::Path;

use druid_shell::kurbo::{Line, Rect, Size, Vec2};
use druid_shell::piet::{
    FontBuilder, Piet, RenderContext, Text, TextLayout, TextLayoutBuilder,
};
use druid_shell::{
    FileDialogOptions, HotKey, KeyCode, KeyEvent, KeyModifiers,
    SysMods, WinCtx};

use crate::app_context::AppContext;
use crate::text_buffer::EditStack;
use crate::dialog;
use crate::{BG_COLOR, FG_COLOR, FONT_HEIGHT};

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
            ..((-self.delta_y + self.size.width) / FONT_HEIGHT) as usize
    }

    pub fn paint(&mut self, piet: &mut Piet, _ctx: &mut dyn WinCtx) -> bool {
        let font = piet
            .text()
            .new_font_by_name("Consolas", FONT_HEIGHT)
            .build()
            .unwrap();

        let rect = Rect::new(0.0, 0.0, self.size.width, self.size.height);
        piet.fill(rect, &BG_COLOR);
        // piet.stroke(Line::new((10.0, 50.0), (90.0, 90.0)), &FG_COLOR, 1.0);
        let r = self.visible_range();
        let mut dy = (self.delta_y / FONT_HEIGHT).fract();
        //for line in self.text.lines().skip(r.start).take(r.end - r.start) {
        let mut line = String::new();
        for line_idx in r {
            self.editor.buffer.line(line_idx, &mut line);
            let layout = piet.text().new_text_layout(&font, &line).build().unwrap();

            piet.draw_text(&layout, (0.0, FONT_HEIGHT + dy), &FG_COLOR);

            self.editor.buffer.carrets_on_line(line_idx).for_each(|c| {
                println!("carret {:?} on line {}", c, line_idx);
                println!("{:?}", layout.hit_test_text_position(c.col_index));
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

        if HotKey::new(None, KeyCode::ArrowRight).matches(event) {
            self.editor.forward(false);
            ctx.invalidate();
            return true;
        }
        if HotKey::new(None, KeyCode::ArrowLeft).matches(event) {
            self.editor.backward(false);
            ctx.invalidate();
            return true;
        }
        if HotKey::new(None, KeyCode::ArrowUp).matches(event) {
            self.editor.up(false);
            ctx.invalidate();
            return true;
        }
        if HotKey::new(None, KeyCode::ArrowDown).matches(event) {
            self.editor.down(false);
            ctx.invalidate();
            return true;
        }
        if HotKey::new(None, KeyCode::PageUp).matches(event) {
            for _ in 0..self.page_len {
                self.editor.up(false);
            }
            ctx.invalidate();
            return true;
        }
        if HotKey::new(None, KeyCode::PageDown).matches(event) {
            for _ in 0..self.page_len {
                self.editor.down(false)
            }
            ctx.invalidate();
            return true;
        }
        if HotKey::new(None, KeyCode::End).matches(event) {
            self.editor.end(false);
            ctx.invalidate();
            return true;
        }
        if HotKey::new(None, KeyCode::Home).matches(event) {
            self.editor.home(false);
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
