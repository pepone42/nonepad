use std::{ffi::OsStr, path::Path};

use super::{
    bottom_panel::{self, BottonPanelState},
    editor_view,
};
use super::{text_buffer::EditStack, PaletteViewState, PaletteView, DialogResult, PaletteBuilder};
use crate::commands::{self, UICommandType, UICommandEventHandler};

use druid::{
    widget::{Flex, Label, MainAxisAlignment},
    Color, Command, Data, Env, Lens, LifeCycle, Target, Widget, WidgetExt,
};

pub struct NPWindow {
    inner: Flex<NPWindowState>,
    palette: PaletteView,
    in_palette: bool,
}

#[derive(Clone, Data, Lens)]
pub struct NPWindowState {
    pub editor: EditStack,
    //pub editor2: EditStack,
    status: String,
    bottom_panel: BottonPanelState,
    palette_state: PaletteViewState,
}

impl Default for NPWindowState {
    fn default() -> Self {
        NPWindowState {
            editor: EditStack::default(),
            //editor2: EditStack::default(),
            status: "Untilted".to_owned(),
            bottom_panel: BottonPanelState::default(),
            palette_state: PaletteViewState::default(),
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
            bottom_panel: BottonPanelState::default(),
            palette_state: PaletteViewState::default(),
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
            }) if data.bottom_panel.is_open() => {
                ctx.submit_command(Command::new(commands::CLOSE_BOTTOM_PANEL, (), druid::Target::Global));
                ctx.set_handled();
                return;
            }
            // druid::Event::KeyDown(event) => {
            //     // #[cfg(target_os = "macos")]
            //     // let p = "p";
            //     // #[cfg(not(target_os = "macos"))]
            //     // let p = "P";
            //     // if HotKey::new(SysMods::CmdShift, p).matches(event) && !self.in_palette {
            //     //     let mut items = Vector::new();
            //     //     for c in &commands::WINCOMMANDSET.commands {
            //     //         items.push_back(Item::new(&c.description, &""));
            //     //     }
            //     //     self.palette()
            //     //         .items(items)
            //     //         .on_select(|result, ctx, win, data| {
            //     //             let ui_cmd = &commands::WINCOMMANDSET.commands[result.index];
            //     //             ui_cmd.exec(ctx, win, data);
            //     //         })
            //     //         .show(ctx);
            //     // }
                
            // }
            druid::Event::MouseUp(_) => {
                ctx.submit_command(Command::new(commands::RESET_HELD_STATE, (), druid::Target::Global))
            }
            druid::Event::Command(cmd) if cmd.is(crate::commands::PALETTE_CALLBACK) => {
                let item = cmd.get_unchecked(crate::commands::PALETTE_CALLBACK);
                match &item.1 {
                    UICommandType::Window(action) => {
                        (action)(item.0.clone(), ctx, self, data);
                        ctx.set_handled();
                        return;
                    }
                    UICommandType::DialogWindow(action) => {
                        let dialog_result = if item.0.index == 0 { DialogResult::Ok} else {DialogResult::Cancel};
                        (action)(dialog_result, ctx, self, data);
                        ctx.set_handled();
                        return;
                    }
                    _ => (),
                }
            }
            druid::Event::Command(cmd) if cmd.is(super::SHOW_PALETTE_FOR_WINDOW) => {
                self.in_palette = true;
                ctx.request_layout();
                let input = cmd.get_unchecked(super::SHOW_PALETTE_FOR_WINDOW).clone();
                self.palette
                    .init(&mut data.palette_state, input.1, input.2.clone(), input.3.map(|f| UICommandType::Window(f)));
                self.palette.take_focus(ctx);
                return;
            }
            druid::Event::Command(cmd) if cmd.is(super::SHOW_PALETTE_FOR_EDITOR) => {
                self.in_palette = true;
                ctx.request_layout();
                let input = cmd.get_unchecked(super::SHOW_PALETTE_FOR_EDITOR).clone();
                self.palette
                    .init(&mut data.palette_state, input.1, input.2.clone(), input.3.map(|f| UICommandType::Editor(f)));
                self.palette.take_focus(ctx);
                return;
            }
            druid::Event::Command(cmd) if cmd.is(super::SHOW_DIALOG_FOR_WINDOW) => {
                self.in_palette = true;
                ctx.request_layout();
                let input = cmd.get_unchecked(super::SHOW_DIALOG_FOR_WINDOW).clone();
                self.palette
                    .init(&mut data.palette_state, input.1, input.2.clone(), input.3.map(|f| UICommandType::DialogWindow(f)));
                self.palette.take_focus(ctx);
                return;
            }
            druid::Event::Command(cmd) if cmd.is(super::SHOW_DIALOG_FOR_EDITOR) => {
                self.in_palette = true;
                ctx.request_layout();
                let input = cmd.get_unchecked(super::SHOW_DIALOG_FOR_EDITOR).clone();
                self.palette
                    .init(&mut data.palette_state, input.1, input.2.clone(), input.3.map(|f| UICommandType::DialogEditor(f)));
                self.palette.take_focus(ctx);
                return;
            }
            druid::Event::Command(cmd) if cmd.is(commands::CLOSE_PALETTE) => {
                // ctx.request_focus don't work. I guess it needs to be delayed
                // TODO: send focus to the last focused editor
                ctx.submit_command(Command::new(commands::GIVE_FOCUS, (), Target::Global));
                self.in_palette = false;
                ctx.request_paint();
                return;
            }
            druid::Event::WindowCloseRequested => {
                if data.editor.is_dirty() {
                    ctx.set_handled();
                    self.dialog()
                        //.items(item!["Yes", "No"])
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
        if self.in_palette {
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
        match event {
            LifeCycle::WidgetAdded => {
                self.palette.lifecycle(ctx, event, &data.palette_state, env);
                self.inner.lifecycle(ctx, event, data, env);
            }
            _ => {
                if self.in_palette {
                    self.palette.lifecycle(ctx, event, &data.palette_state, env);
                }
                self.inner.lifecycle(ctx, event, data, env);
            }
        }
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &NPWindowState, data: &NPWindowState, env: &druid::Env) {
        self.inner.update(ctx, old_data, data, env);
        if self.in_palette {
            self.palette
                .update(ctx, &old_data.palette_state, &data.palette_state, env)
        }
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &NPWindowState,
        env: &druid::Env,
    ) -> druid::Size {
        if self.in_palette {
            self.inner.layout(ctx, bc, data, env);
            self.palette.layout(ctx, bc, &data.palette_state, env)
        } else {
            self.inner.layout(ctx, bc, data, env)
        }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &NPWindowState, env: &druid::Env) {
        self.inner.paint(ctx, data, env);
        if self.in_palette {
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
            inner: Flex::column()
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
            palette: PaletteView::new(),
            in_palette: false,
        }
    }
}
