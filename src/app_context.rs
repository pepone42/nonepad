use druid_shell::kurbo::{Line, Rect, Size, Vec2};
use druid_shell::piet::{FontBuilder, Piet, RenderContext, Text, TextLayout, TextLayoutBuilder};
use druid_shell::{FileDialogOptions, HotKey, KeyCode, KeyEvent, KeyModifiers, SysMods, WinCtx};

#[derive(Debug)]
pub struct PaletteItem {
    name: String,
    desc: String,
}

pub struct CommandPalette {
    callback: Box<dyn Fn(usize)>,
    items: Vec<PaletteItem>,
}

#[derive(Default)]
pub struct AppContext {
    size: Size,
    palette: Option<CommandPalette>,
}

impl AppContext {
    pub fn open_palette<'a, F: 'static + Fn(usize)>(&mut self, items: Vec<PaletteItem>, f: F) {
        self.palette = Some(CommandPalette {
            callback: Box::new(f),
            items,
        });
    }

    pub fn close_palette(&mut self, result: usize) {
        if let Some(palette) = &self.palette {
            (palette.callback)(result)
        }
        self.palette = None;
    }

    pub fn is_palette_active(&self) -> bool {
        self.palette.is_some()
    }

    pub fn paint_palette(&mut self, piet: &mut Piet, _ctx: &mut dyn WinCtx) -> bool {
        let rect = Rect::new(self.size.width / 2. - 200., 0.0, 400., 400.);
        piet.fill(rect, &crate::BG_COLOR);
        false
    }

    pub fn size(&mut self, width: u32, height: u32, dpi: f32, _ctx: &mut dyn WinCtx) {
        let dpi_scale = dpi as f64 / 96.0;
        let width_f = (width as f64) / dpi_scale;
        let height_f = (height as f64) / dpi_scale;
        self.size = Size::new(width_f, height_f);
    }
}
