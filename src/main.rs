// "Hello 😊︎ 😐︎ ☹︎ example"
mod dialog;
mod file;
mod text_buffer;
mod editor_view;
mod app_context;

use std::any::Any;
use std::io::Result;

use std::path::Path;

use druid_shell::kurbo::Vec2;
use druid_shell::piet::{Piet, Color};

use druid_shell::{
    Application, Cursor, HotKey, KeyCode, KeyEvent, KeyModifiers,
    Menu, MouseEvent, RunLoop, SysMods, TimerToken, WinCtx, WinHandler, WindowBuilder,
    WindowHandle,
};

use crate::app_context::AppContext;
use crate::editor_view::EditorView;

const BG_COLOR: Color = Color::rgb8(0x2f, 0x4f, 0x4f);
const FG_COLOR: Color = Color::rgb8(0xdb, 0xd0, 0xa7);
const SEL_COLOR: Color = Color::rgb8(0xdb, 0x00, 0x00);

const FONT_HEIGHT: f64 = 13.0;

#[derive(Default)]
struct MainWindowState {
    size: (f64, f64),
    handle: WindowHandle,
    editor: EditorView,
    app_context: AppContext,
}

impl MainWindowState {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn from_file<'a, P: AsRef<Path>>(path: P) -> Result<Self> {
        let editor_view = EditorView::from_file(path)?;
        Ok(Self {
            editor: editor_view,
            ..Default::default()
        })
    }
}

impl WinHandler for MainWindowState {
    fn connect(&mut self, handle: &WindowHandle) {
        self.handle = handle.clone();
    }

    fn paint(&mut self, piet: &mut Piet, _ctx: &mut dyn WinCtx) -> bool {
        let mut repaint = self.editor.paint(piet, _ctx);
        if self.app_context.is_palette_active() {
            repaint = self.app_context.paint_palette(piet,_ctx);
            //unimplemented!();
        }

        repaint
    }

    fn command(&mut self, id: u32, _ctx: &mut dyn WinCtx) {
        match id {
            0x100 => {
                self.handle.close();
                Application::quit();
            }
            _ => println!("unexpected id {}", id),
        }
    }

    fn key_down(&mut self, event: KeyEvent, ctx: &mut dyn WinCtx) -> bool {
        if self.app_context.is_palette_active() {
            //unimplemented!()
            self.app_context.close_palette(0);
            true
        } else {
            self.editor.key_down(event, ctx, &mut self.app_context)
        }
    }

    fn wheel(&mut self, delta: Vec2, mods: KeyModifiers, ctx: &mut dyn WinCtx) {
        return if self.app_context.is_palette_active() {
            unimplemented!()
        } else {
            self.editor.wheel(delta, mods, ctx);
        };
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
        self.editor.size(width, height, self.handle.get_dpi(), _ctx,);
        if self.app_context.is_palette_active() {
            self.app_context.size(width, height, self.handle.get_dpi(), _ctx,);
        }
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
        MainWindowState::from_file(filename).unwrap()
    } else {
        MainWindowState::new()
    };

    builder.set_handler(Box::new(state));

    builder.set_title("NonePad");
    builder.set_menu(menubar);
    let window = builder.build().unwrap();

    window.show();
    run_loop.run();
}
