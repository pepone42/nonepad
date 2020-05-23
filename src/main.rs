// "Hello 😊︎ 😐︎ ☹︎ example"
mod dialog;
mod file;
mod text_buffer;
mod editor_view;
mod carret;
mod rope_utils;
mod position;

use std::any::Any;

use std::{sync::{Mutex, Arc}, path::Path};

use druid::{AppLauncher, WindowDesc, Widget, PlatformError, Data, LocalizedString};
use druid::widget::Label;

use druid::piet::Color;

use crate::editor_view::EditorView;
use text_buffer::EditStack;

const BG_COLOR: Color = Color::rgb8(34, 40, 42);
const FG_COLOR: Color = Color::rgb8(241, 242, 243);
const FG_SEL_COLOR: Color = Color::rgb8(59, 73, 75);
const BG_SEL_COLOR: Color = Color::rgb8(79, 97, 100);

const FONT_HEIGHT: f64 = 13.0;

#[derive(Clone, Data)]
struct MainWindowState {
    editor: Arc<Mutex<EditorView>>,
}

impl Default for MainWindowState {
    fn default() -> Self {
        MainWindowState {
            editor: Arc::new(Mutex::new(EditorView::default()))
        }
    }
}

impl MainWindowState {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn from_file<'a, P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let editor_view = EditorView::from_file(path)?;
        Ok(Self {
            editor: Arc::new(Mutex::new(editor_view))
        })
    }
}

fn build_ui() -> EditorView {
    EditorView::default()
}

fn main()  -> Result<(), PlatformError> {
    let stack = if let Some(filename) = std::env::args().nth(1) {
        EditStack::from_file(filename).unwrap()
    } else {
        EditStack::new()
    };


    let win = WindowDesc::new(build_ui).title(LocalizedString::new("NonePad"));
    AppLauncher::with_window(win).launch(stack)?;
    Ok(())
}
