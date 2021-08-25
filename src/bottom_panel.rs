use druid::{
    widget::{Controller, Flex, Label, TextBox, ViewSwitcher},
    Command, Data, Env, Event, EventCtx, Lens, Target, Widget, WidgetExt, WidgetId,
};
use once_cell::unsync::Lazy;

use crate::widgets::{EmptyWidget, Extension};
use crate::{commands, text_buffer::EditStack, widgets};

pub const PANEL_CLOSED: usize = 0x0;
pub const PANEL_SEARCH: usize = 0x1;
pub const PANEL_PALETTE: usize = 0x2;

pub struct BottomPanel {}

#[derive(Debug, Clone, Data, Lens, Default)]
pub struct BottonPanelState {
    pub current: usize,
    search_state: SearchState,
    panel_state: PaletteState,
}

impl BottonPanelState {
    pub fn is_open(&self) -> bool {
        self.current != 0
    }
}

pub fn build() -> impl Widget<BottonPanelState> {
    let view_switcher = ViewSwitcher::new(
        |data: &BottonPanelState, _env| data.current,
        |selector, _data, _env| match *selector {
            PANEL_CLOSED => Box::new(EmptyWidget::default()),
            PANEL_SEARCH => Box::new(build_search_panel().lens(BottonPanelState::search_state)),
            PANEL_PALETTE => Box::new(build_palette_panel().lens(BottonPanelState::panel_state)),
            _ => unreachable!(),
        },
    );
    let panel_id = WidgetId::next();
    view_switcher.with_id(panel_id).controller(BottomPanel {})
}

impl<W: Widget<BottonPanelState>> Controller<BottonPanelState, W> for BottomPanel {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut BottonPanelState, env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(commands::CLOSE_BOTTOM_PANEL) => {
                data.current = PANEL_CLOSED;
                ctx.submit_command(Command::new(commands::GIVE_FOCUS, (), Target::Global));

                return;
            }
            Event::Command(cmd) if cmd.is(commands::SHOW_SEARCH_PANEL) => {
                data.current = PANEL_SEARCH;
                let id = child.id().unwrap();
                let input = cmd.get_unchecked(commands::SHOW_SEARCH_PANEL).clone();
                ctx.submit_command(Command::new(commands::GIVE_FOCUS, (), id));
                ctx.submit_command(Command::new(commands::SEND_DATA, input, id));
                return;
            }
            Event::Command(cmd) if cmd.is(commands::SHOW_PALETTE_PANEL) => {
                data.current = PANEL_PALETTE;
                let id = child.id().unwrap();
                ctx.submit_command(Command::new(commands::GIVE_FOCUS, (), id));
                return;
            }
            _ => (),
        }
        child.event(ctx, event, data, env)
    }
}

#[derive(Debug, Clone, Data, Lens, Default)]
struct SearchState {
    s: String,
}

fn build_search_panel() -> impl Widget<SearchState> {
    Flex::row()
        .with_child(Label::new("Search").with_text_size(12.0))
        .with_flex_child(
            TextBox::new()
                .with_text_size(12.0)
                .on_enter(|ctx, data: &mut String, _| {
                    ctx.submit_command(Command::new(
                        commands::REQUEST_NEXT_SEARCH,
                        data.clone(),
                        Target::Global,
                    ))
                })
                .focus()
                .on_data_received(|_ctx, state: &mut String, data: &String, _| {
                    state.clone_from(data);
                })
                .lens(SearchState::s)
                .expand_width(),
            1.0,
        )
}
#[derive(Debug, Clone, Data, Lens, Default)]
struct PaletteState {
    s: EditStack,
}
fn build_palette_panel() -> impl Widget<PaletteState> {
    Flex::column().with_child(
        //TextBox::new().with_text_size(12.0)
        druid::widget::Container::new(
            crate::editor_view::EditorView::new(WidgetId::reserved(1234))
                .on_enter(|ctx, data: &mut crate::EditStack, _| {
                    dbg!(data);
                    ctx.submit_command(Command::new(commands::REQUEST_CLOSE_BOTTOM_PANEL, (), Target::Global));
                })
                .focus()
                .lens(PaletteState::s)
                .expand_width(),
        )
        .rounded(druid::theme::TEXTBOX_BORDER_RADIUS)
        .border(druid::theme::BORDER_DARK, druid::theme::TEXTBOX_BORDER_WIDTH),
    )
}
