use druid::{widget::{Controller, ControllerHost}, Data, EventCtx, Env, Event, KeyEvent, Widget, KeyCode, Command};

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
        match event {
            Event::KeyDown(KeyEvent {
                key_code: KeyCode::NumpadEnter,
                ..
            })
            | Event::KeyDown(KeyEvent {
                key_code: KeyCode::Return,
                ..
            }) => {
                (self.action)(ctx, data, env);
            }
            _ => (),
        }

        child.event(ctx, event, data, env)
    }
}

pub struct TakeFocus;
impl TakeFocus {
    pub fn new() -> Self {
        TakeFocus{}
    }
}
impl<T: Data, W: Widget<T>> Controller<T, W> for TakeFocus {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(crate::commands::GIVE_FOCUS) => ctx.request_focus(),
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
            ctx.submit_command(Command::new(crate::commands::GIVE_FOCUS, ()), ctx.widget_id());
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
}

impl<T: Data, W: Widget<T> + 'static> Extension<T> for W {}