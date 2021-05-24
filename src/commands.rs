use druid::Selector;

pub const SHOW_SEARCH_PANEL: Selector<()> = Selector::new("nonepad.bottom_panel.show_search");
pub const CLOSE_BOTTOM_PANEL: Selector<()> = Selector::new("nonepad.bottom_panel.close");
pub const REQUEST_NEXT_SEARCH: Selector<String> = Selector::new("nonepad.editor.request_next_search");
pub const GIVE_FOCUS: Selector<()> = Selector::new("nonepad.all.give_focus");
pub const SCROLL_VIEWPORT: Selector<f64> = Selector::new("nonepad.editor.scroll");
pub const SELECT_LINE: Selector<(usize,bool)> = Selector::new("nonepad.editor.select_line");
