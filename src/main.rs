// "Hello 😊︎ 😐︎ ☹︎ example"
mod carret;
mod dialog;
mod editor_view;
mod file;
mod position;
mod rope_utils;
mod text_buffer;

use std::any::Any;

use std::{
    error::Error,
    path::Path,
    sync::{Arc, Mutex},
};

use druid::widget::{CrossAxisAlignment, Flex, Label, MainAxisAlignment};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc};

use druid::piet::Color;

use crate::editor_view::EditorView;
use text_buffer::EditStack;

const BG_COLOR: Color = Color::rgb8(34, 40, 42);
const FG_COLOR: Color = Color::rgb8(241, 242, 243);
const FG_SEL_COLOR: Color = Color::rgb8(59, 73, 75);
const BG_SEL_COLOR: Color = Color::rgb8(79, 97, 100);

const FONT_HEIGHT: f64 = 13.0;

#[derive(Clone, Data, Lens)]
struct MainWindowState {
    editor: EditStack,
    status: String,
}

impl Default for MainWindowState {
    fn default() -> Self {
        MainWindowState {
            editor: EditStack::default(),
            status: "Untilted".to_owned(),
        }
    }
}

impl MainWindowState {
    fn new() -> Self {
        Self { ..Default::default() }
    }

    fn from_file<'a, P: AsRef<Path>>(path: P) -> anyhow::Result<MainWindowState> {
        Ok(Self {
            editor: EditStack::from_file(&path)?,
            status: path
                .as_ref()
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
        })
    }
}

fn build_ui() -> impl Widget<MainWindowState> {
    let label = Label::new(|data: &MainWindowState, _env: &Env| data.editor.filename.clone().unwrap_or_default().to_string_lossy().to_string()).with_text_size(10.0);
    let edit = EditorView::default().lens(MainWindowState::editor);
    Flex::column()
        .with_flex_child(edit, 1.0)
        .must_fill_main_axis(true)
        .with_child(label.padding(2.0))
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .main_axis_alignment(MainAxisAlignment::Center)
}

fn main() -> Result<(), PlatformError> {

    let app_state = if let Some(filename) = std::env::args().nth(1) {
        MainWindowState::from_file(filename).unwrap()
    } else {
        MainWindowState::new()
    };

    let win = WindowDesc::new(build_ui).title(LocalizedString::new("NonePad"));
    AppLauncher::with_window(win).launch(app_state)?;
    Ok(())
}
