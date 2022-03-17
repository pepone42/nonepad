use std::cmp::Ordering::Equal;
use std::sync::Arc;

use druid::im::Vector;
use druid::kurbo::Line;
use druid::piet::{Text, TextLayout, TextLayoutBuilder};
use druid::widget::{Flex, Padding, Scroll, TextBox};
use druid::{
    Color, Command, Data, Event, EventCtx, KbKey, KeyEvent, Lens, LifeCycle, Rect, RenderContext, Selector, Size,
    Target, UnitPoint, Widget, WidgetExt, WidgetId,
};

use sublime_fuzzy::best_match;

use crate::commands::{self, UICommandType};
use crate::theme::{SIDE_BAR_BACKGROUND, THEME};

use super::editor_view::{ScrollBar, ScrollBarDirection};
use super::Extension;

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

#[derive(Debug, Data, Lens, Clone, Default)]
pub struct PaletteListState {
    filter: String,
    selected_idx: usize,
    list: Vector<Item>,
    visible_list: Vector<(usize, Item)>,
    bbox: Rect,
}

impl PaletteListState {
    fn apply_filter(&mut self) {
        if self.filter.len() == 0 {
            for s in self.list.iter_mut() {
                s.filtered = false;
                s.score = 0;
            }
            self.visible_list = self.list.iter().enumerate().map(|i| (i.0, i.1.clone())).collect();
        } else {
            for s in self.list.iter_mut() {
                if let Some(m) = best_match(&self.filter, &s.title) {
                    s.filtered = false;
                    s.score = m.score();
                } else {
                    s.filtered = true;
                }
            }
            self.visible_list = self
                .list
                .iter()
                .enumerate()
                .filter(|c| !c.1.filtered)
                .map(|i| (i.0, i.1.clone()))
                .collect();
            self.visible_list.sort_by(|l, r| {
                let result = r.1.score.cmp(&l.1.score);
                if result == Equal {
                    l.1.title.cmp(&r.1.title)
                } else {
                    result
                }
            });
        }
    }

    fn prev(&mut self) {
        if self.selected_idx > 0 {
            self.selected_idx -= 1;
        }
    }
    fn next(&mut self) {
        if (self.selected_idx < self.visible_list.len() - 1) {
            self.selected_idx += 1;
        }
    }
}

pub struct Palette {
    inner: Flex<PaletteListState>,
    textbox_id: WidgetId,
    action: Option<UICommandType>,
    emmeter: Option<WidgetId>,
}

impl Palette {
    pub fn new() -> Self {
        let textbox_id = WidgetId::next();
        Palette {
            inner: build(textbox_id),
            textbox_id,
            action: None,
            emmeter: None,
        }
    }
    pub fn init(&mut self, data: &mut PaletteListState, list: Vector<Item>, action: UICommandType) {
        data.list = list.clone();
        data.selected_idx = 0;
        data.filter.clear();
        self.action = Some(action);
        data.visible_list = list.iter().enumerate().map(|i| (i.0, i.1.clone())).collect()
    }
    pub fn take_focus(&self, ctx: &mut EventCtx) {
        ctx.submit_command(Command::new(commands::GIVE_FOCUS, (), self.textbox_id));
    }
}

impl Widget<PaletteListState> for Palette {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut PaletteListState,
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
                    ctx.resign_focus();
                    ctx.submit_command(Command::new(commands::CLOSE_PALETTE, (), Target::Global));
                    if let Some(f) = self.action {
                        if let Some(item) = data.visible_list.get(data.selected_idx) {
                            ctx.submit_command(Command::new(
                                commands::PALETTE_CALLBACK,
                                (dbg!(item.0), item.1.title.clone(), f),
                                Target::Global,
                            ));
                        }
                    }

                    ctx.set_handled();
                }
                KeyEvent { key: KbKey::Escape, .. } => {
                    ctx.resign_focus();
                    ctx.submit_command(Command::new(commands::CLOSE_PALETTE, (), Target::Global));
                    ctx.set_handled();
                }
                _ => (),
            },
            Event::Command(c) if c.is(FILTER) => {
                dbg!(&data.filter);
                data.apply_filter();
                data.selected_idx = 0;
                ctx.request_paint();
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
        data: &PaletteListState,
        env: &druid::Env,
    ) {
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &PaletteListState,
        data: &PaletteListState,
        env: &druid::Env,
    ) {
        self.inner.update(ctx, &old_data, data, env);

        if old_data.selected_idx != data.selected_idx || !old_data.filter.same(&data.filter) {
            ctx.request_paint();
        }
        if !old_data.filter.same(&data.filter) {
            ctx.submit_command(Command::new(FILTER, (), ctx.widget_id()))
        }
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &PaletteListState,
        env: &druid::Env,
    ) -> druid::Size {
        self.inner.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &PaletteListState, env: &druid::Env) {
        self.inner.paint(ctx, data, env);
    }
}

struct PaletteList;

impl Widget<PaletteListState> for PaletteList {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &Event, data: &mut PaletteListState, env: &druid::Env) {}

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &PaletteListState,
        env: &druid::Env,
    ) {
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &PaletteListState,
        data: &PaletteListState,
        env: &druid::Env,
    ) {
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &PaletteListState,
        env: &druid::Env,
    ) -> Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &PaletteListState, env: &druid::Env) {
        let size = ctx.size();
        ctx.clip(Rect::ZERO.with_size(size));
        let mut dy = 2.5;
        for (i, item) in data.visible_list.iter().enumerate() {
            let layout = ctx
                .render_ctx
                .text()
                //.new_text_layout(format!("{} {}", item.1.title.clone(), item.1.score))
                .new_text_layout(item.1.title.clone())
                .font(env.get(druid::theme::UI_FONT).family, 14.0)
                .text_color(env.get(druid::theme::TEXT_COLOR))
                .alignment(druid::TextAlignment::Start).max_width(500.)
                .build()
                .unwrap();
            if i == data.selected_idx {
                ctx.render_ctx.fill(
                    Rect::new(2.5, dy, size.width - 4.5, dy + layout.size().height+ 4.5),
                    &env.get(crate::theme::SIDE_BAR_SECTION_HEADER_BACKGROUND)
                );
            }
            ctx.render_ctx.draw_text(&layout, (25.5, dy));
            dy += layout.size().height + 2.;
        }
    }
}

struct EmptyWidget;
impl<T> Widget<T> for EmptyWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &druid::Env) {
        match event {
            Event::MouseDown(_) => {
                ctx.submit_command(Command::new(commands::CLOSE_PALETTE, (), Target::Global));
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, ctx: &mut druid::LifeCycleCtx, event: &LifeCycle, data: &T, env: &druid::Env) {}

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &T, data: &T, env: &druid::Env) {}

    fn layout(&mut self, ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, data: &T, env: &druid::Env) -> Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &T, env: &druid::Env) {}
}

fn build(id: WidgetId) -> Flex<PaletteListState> {
    Flex::row()
        .with_flex_child(EmptyWidget, 0.5)
        .with_child(
            Flex::column()
                .with_child(
                    Padding::new(
                        2.,
                        Flex::column()
                            .with_child(
                                TextBox::new()
                                    .with_text_size(12.0)
                                    .focus()
                                    .fix_width(550.)
                                    .with_id(id)
                                    .lens(PaletteListState::filter),
                            )
                            .with_child(PaletteList.fix_size(550., 500.)),
                    )
                    .background(Color::from_hex_str(&THEME.vscode.colors.side_bar_background).unwrap())
                    .rounded(4.),
                )
                .with_flex_child(EmptyWidget, 1.)
                .fix_width(550.),
        )
        .with_flex_child(EmptyWidget, 0.5)
}
