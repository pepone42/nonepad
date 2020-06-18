
use druid::{Data, Widget, widget::{Label, ViewSwitcher}, Lens, WidgetExt, Selector};

pub const PANEL_CLOSED: usize = 0x0;
pub const PANEL_SEARCH: usize = 0x1;


pub const SHOW_SEARCH_PANEL: Selector<()> = Selector::new("nonepad.search");

#[derive(Debug,Clone,Data, Lens, Default)]
pub struct BottonPanelState {
    pub current: usize,
}
pub fn build() -> impl Widget<BottonPanelState> {
    let view_switcher = ViewSwitcher::new(
        |data: &BottonPanelState, _env|  data.current,
        |selector,_data,_env| match *selector {
            PANEL_CLOSED => Box::new(EmptyWidget{}),
            PANEL_SEARCH => Box::new(Label::new("Search").center()),
            _ => unreachable!()
        }
    );
    view_switcher
}


struct EmptyWidget;

impl Widget<BottonPanelState> for EmptyWidget {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut BottonPanelState, env: &druid::Env) {
    }
    fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &druid::LifeCycle, data: &BottonPanelState, env: &druid::Env) {
    }
    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &BottonPanelState, data: &BottonPanelState, env: &druid::Env) {
    }
    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &BottonPanelState, env: &druid::Env) -> druid::Size {
        druid::Size::ZERO
    }
    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &BottonPanelState, env: &druid::Env) {
    }
    
}