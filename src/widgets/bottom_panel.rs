use druid::{
    widget::{Controller, Flex, Label, TextBox, ViewSwitcher},
    Command, Data, Env, Event, EventCtx, KbKey, KeyEvent, Lens, Selector, Target, Widget, WidgetExt, WidgetId,
};

use crate::widgets::{EmptyWidget, Extension, PaletteViewState};

pub const PANEL_CLOSED: usize = 0x0;
pub const PANEL_SEARCH: usize = 0x1;

pub const SHOW_SEARCH_PANEL: Selector<String> = Selector::new("nonepad.bottom_panel.show_search");
pub const SEND_STRING_DATA: Selector<String> = Selector::new("nonepad.all.send_data");
pub const CLOSE_BOTTOM_PANEL: Selector<()> = Selector::new("nonepad.bottom_panel.close");

pub struct BottomPanel {}

#[derive(Debug, Clone, Data, Lens, Default)]
pub struct BottonPanelState {
    pub current: usize,
    search_state: SearchState,
    panel_state: PaletteViewState,
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
            _ => unreachable!(),
        },
    );
    let panel_id = WidgetId::next();
    view_switcher.with_id(panel_id).controller(BottomPanel {})
}

impl<W: Widget<BottonPanelState>> Controller<BottonPanelState, W> for BottomPanel {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut BottonPanelState, env: &Env) {
        match event {
            Event::KeyDown(KeyEvent { key: KbKey::Escape, .. }) => {
                data.current = PANEL_CLOSED;
                ctx.focus_prev();
                return;
            }
            Event::Command(cmd) if cmd.is(CLOSE_BOTTOM_PANEL) => {
                data.current = PANEL_CLOSED;
                ctx.focus_prev();
                return;
            }
            Event::Command(cmd) if cmd.is(SHOW_SEARCH_PANEL) => {
                data.current = PANEL_SEARCH;
                let input = cmd.get_unchecked(SHOW_SEARCH_PANEL).clone();
                ctx.submit_command(SEND_STRING_DATA.with(input).to(ctx.widget_id()));
                return;
            }
            _ => (),
        }
        child.event(ctx, event, data, env)
    }
    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &BottonPanelState,
        env: &Env,
    ) {
        child.lifecycle(ctx, event, data, env);
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
                    ctx.submit_command(super::editor_view::REQUEST_NEXT_SEARCH.with(data.clone()))
                })
                .focus()
                .on_data_received(|ctx, state: &mut String, data: &String, _| {
                    ctx.request_focus();
                    state.clone_from(data);
                })
                .lens(SearchState::s)
                .expand_width(),
            1.0,
        )
}
