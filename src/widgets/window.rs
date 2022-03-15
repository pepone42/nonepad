use std::{ffi::OsStr, path::Path};

use druid::{
    widget::{Flex, Label, MainAxisAlignment},
    Color, Data, Env, Lens, Widget, WidgetExt, Command, HotKey, SysMods, im::Vector,
};

use super::{text_buffer::EditStack,  Item};
use crate::commands::{self, ShowPalette};
use super::{
    bottom_panel::{self, BottonPanelState},
    editor_view,
};

pub struct Window {
    inner: Flex<NPWindowState>,
}

#[derive(Clone, Data, Lens)]
pub struct NPWindowState {
    pub editor: EditStack,
    //pub editor2: EditStack,
    status: String,
    bottom_panel: BottonPanelState,
}

impl Default for NPWindowState {
    fn default() -> Self {
        NPWindowState {
            editor: EditStack::default(),
            //editor2: EditStack::default(),
            status: "Untilted".to_owned(),
            bottom_panel: BottonPanelState::default(),
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
        })
    }
}
impl Widget<NPWindowState> for Window {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut NPWindowState, env: &druid::Env) {
        match event {
            druid::Event::KeyDown(druid::KeyEvent {
                key: druid::KbKey::Escape,
                ..
            }) if data.bottom_panel.is_open() => {
                ctx.submit_command(Command::new(commands::CLOSE_BOTTOM_PANEL, (), druid::Target::Global));
                ctx.set_handled();
                return;
            }
            druid::Event::KeyDown(event) => {
                if HotKey::new(SysMods::CmdShift, "P").matches(event) {
                    let mut items = Vector::new();
                    for c in &commands::COMMANDSET.commands {
                        items.push_back(Item::new(&c.description, &""));
                    }
                    ctx.show_palette(items, "Commands",commands::PALETTE_EXECUTE_COMMAND);
                }
                commands::COMMANDSET.hotkey_submit(ctx, event, self, data);
            }
            druid::Event::MouseUp(_) => {
                ctx.submit_command(Command::new(commands::RESET_HELD_STATE, (), druid::Target::Global))
            }
            druid::Event::Command(cmd) if cmd.is(crate::commands::PALETTE_EXECUTE_COMMAND) => {
                let index = cmd.get_unchecked(crate::commands::PALETTE_EXECUTE_COMMAND);
                let ui_cmd = &commands::COMMANDSET.commands[index.0];
                ui_cmd.exec(ctx,self,data);
                ctx.set_handled();
                return;
            }
            _ => (),
        }

        self.inner.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &NPWindowState,
        env: &druid::Env,
    ) {
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &NPWindowState, data: &NPWindowState, env: &druid::Env) {
        self.inner.update(ctx, old_data, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &NPWindowState,
        env: &druid::Env,
    ) -> druid::Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &NPWindowState, env: &druid::Env) {
        self.inner.paint(ctx, data, env);
    }
}

impl Window {
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
        Window {
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
        }
    }
}
