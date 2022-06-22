use std::{path::Path, sync::atomic::{AtomicU64, Ordering, AtomicUsize}};

use super::{
    editor_view::{EditorView, TextEditor},
    text_buffer::EditStack,
};
use druid::{
    im::{vector, Vector, self, hashmap},
    Data, Event, Lens, Selector, Widget, WidgetPod,
};

struct Counter(AtomicUsize);

impl Counter {
    pub const fn new() -> Counter {
        Counter(AtomicUsize::new(1))
    }

    pub fn next(&self) -> usize {
        self.0.fetch_add(1, Ordering::Relaxed)
    }
}

#[derive(Debug,Default,Data,Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ViewId(usize);

impl ViewId {
    pub fn new(i: usize) -> Self {
        ViewId(i)
    }
    pub fn next() -> ViewId {
        static VIEW_ID_COUNTER: Counter = Counter::new();
        dbg!(ViewId(VIEW_ID_COUNTER.next()))
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

#[derive(Clone, Data, Lens)]
pub struct NPViewSwitcherState {
    pub editors: im::HashMap<ViewId, EditStack>,
    active_editor_index: ViewId,
}

impl Default for NPViewSwitcherState {
    fn default() -> Self {
        Self {
            editors: hashmap![ViewId::default() => EditStack::default()],
            active_editor_index: dbg!(Default::default()),
        }
    }

}

impl NPViewSwitcherState {
    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<NPViewSwitcherState> {
        Ok(Self {
            editors: hashmap![ViewId::default() => EditStack::from_file(&path)?],
            active_editor_index: Default::default(),
        })
    }
    pub fn active_editor(&self) -> &EditStack {
        &self.editors[&self.active_editor_index]
    }
    pub fn active_editor_mut(&mut self) -> &mut EditStack {
        &mut self.editors[&self.active_editor_index]
    }
    pub fn select_view(&mut self, vid: ViewId) {
        self.active_editor_index = vid;
    }
}

pub struct NPViewSwitcher {
    views: std::collections::HashMap<ViewId, WidgetPod<EditStack, TextEditor>>,
}

pub const NEW_EDITVIEW: Selector<()> = Selector::new("nonepad.viewswitcher.new_editview");
pub(super) const ACTIVATE_EDITVIEW: Selector<()> = Selector::new("nonepad.viewswitcher.activate_editview");

impl NPViewSwitcher {
    pub fn new() -> Self {
        NPViewSwitcher { views: std::collections::HashMap::new() }
    }
    pub fn new_view(&mut self,data: &mut NPViewSwitcherState) {
        let id = ViewId::next();
        self.views.insert(id, WidgetPod::new(TextEditor::default()));
        data.editors.insert(id ,EditStack::new());
        data.active_editor_index = id;
    }

    fn active_view(&self, data:&NPViewSwitcherState) -> &TextEditor {
        self.views[&data.active_editor_index].widget()
    }
    fn active_view_mut(&mut self, data:&NPViewSwitcherState) -> &mut TextEditor {
        let id=data.active_editor_index;
        self.views.get_mut(&id).unwrap().widget_mut()
    }
    fn active_pod(&self, data:&NPViewSwitcherState) -> &WidgetPod<EditStack, TextEditor> {
        &self.views[&data.active_editor_index]
    }
    fn active_pod_mut(&mut self, data:&NPViewSwitcherState) -> &mut WidgetPod<EditStack, TextEditor> {
        let id=data.active_editor_index;
        self.views.get_mut(&id).unwrap()
    }
}

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
                self.new_view(data);

                ctx.children_changed();

                ctx.submit_command(ACTIVATE_EDITVIEW.to(self.views[&data.active_editor_index].id()));
                ctx.set_handled();
                return;
            }
            Event::Command(cmd) if cmd.is(druid::commands::OPEN_FILE) => {
                
                self.new_view(data);
                ctx.children_changed();

                ctx.submit_command(ACTIVATE_EDITVIEW.to(self.views[&data.active_editor_index].id()));

                if let Some(file_info) = cmd.get(druid::commands::OPEN_FILE) {
                    ctx.submit_command(druid::commands::OPEN_FILE.with(file_info.clone()).to(self.views[&data.active_editor_index].id()));
                    // if let Err(_) = self.views..open(editor, file_info.path()) {
                    //     self.alert("Error loading file").show(ctx);
                    // }
                }
                ctx.set_handled();
                return;
            }
            _ => (),
        }
        if event.should_propagate_to_hidden() {
            for v in self.views.iter_mut() {
                v.1.event(ctx, event, &mut data.editors[v.0], env);
            }
        } else {
            self.active_pod_mut(data).event(ctx, event, data.active_editor_mut(), env);
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
            for v in self.views.iter_mut() {
                v.1.lifecycle(ctx, event, &data.editors[v.0], env);
            }
        } else {
            self.active_pod_mut(data).lifecycle(ctx, event, data.active_editor(), env);
        }
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &NPViewSwitcherState,
        data: &NPViewSwitcherState,
        env: &druid::Env,
    ) {
        self.active_pod_mut(data).update(
            ctx,
            &data.editors[&data.active_editor_index],
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
        self.active_pod_mut(data).layout(ctx, bc, &data.active_editor(), env)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &NPViewSwitcherState, env: &druid::Env) {
        self.active_pod_mut(data).paint(ctx, &data.active_editor(), env);
    }
}

pub fn new() -> impl Widget<NPViewSwitcherState> {
    let mut v = std::collections::HashMap::new();
    v.insert(ViewId::default(), WidgetPod::new( TextEditor::default()));
    NPViewSwitcher { views: v }
}
