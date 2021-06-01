use druid::{Rect, Selector, Vec2};

use crate::editor_view::CommonMetrics;

pub const SHOW_SEARCH_PANEL: Selector<()> = Selector::new("nonepad.bottom_panel.show_search");
pub const CLOSE_BOTTOM_PANEL: Selector<()> = Selector::new("nonepad.bottom_panel.close");
pub const REQUEST_NEXT_SEARCH: Selector<String> = Selector::new("nonepad.editor.request_next_search");
pub const GIVE_FOCUS: Selector<()> = Selector::new("nonepad.all.give_focus");
pub const SELECT_LINE: Selector<(usize,bool)> = Selector::new("nonepad.editor.select_line");
pub const SCROLL_TO: Selector<(Option<f64>,Option<f64>)> = Selector::new("nonepad.editor.scroll_to_rect");
pub const NEW_METRICS: Selector<CommonMetrics> = Selector::new("nonepad.editor.new_metrics");