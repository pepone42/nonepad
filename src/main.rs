// "Hello 😊︎ 😐︎ ☹︎ example"
mod file;
mod text_buffer;

use std::any::Any;
use std::fs;
use std::io::{Read, Result};
use std::ops::Range;

use druid_shell::kurbo::{Line, Rect, Vec2};
use druid_shell::piet::{Color, FontBuilder, RenderContext, Text, TextLayout, TextLayoutBuilder};

use druid_shell::piet::Piet;

use druid_shell::{
    Application, Cursor, FileDialogOptions, FileSpec, HotKey, KeyEvent, KeyModifiers, KeyCode, Menu,
    MouseEvent, RunLoop, SysMods, TimerToken, WinCtx, WinHandler, WindowBuilder, WindowHandle,
};

use crate::text_buffer::EditStack;

const BG_COLOR: Color = Color::rgb8(0x27, 0x28, 0x22);
const FG_COLOR: Color = Color::rgb8(0xf0, 0xf0, 0xea);

const FONT_HEIGHT: f64 = 12.0;

#[derive(Default)]
struct HelloState {
    size: (f64, f64),
    handle: WindowHandle,
    need_recalculate_font_size: bool,
    font_advance: f64,
    font_height: f64,
    editor: EditStack,
    delta_y: f64,
}

impl HelloState {
    fn new() -> Self {
        Self {
            size: Default::default(),
            handle: Default::default(),
            need_recalculate_font_size: true,
            font_advance: Default::default(),
            font_height: Default::default(),
            delta_y: Default::default(),
            editor: Default::default(),
        }
    }
    fn from_reader<T: Read>(reader: T) -> Result<Self> {
        EditStack::from_reader(reader).map(|r| Self {
            size: Default::default(),
            handle: Default::default(),
            need_recalculate_font_size: true,
            font_advance: Default::default(),
            font_height: Default::default(),
            delta_y: Default::default(),
            editor: r,
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
            if let Some(b) = self.editor.buffer() {
                b.line(line_idx, &mut line);
            }
            let layout = piet.text().new_text_layout(&font, &line).build().unwrap();

            piet.draw_text(&layout, (0.0, self.font_height + dy), &FG_COLOR);

            if let Some (b) = self.editor.buffer() {
                b.carrets_on_line(line_idx).for_each(|c| {
                    println!("carret {:?} on line {}",c,line_idx);
                    println!("{:?}",layout.hit_test_text_position(c.col_index));
                    if let Some(metrics) = layout.hit_test_text_position(c.col_index) {
                        piet.stroke(Line::new((metrics.point.x, self.font_height + dy + 2.2), (metrics.point.x, dy+2.2)), &FG_COLOR, 2.0);
                    }
                }
                );
                
            };

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
        //let deadline = std::time::Instant::now() + std::time::Duration::from_millis(500);
        //let id = ctx.request_timer(deadline);
        match event.key_code {
            KeyCode::ArrowRight => {self.editor.forward(false);ctx.invalidate();},
            KeyCode::ArrowLeft => {self.editor.backward(false);ctx.invalidate();},
            _ => (),
        };
        //println!("keydown: {:?}, timer id = {:?}", event, id);
        if let Some(text) = event.text() {
            println!("keydown: {}", text);
        }
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
        Some(&HotKey::new(SysMods::Cmd, "o")),
        true,
        false,
    );
    let mut menubar = Menu::new();
    menubar.add_dropdown(Menu::new(), "Application", true);
    menubar.add_dropdown(file_menu, "&File", true);

    let mut run_loop = RunLoop::new();
    let mut builder = WindowBuilder::new();
    let state = HelloState::from_reader(
        file::load(std::env::args().nth(1).unwrap())
            .unwrap()
            .buffer
            .as_bytes(),
    )
    .unwrap();

    builder.set_handler(Box::new(state));

    builder.set_title("Hello 😊︎ 😐︎ ☹︎ example");
    builder.set_menu(menubar);
    let window = builder.build().unwrap();

    window.show();
    run_loop.run();
}
