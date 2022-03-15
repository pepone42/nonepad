use std::sync::Arc;

use druid::im::Vector;
use druid::kurbo::Line;
use druid::piet::{Text, TextLayout, TextLayoutBuilder};
use druid::widget::TextBox;
use druid::{Command, Data, Event, KbKey, KeyEvent, RenderContext, Selector, Size, Target, Widget, WidgetId};

use sublime_fuzzy::best_match;

use crate::commands::{self, UICommandType};

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

#[derive(Debug, Data, Clone, Default)]
pub struct PaletteListState {
    filter: String,
    selected_idx: usize,
    list: Vector<Item>,
}

impl PaletteListState {
    pub fn new(list: Vector<Item>) -> Self {
        PaletteListState {
            filter: "".into(),
            selected_idx: 0,
            list: list,
        }
    }

    fn filter(&mut self) {
        if self.filter.len() == 0 {
            for s in self.list.iter_mut() {
                s.filtered = false;
                s.score = 0;
            }
        } else {
            for s in self.list.iter_mut() {
                if let Some(m) = best_match(&self.filter, &s.title) {
                    s.filtered = false;
                    s.score = m.score();
                } else {
                    s.filtered = true;
                }
            }
        }
    }

    fn prev(&mut self) {
        if let Some((i,_)) = dbg!(self.list.iter().filter(|f| !f.filtered).enumerate().nth(self.selected_idx-1)) {
            self.selected_idx = i;
        }
    }
    fn next(&mut self) {
        if let Some((i,_)) = dbg!(self.list.iter().filter(|f| !f.filtered).enumerate().nth(self.selected_idx+1)) {
            self.selected_idx = i;
        }
    }
}

pub struct PaletteList {
    search: TextBox<String>,
    filter_height: f64,
    action: Option<UICommandType>,
    emmeter: Option<WidgetId,>
}

impl PaletteList {
    pub fn new() -> Self {
        PaletteList {
            search: TextBox::new().with_text_size(12.0),
            filter_height: 0.,
            action: None,
            emmeter: None,
        }
    }
}

impl Widget<PaletteListState> for PaletteList {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut PaletteListState,
        env: &druid::Env,
    ) {
        match event {
            Event::Command(cmd) if cmd.is(crate::commands::SEND_PALETTE_PANEL_DATA) => {
                let d = cmd.get_unchecked(crate::commands::SEND_PALETTE_PANEL_DATA);
                data.list = d.1.clone();
                data.selected_idx = 0;
                data.filter.clear();
                self.action = Some(d.2);
                self.emmeter = Some(d.0);
            }
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
                    if let Some(f) = self.action {
                        let index = data.list.iter().enumerate().filter(|i| !i.1.filtered).nth(data.selected_idx).unwrap().0;
                        ctx.submit_command(Command::new(commands::PALETTE_CALLBACK, (index, data.list[index].title.clone(),f), Target::Global));
                    }
                    ctx.submit_command(Command::new(commands::CLOSE_BOTTOM_PANEL, (), Target::Global));
                    ctx.set_handled();
                }
                _ => ()
            },
            Event::Command(c) if c.is(FILTER) => {
                dbg!(&data.filter);
                data.filter();
                data.selected_idx = 0;
                ctx.request_paint();
            }
            _ => self.search.event(ctx, event, &mut data.filter, env),
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &PaletteListState,
        env: &druid::Env,
    ) {
        self.search.lifecycle(ctx, event, &data.filter, env);
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &PaletteListState,
        data: &PaletteListState,
        env: &druid::Env,
    ) {
        self.search.update(ctx, &old_data.filter, &data.filter, env);

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
        let s = self.search.layout(ctx, bc, &data.filter, env);
        self.filter_height = s.height;
        Size::new(bc.max().width, self.filter_height + 100.)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &PaletteListState, env: &druid::Env) {
        self.search.paint(ctx, &data.filter, env);

        let mut dy = self.filter_height + 2.;
        for (i, item) in data.list.iter().filter(|c| !c.filtered).enumerate() {
            let layout = ctx
                .render_ctx
                .text()
                .new_text_layout(item.title.clone())
                //.font(FontFamily::MONOSPACE, 12.0)
                .font(
                    env.get(druid::theme::UI_FONT).family,
                    env.get(druid::theme::TEXT_SIZE_NORMAL),
                )
                .text_color(env.get(druid::theme::TEXT_COLOR))
                .build()
                .unwrap();
            ctx.render_ctx.draw_text(&layout, (5.0, dy));
            if i == data.selected_idx {
                ctx.render_ctx.stroke(
                    Line::new((2., dy), (2., dy + layout.size().height)),
                    &env.get(crate::theme::EDITOR_CURSOR_FOREGROUND),
                    2.0,
                );
            }
            dy += layout.size().height;
        }
        // ctx.render_ctx.stroke(
        //     Line::new(
        //         (0., 0.),
        //         (50.,50.),
        //     ),
        //     &env.get(crate::theme::EDITOR_CURSOR_FOREGROUND),
        //     2.0,
        // );
    }
}
