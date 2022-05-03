use druid::{
    widget::{Controller, ControllerHost},
    Data, Env, Event, EventCtx, KbKey, KeyEvent, Widget, Selector,
};

pub struct OnEnter<T> {
    action: Box<dyn Fn(&mut EventCtx, &mut T, &Env)>,
}
impl<T: Data> OnEnter<T> {
    pub fn new(action: impl Fn(&mut EventCtx, &mut T, &Env) + 'static) -> Self {
        OnEnter {
            action: Box::new(action),
        }
    }
}

impl<T: Data, W: Widget<T>> Controller<T, W> for OnEnter<T> {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        if let Event::KeyDown(KeyEvent { key: KbKey::Enter, .. }) = event {
            (self.action)(ctx, data, env);
            ctx.set_handled();            
        } else {
            child.event(ctx, event, data, env)
        }
    }
}

pub struct SendData<T> {
    action: Box<dyn Fn(&mut EventCtx, &mut T, &String, &Env)>,
}
impl<T: Data> SendData<T> {
    pub fn new(action: impl Fn(&mut EventCtx, &mut T, &String, &Env) + 'static) -> Self {
        SendData {
            action: Box::new(action),
        }
    }
}

impl<T: Data, W: Widget<T>> Controller<T, W> for SendData<T> {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, state: &mut T, env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(super::bottom_panel::SEND_STRING_DATA) => {
                let data = cmd.get_unchecked(super::bottom_panel::SEND_STRING_DATA);
                (self.action)(ctx, state, data, env);
            }
            _ => (),
        }
        child.event(ctx, event, state, env)
    }
}

const AUTO_FOCUS: Selector<()> = Selector::new("nonepad.extension.autofocus");

pub struct TakeFocus;
impl TakeFocus {
    pub fn new() -> Self {
        TakeFocus {}
    }
}
impl<T: Data, W: Widget<T>> Controller<T, W> for TakeFocus {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(AUTO_FOCUS) => {
                ctx.request_focus();
                ctx.set_handled();
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
        data: &T,
        env: &Env,
    ) {
        if let druid::LifeCycle::WidgetAdded = event {
            ctx.submit_command(AUTO_FOCUS.to(ctx.widget_id()));
        }
        child.lifecycle(ctx, event, data, env)
    }
}

pub trait Extension<T: Data>: Widget<T> + Sized + 'static {
    fn on_enter(self, f: impl Fn(&mut EventCtx, &mut T, &Env) + 'static) -> ControllerHost<Self, OnEnter<T>> {
        ControllerHost::new(self, OnEnter::new(f))
    }
    fn focus(self) -> ControllerHost<Self, TakeFocus> {
        ControllerHost::new(self, TakeFocus::new())
    }
    fn on_data_received(
        self,
        f: impl Fn(&mut EventCtx, &mut T, &String, &Env) + 'static,
    ) -> ControllerHost<Self, SendData<T>> {
        ControllerHost::new(self, SendData::new(f))
    }
}

impl<T: Data, W: Widget<T> + 'static> Extension<T> for W {}
