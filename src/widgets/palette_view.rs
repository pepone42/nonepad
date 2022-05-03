use std::cmp::Ordering::Equal;
use std::rc::Rc;
use std::sync::Arc;

use druid::im::Vector;
use druid::piet::{Text, TextLayout, TextLayoutBuilder};
use druid::widget::{Flex, Label, Padding, TextBox};
use druid::{
    Affine, Color, Data, Env, Event, EventCtx, KbKey, KeyEvent, Lens, LifeCycle, Point, Rect, RenderContext,
    Selector, Size, Widget, WidgetExt, WidgetId, WidgetPod,
};

use sublime_fuzzy::best_match;

use crate::theme::THEME;

use super::editor_view::EditorView;
use super::text_buffer::EditStack;
use super::window::{NPWindow, NPWindowState};

const FILTER: Selector<()> = Selector::new("nonepad.editor.palette.filter");

#[derive(Debug, Data, Clone, Default)]
pub struct Item {
    title: Arc<String>,
    description: Arc<String>,
    filtered: bool,
    score: isize,
}

impl Item {
    pub fn new(title: &str, description: &str) -> Self {
        Self {
            title: Arc::new(title.into()),
            description: Arc::new(description.into()),
            filtered: false,
            score: 0,
        }
    }
}

macro_rules! item {
    ($($n : expr), + $(,) ?) => {{
        let mut v = Vector::new();
        $(v.push_back(Item::new($n,"") );)+
        v
    }};
}
pub(crate) use item;

#[derive(Debug, Data, Lens, Clone, Default)]
pub struct PaletteViewState {
    title: String,
    filter: String,
    selected_idx: usize,
    list: Option<Vector<Item>>,
    visible_list: Option<Vector<(usize, Item)>>,
    bbox: Rect,
}

impl PaletteViewState {
    fn apply_filter(&mut self) {
        if let Some(l) = &mut self.list {
            if self.filter.len() == 0 {
                for s in l.iter_mut() {
                    s.filtered = false;
                    s.score = 0;
                }
                self.visible_list = Some(l.iter().enumerate().map(|i| (i.0, i.1.clone())).collect());
            } else {
                for s in l.iter_mut() {
                    if let Some(m) = best_match(&self.filter, &s.title) {
                        s.filtered = false;
                        s.score = m.score();
                    } else {
                        s.filtered = true;
                    }
                }
                let mut vl: Vector<(usize, Item)> = l
                    .iter()
                    .enumerate()
                    .filter(|c| !c.1.filtered)
                    .map(|i| (i.0, i.1.clone()))
                    .collect();
                vl.sort_by(|l, r| {
                    let result = r.1.score.cmp(&l.1.score);
                    if result == Equal {
                        l.1.title.cmp(&r.1.title)
                    } else {
                        result
                    }
                });
                self.visible_list = Some(vl);
            }
        }
    }

    fn prev(&mut self) {
        if self.selected_idx > 0 {
            self.selected_idx -= 1;
        }
    }
    fn next(&mut self) {
        if let Some(l) = &self.visible_list {
            if self.selected_idx < l.len() - 1 {
                self.selected_idx += 1;
            }
        }
    }
}

pub struct PaletteView {
    inner: WidgetPod<PaletteViewState, Flex<PaletteViewState>>,
    textbox_id: WidgetId,
    action: Option<PaletteCommandType>,
}

impl PaletteView {
    pub(super) fn new() -> Self {
        let textbox_id = WidgetId::next();
        PaletteView {
            inner: WidgetPod::new(build(textbox_id)),
            textbox_id,
            action: None,
        }
    }
    pub(super) fn init(
        &mut self,
        data: &mut PaletteViewState,
        title: String,
        list: Option<Vector<Item>>,
        action: Option<PaletteCommandType>,
    ) {
        data.list = list.clone();
        data.title = title.to_owned();
        data.selected_idx = 0;
        data.filter.clear();
        self.action = action;
        data.visible_list = list.map(|l| l.iter().enumerate().map(|i| (i.0, i.1.clone())).collect())
    }
    pub fn take_focus(&self, ctx: &mut EventCtx) {
        ctx.set_focus(self.textbox_id);
    }
}

impl Widget<PaletteViewState> for PaletteView {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut PaletteViewState,
        env: &druid::Env,
    ) {
        match event {
            Event::KeyDown(k) => match k {
                KeyEvent {
                    key: druid::keyboard_types::Key::ArrowUp,
                    ..
                } => {
                    data.prev();
                    ctx.set_handled();
                }
                KeyEvent {
                    key: druid::keyboard_types::Key::ArrowDown,
                    ..
                } => {
                    data.next();
                    ctx.set_handled();
                }
                KeyEvent { key: KbKey::Enter, .. } => {
                    ctx.submit_command(CLOSE_PALETTE);

                    if let Some(f) = self.action.take() {
                        match &data.visible_list {
                            Some(l) => {
                                if let Some(item) = l.get(data.selected_idx) {
                                    ctx.submit_command(
                                        PALETTE_CALLBACK.with(
                                        (
                                            PaletteResult {
                                                index: item.0,
                                                name: item.1.title.clone(),
                                            },
                                            f,
                                        )));
                                }
                            }
                            None => {
                                ctx.submit_command(
                                    PALETTE_CALLBACK.with(
                                    (
                                        PaletteResult {
                                            index: 0,
                                            name: Arc::new(data.filter.clone()),
                                        },
                                        f,
                                    )));
                            }
                        }
                    }

                    ctx.set_handled();
                }
                KeyEvent { key: KbKey::Escape, .. } => {
                    ctx.submit_command(CLOSE_PALETTE);
                    ctx.set_handled();
                }
                _ => {
                    self.inner.event(ctx, event, data, env);
                }
            },
            Event::Command(c) if c.is(FILTER) => {
                data.apply_filter();
                data.selected_idx = 0;
                ctx.request_paint();
                ctx.set_handled();
            }
            _ => {
                self.inner.event(ctx, event, data, env);
            }
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &PaletteViewState,
        env: &druid::Env,
    ) {
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &PaletteViewState,
        data: &PaletteViewState,
        env: &druid::Env,
    ) {
        self.inner.update(ctx, data, env);

        if old_data.selected_idx != data.selected_idx || !old_data.filter.same(&data.filter) {
            ctx.request_paint();
        }
        if !old_data.filter.same(&data.filter) {
            ctx.submit_command(FILTER.to(ctx.widget_id()))
        }
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &PaletteViewState,
        env: &druid::Env,
    ) -> druid::Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &PaletteViewState, env: &druid::Env) {
        self.inner.paint(ctx, data, env);
    }
}

#[derive(Default)]
struct PaletteList {
    total_height: f64,
    position: Point,
}

impl Widget<PaletteViewState> for PaletteList {
    fn event(&mut self, _ctx: &mut druid::EventCtx, _event: &Event, _data: &mut PaletteViewState, _env: &druid::Env) {}

    fn lifecycle(
        &mut self,
        _ctx: &mut druid::LifeCycleCtx,
        _event: &druid::LifeCycle,
        _data: &PaletteViewState,
        _env: &druid::Env,
    ) {
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &PaletteViewState,
        data: &PaletteViewState,
        _env: &druid::Env,
    ) {
        if !old_data.list.same(&data.list) {
            ctx.request_layout();
        }
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &PaletteViewState,
        env: &druid::Env,
    ) -> Size {
        let mut dy = 2.5;
        if let Some(l) = &data.visible_list {
            for item in l.iter() {
                let layout = ctx
                    .text()
                    //.new_text_layout(format!("{} {}", item.1.title.clone(), item.1.score))
                    .new_text_layout(item.1.title.clone())
                    .font(env.get(druid::theme::UI_FONT).family, 14.0)
                    .text_color(env.get(druid::theme::TEXT_COLOR))
                    .alignment(druid::TextAlignment::Start)
                    .max_width(500.)
                    .build()
                    .unwrap();
                dy += layout.size().height + 2.;
            }
        }
        self.total_height = dy;
        Size::new(bc.max().width, self.total_height.min(500.))
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &PaletteViewState, env: &druid::Env) {
        if data.visible_list.is_some() {
            let size = ctx.size();
            ctx.clip(Rect::ZERO.with_size(size));
            let mut dy = 2.5;

            let mut layouts = Vec::new();
            let mut selection_rect = Rect::ZERO;

            for (i, item) in data.visible_list.clone().unwrap().iter().enumerate() {
                let layout = ctx
                    .text()
                    //.new_text_layout(format!("{} {}", item.1.title.clone(), item.1.score))
                    .new_text_layout(item.1.title.clone())
                    .font(env.get(druid::theme::UI_FONT).family, 14.0)
                    .text_color(env.get(druid::theme::TEXT_COLOR))
                    .alignment(druid::TextAlignment::Start)
                    .max_width(500.)
                    .build()
                    .unwrap();
                let height = layout.size().height;
                layouts.push((dy, layout));
                if i == data.selected_idx {
                    selection_rect = Rect::new(2.5, dy, size.width - 4.5, dy + height + 4.5);
                }

                dy += height + 2.;
            }

            if selection_rect.y0 < self.position.y {
                self.position.y = selection_rect.y0
            }
            if selection_rect.y1 > self.position.y + size.height {
                self.position.y = selection_rect.y1 - size.height
            }

            ctx.with_save(|ctx| {
                ctx.transform(Affine::translate((-self.position.x, -self.position.y)));

                ctx.fill(
                    selection_rect,
                    &env.get(crate::theme::SIDE_BAR_SECTION_HEADER_BACKGROUND),
                );
                for l in layouts {
                    ctx.draw_text(&l.1, (25.5, l.0));
                }
            });
        }
    }
}

// const SCROLLABLE_MOVE_VIEWPORT: Selector<Point> = Selector::new("nonepad.scrollable.move_viewport");

// pub trait Scrollable {
//     fn update_viewport(&mut self, rect: Rect);

// }

// pub trait ViewportMover {
//     fn move_viewport(&mut self, point: Point);
// }

// impl ViewportMover for EventCtx<'_, '_> {
//     fn move_viewport(&mut self, point: Point) {
//         self.submit_notification(SCROLLABLE_MOVE_VIEWPORT.with(point));
//     }
// }

// struct Scroller<T, W> {
//     inner: WidgetPod<T, W>
// }
// impl <T,W: Scrollable> Widget<T> for Scroller<T, W> {
//     fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &druid::Env) {
//         match event {
//             Event::Notification(notif) => if notif.is(SCROLLABLE_MOVE_VIEWPORT) {
//                 self.inner.widget_mut().update_viewport(Rect::ZERO);
//                 todo!()
//             }
//             _ => (),
//         }

//     }

//     fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &LifeCycle, data: &T, env: &druid::Env) {

//     }

//     fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &T, data: &T, env: &druid::Env) {

//     }

//     fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &T, env: &druid::Env) -> Size {
//         let r= self.inner.widget_mut().layout(ctx,bc,data,env);
//         self.inner.widget_mut().update_viewport(Rect::ZERO);
//         bc.max()
//     }

//     fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &T, env: &druid::Env) {

//     }
// }

struct EmptyWidget;
impl<T> Widget<T> for EmptyWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut T, _env: &druid::Env) {
        match event {
            Event::MouseDown(_) => {
                ctx.submit_command(CLOSE_PALETTE);
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, _ctx: &mut druid::LifeCycleCtx, _event: &LifeCycle, _data: &T, _env: &druid::Env) {}

    fn update(&mut self, _ctx: &mut druid::UpdateCtx, _old_data: &T, _data: &T, _env: &druid::Env) {}

    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        _data: &T,
        _env: &druid::Env,
    ) -> Size {
        bc.max()
    }

    fn paint(&mut self, _ctx: &mut druid::PaintCtx, _data: &T, _env: &druid::Env) {}
}

fn build(id: WidgetId) -> Flex<PaletteViewState> {
    Flex::row()
        .with_flex_child(EmptyWidget, 0.5)
        .with_child(
            Flex::column()
                .with_child(
                    Padding::new(
                        2.,
                        Flex::column()
                            .with_child(
                                Label::new(|data: &PaletteViewState, _env: &Env| format!("{}", data.title))
                                    .with_text_size(12.0),
                            )
                            .with_child(
                                TextBox::new()
                                    .with_text_size(12.0)
                                    .fix_width(550.)
                                    .with_id(id)
                                    .lens(PaletteViewState::filter),
                            )
                            .with_child(PaletteList::default())
                            .cross_axis_alignment(druid::widget::CrossAxisAlignment::Start),
                    )
                    .background(Color::from_hex_str(&THEME.vscode.colors.side_bar_background).unwrap())
                    .rounded(4.),
                )
                .with_flex_child(EmptyWidget, 1.)
                .fix_width(550.),
        )
        .with_flex_child(EmptyWidget, 0.5)
}

#[derive(Clone)]
pub(super) enum PaletteCommandType {
    Editor(Rc<dyn Fn(PaletteResult, &mut EventCtx, &mut EditorView, &mut EditStack)>),
    Window(Rc<dyn Fn(PaletteResult, &mut EventCtx, &mut NPWindow, &mut NPWindowState)>),
    DialogEditor(Rc<dyn Fn(DialogResult, &mut EventCtx, &mut EditorView, &mut EditStack)>),
    DialogWindow(Rc<dyn Fn(DialogResult, &mut EventCtx, &mut NPWindow, &mut NPWindowState)>),
}

pub(super) const SHOW_PALETTE_FOR_EDITOR: Selector<(
    WidgetId,
    String,
    Option<Vector<Item>>,
    Option<Rc<dyn Fn(PaletteResult, &mut EventCtx, &mut EditorView, &mut EditStack)>>,
)> = Selector::new("nonepad.palette.show_for_editor");
pub(super) const SHOW_PALETTE_FOR_WINDOW: Selector<(
    WidgetId,
    String,
    Option<Vector<Item>>,
    Option<Rc<dyn Fn(PaletteResult, &mut EventCtx, &mut NPWindow, &mut NPWindowState)>>,
)> = Selector::new("nonepad.palette.show_for_window");
pub(super) const SHOW_DIALOG_FOR_EDITOR: Selector<(
    WidgetId,
    String,
    Option<Vector<Item>>,
    Option<Rc<dyn Fn(DialogResult, &mut EventCtx, &mut EditorView, &mut EditStack)>>,
)> = Selector::new("nonepad.dialog.show_for_editor");
pub(super) const SHOW_DIALOG_FOR_WINDOW: Selector<(
    WidgetId,
    String,
    Option<Vector<Item>>,
    Option<Rc<dyn Fn(DialogResult, &mut EventCtx, &mut NPWindow, &mut NPWindowState)>>,
)> = Selector::new("nonepad.dialog.show_for_window");

pub(super) const PALETTE_CALLBACK: Selector<(PaletteResult, PaletteCommandType)> =
    Selector::new("nonepad.editor.execute_command");
pub(super) const CLOSE_PALETTE: Selector<()> = Selector::new("nonepad.palette.close");

trait ShowPalette<R, W, D> {
    fn show_palette(
        &mut self,
        title: String,
        items: Option<Vector<Item>>,
        callback: Option<Rc<dyn Fn(R, &mut EventCtx, &mut W, &mut D)>>,
    );
}

impl<'a, 'b, 'c> ShowPalette<PaletteResult, EditorView, EditStack> for EventCtx<'b, 'c> {
    fn show_palette(
        &mut self,
        title: String,
        items: Option<Vector<Item>>,
        callback: Option<Rc<dyn Fn(PaletteResult, &mut EventCtx, &mut EditorView, &mut EditStack)>>,
    ) {
        self.submit_command(SHOW_PALETTE_FOR_EDITOR.with((self.widget_id(), title, items, callback)));
    }
}

impl<'a, 'b, 'c> ShowPalette<PaletteResult, NPWindow, NPWindowState> for EventCtx<'b, 'c> {
    fn show_palette(
        &mut self,
        title: String,
        items: Option<Vector<Item>>,
        callback: Option<Rc<dyn Fn(PaletteResult, &mut EventCtx, &mut NPWindow, &mut NPWindowState)>>,
    ) {
        self.submit_command(SHOW_PALETTE_FOR_WINDOW.with((self.widget_id(), title, items, callback)));
    }
}

impl<'a, 'b, 'c> ShowPalette<DialogResult, EditorView, EditStack> for EventCtx<'b, 'c> {
    fn show_palette(
        &mut self,
        title: String,
        items: Option<Vector<Item>>,
        callback: Option<Rc<dyn Fn(DialogResult, &mut EventCtx, &mut EditorView, &mut EditStack)>>,
    ) {
        self.submit_command(SHOW_DIALOG_FOR_EDITOR.with((self.widget_id(), title, items, callback)));
    }
}

impl<'a, 'b, 'c> ShowPalette<DialogResult, NPWindow, NPWindowState> for EventCtx<'b, 'c> {
    fn show_palette(
        &mut self,
        title: String,
        items: Option<Vector<Item>>,
        callback: Option<Rc<dyn Fn(DialogResult, &mut EventCtx, &mut NPWindow, &mut NPWindowState)>>,
    ) {
        self.submit_command(SHOW_DIALOG_FOR_WINDOW.with((self.widget_id(), title, items, callback)));
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DialogResult {
    Ok,
    Cancel,
}
#[derive(Debug, Clone)]
pub struct PaletteResult {
    pub index: usize,
    pub name: Arc<String>,
}

pub struct Palette<R, W, D> {
    title: Option<String>,
    action: Option<Rc<dyn Fn(R, &mut EventCtx, &mut W, &mut D)>>,
    items: Option<Vector<Item>>,
}

impl<R, W, D> Default for Palette<R, W, D> {
    fn default() -> Self {
        Self {
            title: Default::default(),
            action: Default::default(),
            items: Default::default(),
        }
    }
}

impl<R, W, D> Palette<R, W, D> {
    fn new() -> Self {
        Palette::default()
    }
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_owned());
        self
    }
    pub fn on_select(mut self, action: impl Fn(R, &mut EventCtx, &mut W, &mut D) + 'static) -> Self {
        self.action = Some(Rc::new(action));
        self
    }
    pub fn items(mut self, items: Vector<Item>) -> Self {
        self.items = Some(items);
        self
    }
}

pub trait PaletteBuilder<D> {
    fn palette(&self) -> Palette<PaletteResult, Self, D>
    where
        Self: Sized,
    {
        Palette::<PaletteResult, Self, D>::new()
    }
    fn dialog(&self) -> Palette<DialogResult, Self, D>
    where
        Self: Sized,
    {
        Palette::<DialogResult, Self, D>::new().items(item!["Ok", "Cancel"])
    }
    fn alert(&self, title: &str) -> Palette<PaletteResult, Self, D>
    where
        Self: Sized,
    {
        Palette::new().title(title).items(item!["Ok"])
    }
}

impl PaletteBuilder<EditStack> for EditorView {}

impl PaletteBuilder<NPWindowState> for NPWindow {}

impl Palette<PaletteResult, EditorView, EditStack> {
    pub fn show(self, ctx: &mut EventCtx) {
        ctx.show_palette(self.title.unwrap_or_default(), self.items, self.action);
    }
}

impl Palette<PaletteResult, NPWindow, NPWindowState> {
    pub fn show(self, ctx: &mut EventCtx) {
        ctx.show_palette(self.title.unwrap_or_default(), self.items, self.action);
    }
}

impl Palette<DialogResult, EditorView, EditStack> {
    pub fn show(self, ctx: &mut EventCtx) {
        ctx.show_palette(self.title.unwrap_or_default(), self.items, self.action);
    }
}

impl Palette<DialogResult, NPWindow, NPWindowState> {
    pub fn show(self, ctx: &mut EventCtx) {
        ctx.show_palette(self.title.unwrap_or_default(), self.items, self.action);
    }
}
