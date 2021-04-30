// "Hello 😊︎ 😐︎ ☹︎ example"
#![windows_subsystem = "windows"]

mod bottom_panel;
mod commands;
mod editor_view;
mod text_buffer;
mod widgets;

use std::path::Path;

use druid::widget::{Flex, Label, MainAxisAlignment};
use druid::{
    piet::Color, AppDelegate, AppLauncher, Command, Data, DelegateCtx, Env, Lens, LocalizedString, Target, Widget,
    WidgetExt, WindowDesc,
};

use crate::editor_view::EditorView;
use bottom_panel::BottonPanelState;
use text_buffer::EditStack;

#[derive(Debug)]
pub struct Delegate {
    disabled: bool,
}
impl AppDelegate<MainWindowState> for Delegate {
    fn event(
        &mut self,
        ctx: &mut druid::DelegateCtx,
        _window_id: druid::WindowId,
        event: druid::Event,
        data: &mut MainWindowState,
        _env: &Env,
    ) -> Option<druid::Event> {
        if matches!(
            event,
            druid::Event::KeyDown(druid::KeyEvent {
                key: druid::KbKey::Escape,
                ..
            })
        ) && data.bottom_panel.is_open()
        {
            ctx.submit_command(Command::new(commands::CLOSE_BOTTOM_PANEL, (), druid::Target::Global));
            return None;
        }
        Some(event)
    }
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        _cmd: &Command,
        _data: &mut MainWindowState,
        _env: &Env,
    ) -> druid::Handled {
        druid::Handled::No
    }
    fn window_added(
        &mut self,
        _id: druid::WindowId,
        _data: &mut MainWindowState,
        _env: &Env,
        _ctx: &mut druid::DelegateCtx,
    ) {
    }
    fn window_removed(
        &mut self,
        _id: druid::WindowId,
        _data: &mut MainWindowState,
        _env: &Env,
        _ctx: &mut druid::DelegateCtx,
    ) {
    }
}

#[derive(Clone, Data, Lens)]
struct MainWindowState {
    editor: EditStack,
    status: String,
    bottom_panel: BottonPanelState,
}

impl Default for MainWindowState {
    fn default() -> Self {
        MainWindowState {
            editor: EditStack::default(),
            status: "Untilted".to_owned(),
            bottom_panel: BottonPanelState::default(),
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
            bottom_panel: BottonPanelState::default(),
        })
    }
}

fn build_ui() -> impl Widget<MainWindowState> {
    let label_left = Label::new(|data: &MainWindowState, _env: &Env| {
        format!(
            "{}{}",
            data.editor
                .filename
                .clone()
                .unwrap_or_default()
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            if data.editor.is_dirty() { "*" } else { "" }
        )
    })
    .with_text_size(12.0);
    let label_right = Label::new(|data: &MainWindowState, _env: &Env| {
        format!(
            "{}    {}    {}    {}",
            data.editor.caret_display_info(),
            data.editor.file.indentation,
            data.editor.file.encoding.name(),
            data.editor.file.linefeed
        )
    })
    .with_text_size(12.0);
    let edit = EditorView::default()
        .lens(MainWindowState::editor)
        .with_id(crate::editor_view::WIDGET_ID);
    //.border(Color::rgb8(0x3a, 0x3a, 0x3a), 1.0);
    Flex::column()
        .with_flex_child(edit.padding(2.0), 1.0)
        .must_fill_main_axis(true)
        .with_child(bottom_panel::build().lens(MainWindowState::bottom_panel))
        .with_child(
            Flex::row()
                .with_child(label_left.padding(2.0))
                .with_flex_spacer(1.0)
                .with_child(label_right.padding(2.0))
                .border(Color::rgb8(0x3a, 0x3a, 0x3a), 1.0),
        )
        .main_axis_alignment(MainAxisAlignment::Center)
}

fn main() -> anyhow::Result<()> {
    let app_state = if let Some(filename) = std::env::args().nth(1) {
        MainWindowState::from_file(filename)?
    } else {
        MainWindowState::new()
    };

    let win = WindowDesc::new(build_ui).title(LocalizedString::new("NonePad"));
    AppLauncher::with_window(win)
        .delegate(Delegate { disabled: false })
        .configure_env(|env, _| {
            // env.set(druid::theme::TEXT_SIZE_NORMAL, 5.0);
            // env.set(druid::theme::TEXT_SIZE_LARGE, 8.0);
            env.set(crate::editor_view::FONT_SIZE, 12.0);
            env.set(
                crate::editor_view::FONT_DESCRIPTOR,
                druid::FontDescriptor::new(druid::FontFamily::MONOSPACE),
            );

            env.set(crate::editor_view::BG_COLOR, Color::rgb8(34, 40, 42));
            env.set(crate::editor_view::FG_COLOR, Color::rgb8(241, 242, 243));
            env.set(crate::editor_view::FG_SEL_COLOR, Color::rgb8(59, 73, 75));
            env.set(crate::editor_view::BG_SEL_COLOR, Color::rgb8(79, 97, 100));
        })
        .launch(app_state)?;
    Ok(())
}
