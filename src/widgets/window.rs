use std::{ffi::OsStr, path::Path};

use super::{
    bottom_panel::{self, BottonPanelState},
    editor_view, PaletteCommandType, PALETTE_CALLBACK,
};
use super::{text_buffer::EditStack, DialogResult, PaletteBuilder, PaletteView, PaletteViewState};
use crate::commands::{self, UICommandEventHandler};

use druid::{
    widget::{Flex, Label, MainAxisAlignment},
    Color, Data, Env, Lens, Selector, Widget, WidgetExt, WidgetPod,
};

pub(super) const RESET_HELD_STATE: Selector<()> = Selector::new("nonepad.all.reste_held_state");

pub struct NPWindow {
    inner: WidgetPod<NPWindowState, Flex<NPWindowState>>,
    palette: WidgetPod<PaletteViewState, PaletteView>,
    //in_palette: bool,
}

#[derive(Clone, Data, Lens)]
pub struct NPWindowState {
    pub editor: EditStack,
    //pub editor2: EditStack,
    status: String,
    bottom_panel: BottonPanelState,
    palette_state: PaletteViewState,
    in_palette: bool,
}

impl Default for NPWindowState {
    fn default() -> Self {
        NPWindowState {
            editor: EditStack::default(),
            //editor2: EditStack::default(),
            status: "Untilted".to_owned(),
            bottom_panel: BottonPanelState::default(),
            palette_state: PaletteViewState::default(),
            in_palette: false,
        }
    }
}

impl NPWindowState {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<NPWindowState> {
        Ok(Self {
            editor: EditStack::from_file(&path)?,
            //editor2: EditStack::default(),
            status: path
                .as_ref()
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            ..Default::default()
        })
    }
}
impl Widget<NPWindowState> for NPWindow {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut NPWindowState, env: &druid::Env) {
        commands::CommandSet.event(ctx, event, self, data);
        if ctx.is_handled() {
            return;
        }
        match event {
            druid::Event::KeyDown(druid::KeyEvent {
                key: druid::KbKey::Escape,
                ..
            }) if data.bottom_panel.is_open() && !data.in_palette => {
                ctx.submit_command(bottom_panel::CLOSE_BOTTOM_PANEL);
                ctx.set_handled();
                return;
            }
            druid::Event::MouseUp(_) => ctx.submit_command(super::window::RESET_HELD_STATE),
            druid::Event::Command(cmd) if cmd.is(PALETTE_CALLBACK) => {
                let item = cmd.get_unchecked(PALETTE_CALLBACK);
                match &item.1 {
                    PaletteCommandType::Window(action) => {
                        (action)(item.0.clone(), ctx, self, data);
                        ctx.set_handled();
                        return;
                    }
                    PaletteCommandType::DialogWindow(action) => {
                        let dialog_result = if item.0.index == 0 {
                            DialogResult::Ok
                        } else {
                            DialogResult::Cancel
                        };
                        (action)(dialog_result, ctx, self, data);
                        ctx.set_handled();
                        return;
                    }
                    _ => (),
                }
            }
            druid::Event::Command(cmd) if cmd.is(super::SHOW_PALETTE_FOR_WINDOW) => {
                data.in_palette = true;
                ctx.request_layout();
                let input = cmd.get_unchecked(super::SHOW_PALETTE_FOR_WINDOW).clone();
                self.palette.widget_mut().init(
                    &mut data.palette_state,
                    input.1,
                    input.2.clone(),
                    input.3.map(|f| PaletteCommandType::Window(f)),
                );
                self.palette.widget_mut().take_focus(ctx);
                return;
            }
            druid::Event::Command(cmd) if cmd.is(super::SHOW_PALETTE_FOR_EDITOR) => {
                data.in_palette = true;
                ctx.request_layout();
                let input = cmd.get_unchecked(super::SHOW_PALETTE_FOR_EDITOR).clone();
                self.palette.widget_mut().init(
                    &mut data.palette_state,
                    input.1,
                    input.2.clone(),
                    input.3.map(|f| PaletteCommandType::Editor(f)),
                );
                self.palette.widget_mut().take_focus(ctx);
                return;
            }
            druid::Event::Command(cmd) if cmd.is(super::SHOW_DIALOG_FOR_WINDOW) => {
                data.in_palette = true;
                ctx.request_layout();
                let input = cmd.get_unchecked(super::SHOW_DIALOG_FOR_WINDOW).clone();
                self.palette.widget_mut().init(
                    &mut data.palette_state,
                    input.1,
                    input.2.clone(),
                    input.3.map(|f| PaletteCommandType::DialogWindow(f)),
                );
                self.palette.widget_mut().take_focus(ctx);
                return;
            }
            druid::Event::Command(cmd) if cmd.is(super::SHOW_DIALOG_FOR_EDITOR) => {
                data.in_palette = true;
                ctx.request_layout();
                let input = cmd.get_unchecked(super::SHOW_DIALOG_FOR_EDITOR).clone();
                self.palette.widget_mut().init(
                    &mut data.palette_state,
                    input.1,
                    input.2.clone(),
                    input.3.map(|f| PaletteCommandType::DialogEditor(f)),
                );
                self.palette.widget_mut().take_focus(ctx);
                return;
            }
            druid::Event::Command(cmd) if cmd.is(super::CLOSE_PALETTE) => {
                // TODO: send focus to the last focused editor
                //ctx.submit_command(Command::new(commands::GIVE_FOCUS, (), Target::Global));
                ctx.focus_prev();
                data.in_palette = false;
                ctx.request_paint();
                return;
            }
            druid::Event::WindowCloseRequested => {
                if data.editor.is_dirty() {
                    ctx.set_handled();
                    self.dialog()
                        .title("Discard unsaved change?")
                        .on_select(|result, ctx, _, data| {
                            if result == DialogResult::Ok {
                                data.editor.reset_dirty();
                                ctx.submit_command(druid::commands::CLOSE_WINDOW);
                            }
                        })
                        .show(ctx);
                }
            }
            druid::Event::WindowDisconnected => {
                #[cfg(target_os = "macos")]
                ctx.submit_command(druid::commands::QUIT_APP);
            }
            _ => (),
        }
        if data.in_palette {
            self.palette.event(ctx, event, &mut data.palette_state, env);
        } else {
            self.inner.event(ctx, event, data, env);
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &NPWindowState,
        env: &druid::Env,
    ) {
        if event.should_propagate_to_hidden() {
            self.palette.lifecycle(ctx, event, &data.palette_state, env);
            self.inner.lifecycle(ctx, event, data, env);
        } else {
            if data.in_palette {
                self.palette.lifecycle(ctx, event, &data.palette_state, env);
            }
            self.inner.lifecycle(ctx, event, data, env);
        }
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &NPWindowState, data: &NPWindowState, env: &druid::Env) {
        if old_data.in_palette != data.in_palette {
            ctx.children_changed();
        }
        self.inner.update(ctx, data, env);
        if data.in_palette {
            self.palette.update(ctx, &data.palette_state, env)
        }
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &NPWindowState,
        env: &druid::Env,
    ) -> druid::Size {
        if data.in_palette {
            self.inner.layout(ctx, bc, data, env);
            self.palette.layout(ctx, bc, &data.palette_state, env)
        } else {
            self.inner.layout(ctx, bc, data, env)
        }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &NPWindowState, env: &druid::Env) {
        self.inner.paint(ctx, data, env);
        if data.in_palette {
            self.palette.paint(ctx, &data.palette_state, env);
        }
    }
}

impl NPWindow {
    pub fn build() -> Self {
        let label_left = Label::new(|data: &NPWindowState, _env: &Env| {
            format!(
                "{}{}",
                data.editor
                    .filename
                    .clone()
                    .unwrap_or_default()
                    .file_name()
                    .unwrap_or_else(|| OsStr::new("[Untilted]"))
                    .to_string_lossy()
                    .to_string(),
                if data.editor.is_dirty() { "*" } else { "" }
            )
        })
        .with_text_size(12.0);

        let label_right = Label::new(|data: &NPWindowState, _env: &Env| {
            format!(
                "{}    {}    {}    {}    {}",
                data.editor.caret_display_info(),
                data.editor.file.indentation,
                data.editor.file.encoding.name(),
                data.editor.file.linefeed,
                data.editor.file.syntax.name
            )
        })
        .with_text_size(12.0);

        let edit = editor_view::new().lens(NPWindowState::editor);
        //let edit2 = editor_view::new().lens(NPWindowState::editor2);
        NPWindow {
            inner: WidgetPod::new(
                Flex::column()
                    //.with_flex_child(Flex::row().with_flex_child(edit,0.5).with_flex_child(edit2,0.5).padding(2.0), 1.0)
                    .with_flex_child(edit.padding(2.0), 1.0)
                    .must_fill_main_axis(true)
                    .with_child(bottom_panel::build().lens(NPWindowState::bottom_panel))
                    .with_child(
                        Flex::row()
                            .with_child(label_left.padding(2.0))
                            .with_flex_spacer(1.0)
                            .with_child(label_right.padding(2.0))
                            .padding(1.0)
                            .background(Color::rgb8(0x1d, 0x1e, 0x22)), // using a Painter cause a redraw every frame
                    )
                    .main_axis_alignment(MainAxisAlignment::Center),
            ),
            palette: WidgetPod::new(PaletteView::new()),
        }
    }
}
