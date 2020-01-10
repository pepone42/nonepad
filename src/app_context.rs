#[derive(Debug)]
pub struct PaletteItem {
    name: String,
    desc: String,
}

struct CommandPalette {
    callback: Box<dyn Fn(usize)>,
}

#[derive(Default)]
pub struct AppContext {
    palette: Option<CommandPalette>,
}

impl AppContext {
    pub fn open_palette<'a, F: 'static + Fn(usize)>(&mut self, items: Vec<PaletteItem>, f: F) {
        self.palette = Some(CommandPalette {
            callback: Box::new(f),
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
}
