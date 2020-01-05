// "Hello 😊︎ 😐︎ ☹︎ example"
mod file;
mod text_buffer;

use std::any::Any;
use std::io::{Read, Result};
use std::ops::Range;
use std::path::Path;

use druid_shell::kurbo::{Line, Rect, Vec2};
use druid_shell::piet::{Color, FontBuilder, RenderContext, Text, TextLayout, TextLayoutBuilder};

use druid_shell::piet::Piet;

use druid_shell::{
    Application, Cursor, FileDialogOptions, FileSpec, HotKey, KeyCode, KeyEvent, KeyModifiers,
    Menu, MouseEvent, RunLoop, SysMods, TimerToken, WinCtx, WinHandler, WindowBuilder,
    WindowHandle,
};

use crate::file::{LineFeed, TextFileInfo};
use crate::text_buffer::EditStack;

const BG_COLOR: Color = Color::rgb8(0x2f, 0x4f, 0x4f);
const FG_COLOR: Color = Color::rgb8(0xdb, 0xd0, 0xa7);

const FONT_HEIGHT: f64 = 13.0;

#[derive(Default)]
struct HelloState {
    size: (f64, f64),
    handle: WindowHandle,
    need_recalculate_font_size: bool,
    font_advance: f64,
    font_height: f64,
    editor: EditStack,
    delta_y: f64,
    page_len: usize,
}

impl HelloState {
    fn new() -> Self {
        Self {
            need_recalculate_font_size: true,
            ..Default::default()
        }
    }
    fn from_file<'a, P: AsRef<Path>>(path: P) -> Result<Self> {
        let editor = EditStack::from_file(path)?;
        Ok(Self {
            need_recalculate_font_size: true,
            editor,
            ..Default::default()
        })
    }

    fn visible_range(&self) -> Range<usize> {
        (-self.delta_y / self.font_height) as usize
            ..((-self.delta_y + self.size.1) / self.font_height) as usize
    }
}

impl WinHandler for HelloState {
    fn connect(&mut self, handle: &WindowHandle) {
        self.handle = handle.clone();
    }

    fn paint(&mut self, piet: &mut Piet, _ctx: &mut dyn WinCtx) -> bool {
        let (width, height) = self.size;
        let font = piet
            .text()
            .new_font_by_name("Consolas", FONT_HEIGHT)
            .build()
            .unwrap();

        if self.need_recalculate_font_size {
            self.font_height = FONT_HEIGHT;
            let layout = piet.text().new_text_layout(&font, "O").build().unwrap();
            self.font_advance = layout.width();
            println!("{} {}", self.font_height, self.font_advance);
            self.need_recalculate_font_size = false;
        }
        let rect = Rect::new(0.0, 0.0, width, height);
        piet.fill(rect, &BG_COLOR);
        // piet.stroke(Line::new((10.0, 50.0), (90.0, 90.0)), &FG_COLOR, 1.0);
        let r = self.visible_range();
        let mut dy = (self.delta_y / self.font_height).fract();
        //for line in self.text.lines().skip(r.start).take(r.end - r.start) {
        let mut line = String::new();
        for line_idx in r {
            self.editor.buffer.line(line_idx, &mut line);
            let layout = piet.text().new_text_layout(&font, &line).build().unwrap();

            piet.draw_text(&layout, (0.0, self.font_height + dy), &FG_COLOR);

            self.editor.buffer.carrets_on_line(line_idx).for_each(|c| {
                println!("carret {:?} on line {}", c, line_idx);
                println!("{:?}", layout.hit_test_text_position(c.col_index));
                if let Some(metrics) = layout.hit_test_text_position(c.col_index) {
                    piet.stroke(
                        Line::new(
                            (metrics.point.x, self.font_height + dy + 2.2),
                            (metrics.point.x, dy + 2.2),
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

    fn command(&mut self, id: u32, ctx: &mut dyn WinCtx) {
        match id {
            0x100 => {
                self.handle.close();
                Application::quit();
            }
            0x101 => {
                let options = FileDialogOptions::new().show_hidden().allowed_types(vec![
                    FileSpec::new("Rust Files", &["rs", "toml"]),
                    FileSpec::TEXT,
                    FileSpec::JPG,
                ]);
                let filename = ctx.open_file_sync(options);
                println!("result: {:?}", filename);
            }
            _ => println!("unexpected id {}", id),
        }
    }

    fn key_down(&mut self, event: KeyEvent, ctx: &mut dyn WinCtx) -> bool {
        if let Some(text) = event.text() {
            if !(text.chars().count() == 1 && text.chars().nth(0).unwrap().is_ascii_control()) {
                self.editor.insert(text);
                println!("keydown: {:?}", text);
                ctx.invalidate();
                return true;
            }
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
            for _  in 0..self.page_len {self.editor.up(false);}
            ctx.invalidate();
            return true;
        }
        if HotKey::new(None, KeyCode::PageDown).matches(event) {
            for _  in 0..self.page_len {self.editor.down(false)};
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
                self.editor.save_as(filename.path()).unwrap();
            }
            ctx.invalidate();
            return true;
        }

        //println!("keydown: {:?}, timer id = {:?}", event, id);
        false
    }

    fn wheel(&mut self, delta: Vec2, mods: KeyModifiers, ctx: &mut dyn WinCtx) {
        //println!("mouse_wheel {:?} {:?}", delta, mods);
        self.delta_y -= delta.y;
        ctx.invalidate();
    }

    fn mouse_move(&mut self, event: &MouseEvent, ctx: &mut dyn WinCtx) {
        ctx.set_cursor(&Cursor::Arrow);
        //println!("mouse_move {:?}", event);
    }

    fn mouse_down(&mut self, event: &MouseEvent, _ctx: &mut dyn WinCtx) {
        println!("mouse_down {:?}", event);
    }

    fn mouse_up(&mut self, event: &MouseEvent, _ctx: &mut dyn WinCtx) {
        println!("mouse_up {:?}", event);
    }

    fn timer(&mut self, id: TimerToken, _ctx: &mut dyn WinCtx) {
        println!("timer fired: {:?}", id);
    }

    fn size(&mut self, width: u32, height: u32, _ctx: &mut dyn WinCtx) {
        let dpi = self.handle.get_dpi();
        let dpi_scale = dpi as f64 / 96.0;
        let width_f = (width as f64) / dpi_scale;
        let height_f = (height as f64) / dpi_scale;
        self.size = (width_f, height_f);

        self.page_len = (height_f/FONT_HEIGHT).round() as usize;
    }

    fn destroy(&mut self, _ctx: &mut dyn WinCtx) {
        Application::quit()
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

fn main() {
    Application::init();

    let mut file_menu = Menu::new();
    file_menu.add_item(
        0x100,
        "E&xit",
        Some(&HotKey::new(SysMods::Cmd, "q")),
        true,
        false,
    );
    file_menu.add_item(
        0x101,
        "O&pen",
        Some(&HotKey::new(SysMods::Cmd, KeyCode::KeyO)),
        true,
        false,
    );
    let mut menubar = Menu::new();

    //menubar.add_dropdown(Menu::new(), "Application", true);

    menubar.add_dropdown(file_menu, "&File", true);

    let mut run_loop = RunLoop::new();
    let mut builder = WindowBuilder::new();
    let state = if let Some(filename) = std::env::args().nth(1) {
        HelloState::from_file(filename).unwrap()
    } else {
        HelloState::new()
    };

    builder.set_handler(Box::new(state));

    builder.set_title("NonePad");
    builder.set_menu(menubar);
    let window = builder.build().unwrap();

    window.show();
    run_loop.run();
}
