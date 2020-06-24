use druid::Widget;

pub struct EmptyWidget;

impl<T> Widget<T> for EmptyWidget {
    fn event(&mut self, _ctx: &mut druid::EventCtx, _event: &druid::Event, _data: &mut T, _env: &druid::Env) {}
    fn lifecycle(&mut self, _ctx: &mut druid::LifeCycleCtx, _event: &druid::LifeCycle, _data: &T, _env: &druid::Env) {}
    fn update(&mut self, _ctx: &mut druid::UpdateCtx, _old_data: &T, _data: &T, _env: &druid::Env) {}
    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx,
        _bc: &druid::BoxConstraints,
        _data: &T,
        _env: &druid::Env,
    ) -> druid::Size {
        druid::Size::ZERO
    }
    fn paint(&mut self, _ctx: &mut druid::PaintCtx, _data: &T, _env: &druid::Env) {}
}
