
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

impl<T> Widget<T> for EmptyWidget {
    fn event(&mut self, _ctx: &mut druid::EventCtx, _event: &druid::Event, _data: &mut T, _env: &druid::Env) {
    }
    fn lifecycle(&mut self, _ctx: &mut druid::LifeCycleCtx, _event: &druid::LifeCycle, _data: &T, _env: &druid::Env) {
    }
    fn update(&mut self, _ctx: &mut druid::UpdateCtx, _old_data: &T, _data: &T, _env: &druid::Env) {
    }
    fn layout(&mut self, _ctx: &mut druid::LayoutCtx, _bc: &druid::BoxConstraints, _data: &T, _env: &druid::Env) -> druid::Size {
        druid::Size::ZERO
    }
    fn paint(&mut self, _ctx: &mut druid::PaintCtx, _data: &T, _env: &druid::Env) {
    }
    
}
