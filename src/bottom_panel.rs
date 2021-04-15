use druid::{Command, Data, Env, Event, EventCtx, Lens, Target, Widget, WidgetExt, widget::{Button, Controller, Flex, Label, TextBox, ViewSwitcher}};

use crate::commands;
use crate::widgets::{EmptyWidget, Extension};

pub const PANEL_CLOSED: usize = 0x0;
pub const PANEL_SEARCH: usize = 0x1;

pub struct BottomPanel;

#[derive(Debug, Clone, Data, Lens, Default)]
pub struct BottonPanelState {
    pub current: usize,
    search_state: SearchState,
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
            PANEL_CLOSED => Box::new(EmptyWidget {}),
            PANEL_SEARCH => Box::new(build_search_panel().lens(BottonPanelState::search_state)),
            _ => unreachable!(),
        },
    );
    view_switcher.controller(BottomPanel {})
}

impl<W: Widget<BottonPanelState>> Controller<BottonPanelState, W> for BottomPanel {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut BottonPanelState, env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(commands::CLOSE_BOTTOM_PANEL) => {
                ctx.submit_command(Command::new(commands::GIVE_FOCUS, (), crate::editor_view::WIDGET_ID));
                data.current = PANEL_CLOSED;
                return;
            }
            Event::Command(cmd) if cmd.is(commands::SHOW_SEARCH_PANEL) => {
                data.current = PANEL_SEARCH;
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
        .with_child(Label::new("Search"))
        .with_flex_child(
            TextBox::new()
                .on_enter(|ctx, data: &mut String, _| {
                    ctx.submit_command(Command::new(commands::REQUEST_NEXT_SEARCH, data.clone(),Target::Global))
                })
                .focus()
                .lens(SearchState::s)
                .expand_width(),
            1.0,
        )
        .with_child(
            Button::new("x")
                .on_click(|ctx, _, _| (ctx.submit_command(Command::new(commands::CLOSE_BOTTOM_PANEL, (),Target::Global)))),
        )
}
