use std::path::Path;

use super::{
    editor_view::{EditorView, TextEditor},
    text_buffer::EditStack,
};
use druid::{
    im::{vector, Vector},
    Data, Event, Lens, Selector, Widget, WidgetPod,
};
#[derive(Clone, Data, Lens)]
pub struct NPViewSwitcherState {
    pub editors: Vector<EditStack>,
    active_editor_index: usize,
}

impl Default for NPViewSwitcherState {
    fn default() -> Self {
        Self {
            editors: vector![EditStack::default()],
            active_editor_index: Default::default(),
        }
    }
}

impl NPViewSwitcherState {
    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<NPViewSwitcherState> {
        Ok(Self {
            editors: vector![EditStack::from_file(&path)?],
            active_editor_index: 0,
        })
    }
    pub fn active_editor(&self) -> &EditStack {
        &self.editors[self.active_editor_index]
    }
    pub fn active_editor_mut(&mut self) -> &mut EditStack {
        &mut self.editors[self.active_editor_index]
    }
}

pub struct NPViewSwitcher {
    views: Vec<WidgetPod<EditStack, TextEditor>>,
}

pub const NEW_EDITVIEW: Selector<()> = Selector::new("nonepad.viewswitcher.new_editview");
pub(super) const ACTIVATE_EDITVIEW: Selector<()> = Selector::new("nonepad.viewswitcher.activate_editview");

impl Widget<NPViewSwitcherState> for NPViewSwitcher {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut NPViewSwitcherState,
        env: &druid::Env,
    ) {
        match event {
            Event::Command(c) if c.is(NEW_EDITVIEW) => {
                self.views.push(WidgetPod::new(TextEditor::default()));
                data.editors.push_back(EditStack::new());
                data.active_editor_index += 1;

                ctx.children_changed();

                ctx.submit_command(ACTIVATE_EDITVIEW.to(self.views[data.active_editor_index].id()));
                ctx.set_handled();
                return;
            }
            _ => (),
        }
        if event.should_propagate_to_hidden() {
            for v in self.views.iter_mut().enumerate() {
                v.1.event(ctx, event, &mut data.editors[v.0], env);
            }
        } else {
            self.views[data.active_editor_index].event(ctx, event, &mut data.editors[data.active_editor_index], env);
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &NPViewSwitcherState,
        env: &druid::Env,
    ) {
        if event.should_propagate_to_hidden() {
            for v in self.views.iter_mut().enumerate() {
                v.1.lifecycle(ctx, event, &data.editors[v.0], env);
            }
        } else {
            self.views[data.active_editor_index].lifecycle(ctx, event, &data.editors[data.active_editor_index], env);
        }
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &NPViewSwitcherState,
        data: &NPViewSwitcherState,
        env: &druid::Env,
    ) {
        self.views[data.active_editor_index].update(
            ctx,
            &data.editors[data.active_editor_index],
            env,
        );
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &NPViewSwitcherState,
        env: &druid::Env,
    ) -> druid::Size {
        self.views[data.active_editor_index].layout(ctx, bc, &data.editors[data.active_editor_index], env)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &NPViewSwitcherState, env: &druid::Env) {
        self.views[data.active_editor_index].paint(ctx, &data.editors[data.active_editor_index], env);
    }
}

pub fn new() -> impl Widget<NPViewSwitcherState> {
    let mut v = Vec::new();
    v.push(WidgetPod::new(TextEditor::default()));
    NPViewSwitcher { views: v }
}
