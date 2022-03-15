use std::{ffi::OsStr, path::Path};

use druid::{
    widget::{Flex, Label, MainAxisAlignment, IdentityWrapper},
    Color, Data, Env, Lens, Widget, WidgetExt, Command, HotKey, SysMods, im::Vector, WidgetId, Size,
};

use super::{text_buffer::EditStack,  Item, PaletteList, PaletteListState};
use crate::commands::{self, ShowPalette, UICommandType};
use super::{
    bottom_panel::{self, BottonPanelState},
    editor_view,
};

pub struct NPWindow {
    inner: Flex<NPWindowState>,
    palette: IdentityWrapper<PaletteList>,
    in_palette: bool,
}

#[derive(Clone, Data, Lens)]
pub struct NPWindowState {
    pub editor: EditStack,
    //pub editor2: EditStack,
    status: String,
    bottom_panel: BottonPanelState,
    palette_state: PaletteListState,
}

impl Default for NPWindowState {
    fn default() -> Self {
        NPWindowState {
            editor: EditStack::default(),
            //editor2: EditStack::default(),
            status: "Untilted".to_owned(),
            bottom_panel: BottonPanelState::default(),
            palette_state: PaletteListState::default(),
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
            palette_state: PaletteListState::default(),
        })
    }
}
impl Widget<NPWindowState> for NPWindow {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut NPWindowState, env: &druid::Env) {
        if self.in_palette {
            self.palette.event(ctx, event, &mut data.palette_state, env);
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
            druid::Event::KeyDown(event) => {
                if HotKey::new(SysMods::CmdShift, "P").matches(event) {
                    let mut items = Vector::new();
                    for c in &commands::COMMANDSET.commands {
                        items.push_back(Item::new(&c.description, &""));
                    }
                    ctx.show_palette(items, "Commands",UICommandType::Window(|index,_name,ctx, win, data| {
                        let ui_cmd = &commands::COMMANDSET.commands[index];
                        ui_cmd.exec(ctx,win,data);
                    }));
                }
                commands::COMMANDSET.hotkey_submit(ctx, event, self, data);
            }
            druid::Event::MouseUp(_) => {
                ctx.submit_command(Command::new(commands::RESET_HELD_STATE, (), druid::Target::Global))
            }
            druid::Event::Command(cmd) if cmd.is(crate::commands::PALETTE_CALLBACK) => {
                let item = cmd.get_unchecked(crate::commands::PALETTE_CALLBACK);
                if let UICommandType::Window(action) = item.2 {
                    (action)(item.0,item.1.clone(),ctx,self,data);
                    ctx.set_handled();
                    return;
                }
            }
            druid::Event::Command(cmd) if cmd.is(commands::SHOW_PALETTE_PANEL) => {
                dbg!("here");
                self.in_palette = true;
                ctx.request_layout();
                let id = self.palette.id().unwrap();
                let input = cmd.get_unchecked(commands::SHOW_PALETTE_PANEL).clone();
                ctx.submit_command(Command::new(commands::SEND_PALETTE_PANEL_DATA, input, id));
                ctx.submit_command(Command::new(commands::GIVE_FOCUS, (), id));
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
        self.palette.lifecycle(ctx, event, &data.palette_state, env);
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &NPWindowState, data: &NPWindowState, env: &druid::Env) {
        self.inner.update(ctx, old_data, data, env);
        self.palette.update(ctx, &old_data.palette_state, &data.palette_state, env)
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &NPWindowState,
        env: &druid::Env,
    ) -> druid::Size {
        if self.in_palette {
            self.palette.layout(ctx, &bc.shrink((500.,500.)), &data.palette_state, env)
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
            palette: PaletteList::new().with_id(WidgetId::next()),
            in_palette: false,
        }
    }
}
