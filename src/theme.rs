#[rustfmt::skip]

use druid::{Color, Env, Key};
use miniserde::{Deserialize, Serialize, json};
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Colors {
    #[serde(rename = "focusBorder")]
    pub focus_border : String,
    #[serde(rename = "foreground")]
    pub foreground : String,
    #[serde(rename = "selection.background")]
    pub selection_background : String,
    #[serde(rename = "widget.shadow")]
    pub widget_shadow : String,
    #[serde(rename = "textLink.activeForeground")]
    pub text_link_active_foreground : String,
    #[serde(rename = "textLink.foreground")]
    pub text_link_foreground : String,
    #[serde(rename = "textPreformat.foreground")]
    pub text_preformat_foreground : String,
    #[serde(rename = "button.background")]
    pub button_background : String,
    #[serde(rename = "button.foreground")]
    pub button_foreground : String,
    #[serde(rename = "button.hoverBackground")]
    pub button_hover_background : String,
    #[serde(rename = "dropdown.background")]
    pub dropdown_background : String,
    #[serde(rename = "dropdown.listBackground")]
    pub dropdown_list_background : String,
    #[serde(rename = "input.background")]
    pub input_background : String,
    #[serde(rename = "input.border")]
    pub input_border : String,
    #[serde(rename = "input.foreground")]
    pub input_foreground : String,
    #[serde(rename = "input.placeholderForeground")]
    pub input_placeholder_foreground : String,
    #[serde(rename = "scrollbar.shadow")]
    pub scrollbar_shadow : String,
    #[serde(rename = "scrollbarSlider.activeBackground")]
    pub scrollbar_slider_active_background : String,
    #[serde(rename = "scrollbarSlider.background")]
    pub scrollbar_slider_background : String,
    #[serde(rename = "scrollbarSlider.hoverBackground")]
    pub scrollbar_slider_hover_background : String,
    #[serde(rename = "badge.foreground")]
    pub badge_foreground : String,
    #[serde(rename = "badge.background")]
    pub badge_background : String,
    #[serde(rename = "progressBar.background")]
    pub progress_bar_background : String,
    #[serde(rename = "list.activeSelectionBackground")]
    pub list_active_selection_background : String,
    #[serde(rename = "list.activeSelectionForeground")]
    pub list_active_selection_foreground : String,
    #[serde(rename = "list.inactiveSelectionBackground")]
    pub list_inactive_selection_background : String,
    #[serde(rename = "list.inactiveSelectionForeground")]
    pub list_inactive_selection_foreground : String,
    #[serde(rename = "list.hoverForeground")]
    pub list_hover_foreground : String,
    #[serde(rename = "list.focusForeground")]
    pub list_focus_foreground : String,
    #[serde(rename = "list.focusBackground")]
    pub list_focus_background : String,
    #[serde(rename = "list.hoverBackground")]
    pub list_hover_background : String,
    #[serde(rename = "list.dropBackground")]
    pub list_drop_background : String,
    #[serde(rename = "list.highlightForeground")]
    pub list_highlight_foreground : String,
    #[serde(rename = "list.errorForeground")]
    pub list_error_foreground : String,
    #[serde(rename = "list.warningForeground")]
    pub list_warning_foreground : String,
    #[serde(rename = "activityBar.background")]
    pub activity_bar_background : String,
    #[serde(rename = "activityBar.dropBackground")]
    pub activity_bar_drop_background : String,
    #[serde(rename = "activityBar.foreground")]
    pub activity_bar_foreground : String,
    #[serde(rename = "activityBarBadge.background")]
    pub activity_bar_badge_background : String,
    #[serde(rename = "activityBarBadge.foreground")]
    pub activity_bar_badge_foreground : String,
    #[serde(rename = "sideBar.background")]
    pub side_bar_background : String,
    #[serde(rename = "sideBar.foreground")]
    pub side_bar_foreground : String,
    #[serde(rename = "sideBarSectionHeader.background")]
    pub side_bar_section_header_background : String,
    #[serde(rename = "sideBarSectionHeader.foreground")]
    pub side_bar_section_header_foreground : String,
    #[serde(rename = "sideBarTitle.foreground")]
    pub side_bar_title_foreground : String,
    #[serde(rename = "editorGroup.border")]
    pub editor_group_border : String,
    #[serde(rename = "editorGroup.dropBackground")]
    pub editor_group_drop_background : String,
    #[serde(rename = "editorGroupHeader.noTabsBackground")]
    pub editor_group_header_no_tabs_background : String,
    #[serde(rename = "editorGroupHeader.tabsBackground")]
    pub editor_group_header_tabs_background : String,
    #[serde(rename = "tab.activeBackground")]
    pub tab_active_background : String,
    #[serde(rename = "tab.activeForeground")]
    pub tab_active_foreground : String,
    #[serde(rename = "tab.border")]
    pub tab_border : String,
    #[serde(rename = "tab.activeBorder")]
    pub tab_active_border : String,
    #[serde(rename = "tab.unfocusedActiveBorder")]
    pub tab_unfocused_active_border : String,
    #[serde(rename = "tab.inactiveBackground")]
    pub tab_inactive_background : String,
    #[serde(rename = "tab.inactiveForeground")]
    pub tab_inactive_foreground : String,
    #[serde(rename = "tab.unfocusedActiveForeground")]
    pub tab_unfocused_active_foreground : String,
    #[serde(rename = "tab.unfocusedInactiveForeground")]
    pub tab_unfocused_inactive_foreground : String,
    #[serde(rename = "editor.background")]
    pub editor_background : String,
    #[serde(rename = "editor.foreground")]
    pub editor_foreground : String,
    #[serde(rename = "editor.hoverHighlightBackground")]
    pub editor_hover_highlight_background : String,
    #[serde(rename = "editor.findMatchBackground")]
    pub editor_find_match_background : String,
    #[serde(rename = "editor.findMatchHighlightBackground")]
    pub editor_find_match_highlight_background : String,
    #[serde(rename = "editor.findRangeHighlightBackground")]
    pub editor_find_range_highlight_background : String,
    #[serde(rename = "editor.lineHighlightBackground")]
    pub editor_line_highlight_background : String,
    #[serde(rename = "editor.lineHighlightBorder")]
    pub editor_line_highlight_border : String,
    #[serde(rename = "editor.inactiveSelectionBackground")]
    pub editor_inactive_selection_background : String,
    #[serde(rename = "editor.selectionBackground")]
    pub editor_selection_background : String,
    #[serde(rename = "editor.selectionHighlightBackground")]
    pub editor_selection_highlight_background : String,
    #[serde(rename = "editor.rangeHighlightBackground")]
    pub editor_range_highlight_background : String,
    #[serde(rename = "editor.wordHighlightBackground")]
    pub editor_word_highlight_background : String,
    #[serde(rename = "editor.wordHighlightStrongBackground")]
    pub editor_word_highlight_strong_background : String,
    #[serde(rename = "editorError.foreground")]
    pub editor_error_foreground : String,
    #[serde(rename = "editorError.border")]
    pub editor_error_border : String,
    #[serde(rename = "editorWarning.foreground")]
    pub editor_warning_foreground : String,
    #[serde(rename = "editorInfo.foreground")]
    pub editor_info_foreground : String,
    #[serde(rename = "editorWarning.border")]
    pub editor_warning_border : String,
    #[serde(rename = "editorCursor.foreground")]
    pub editor_cursor_foreground : String,
    #[serde(rename = "editorIndentGuide.background")]
    pub editor_indent_guide_background : String,
    #[serde(rename = "editorLineNumber.foreground")]
    pub editor_line_number_foreground : String,
    #[serde(rename = "editorWhitespace.foreground")]
    pub editor_whitespace_foreground : String,
    #[serde(rename = "editorOverviewRuler.border")]
    pub editor_overview_ruler_border : String,
    #[serde(rename = "editorOverviewRuler.currentContentForeground")]
    pub editor_overview_ruler_current_content_foreground : String,
    #[serde(rename = "editorOverviewRuler.incomingContentForeground")]
    pub editor_overview_ruler_incoming_content_foreground : String,
    #[serde(rename = "editorOverviewRuler.findMatchForeground")]
    pub editor_overview_ruler_find_match_foreground : String,
    #[serde(rename = "editorOverviewRuler.rangeHighlightForeground")]
    pub editor_overview_ruler_range_highlight_foreground : String,
    #[serde(rename = "editorOverviewRuler.selectionHighlightForeground")]
    pub editor_overview_ruler_selection_highlight_foreground : String,
    #[serde(rename = "editorOverviewRuler.wordHighlightForeground")]
    pub editor_overview_ruler_word_highlight_foreground : String,
    #[serde(rename = "editorOverviewRuler.wordHighlightStrongForeground")]
    pub editor_overview_ruler_word_highlight_strong_foreground : String,
    #[serde(rename = "editorOverviewRuler.modifiedForeground")]
    pub editor_overview_ruler_modified_foregrund : String,
    #[serde(rename = "editorOverviewRuler.addedForeground")]
    pub editor_overview_ruler_added_foreground : String,
    #[serde(rename = "editorOverviewRuler.deletedForeground")]
    pub editor_overview_ruler_deleted_foreground : String,
    #[serde(rename = "editorOverviewRuler.errorForeground")]
    pub editor_overview_ruler_error_foreground : String,
    #[serde(rename = "editorOverviewRuler.warningForeground")]
    pub editor_overview_ruler_warning_foreground : String,
    #[serde(rename = "editorOverviewRuler.infoForeground")]
    pub editor_overview_ruler_info_foreground : String,
    #[serde(rename = "editorOverviewRuler.bracketMatchForeground")]
    pub editor_overview_ruler_bracket_match_foreground : String,
    #[serde(rename = "editorGutter.modifiedBackground")]
    pub editor_gutter_modified_background : String,
    #[serde(rename = "editorGutter.addedBackground")]
    pub editor_gutter_added_background : String,
    #[serde(rename = "editorGutter.deletedBackground")]
    pub editor_gutter_deleted_background : String,
    #[serde(rename = "diffEditor.insertedTextBackground")]
    pub diff_editor_inserted_text_background : String,
    #[serde(rename = "diffEditor.removedTextBackground")]
    pub diff_editor_removed_text_background : String,
    #[serde(rename = "editorWidget.background")]
    pub editor_widget_background : String,
    #[serde(rename = "editorWidget.border")]
    pub editor_widget_border : String,
    #[serde(rename = "editorSuggestWidget.background")]
    pub editor_suggest_widget_background : String,
    #[serde(rename = "peekView.border")]
    pub peek_view_border : String,
    #[serde(rename = "peekViewEditor.matchHighlightBackground")]
    pub peek_view_editor_match_highlight_background : String,
    #[serde(rename = "peekViewEditorGutter.background")]
    pub peek_view_editor_gutter_background : String,
    #[serde(rename = "peekViewEditor.background")]
    pub peek_view_editor_background : String,
    #[serde(rename = "peekViewResult.background")]
    pub peek_view_result_background : String,
    #[serde(rename = "peekViewTitle.background")]
    pub peek_view_title_background : String,
    #[serde(rename = "merge.currentHeaderBackground")]
    pub merge_current_header_background : String,
    #[serde(rename = "merge.currentContentBackground")]
    pub merge_current_content_background : String,
    #[serde(rename = "merge.incomingHeaderBackground")]
    pub merge_incoming_header_background : String,
    #[serde(rename = "merge.incomingContentBackground")]
    pub merge_incoming_content_background : String,
    #[serde(rename = "panel.background")]
    pub panel_background : String,
    #[serde(rename = "panel.border")]
    pub panel_border : String,
    #[serde(rename = "panelTitle.activeBorder")]
    pub panel_title_active_border : String,
    #[serde(rename = "statusBar.background")]
    pub status_bar_background : String,
    #[serde(rename = "statusBar.debuggingBackground")]
    pub status_bar_debugging_background : String,
    #[serde(rename = "statusBar.debuggingForeground")]
    pub status_bar_debugging_foreground : String,
    #[serde(rename = "statusBar.noFolderForeground")]
    pub status_bar_no_folder_foreground : String,
    #[serde(rename = "statusBar.noFolderBackground")]
    pub status_bar_no_folder_background : String,
    #[serde(rename = "statusBar.foreground")]
    pub status_bar_foreground : String,
    #[serde(rename = "statusBarItem.activeBackground")]
    pub status_bar_item_active_background : String,
    #[serde(rename = "statusBarItem.hoverBackground")]
    pub status_bar_item_hover_background : String,
    #[serde(rename = "statusBarItem.prominentBackground")]
    pub status_bar_item_prominent_background : String,
    #[serde(rename = "statusBarItem.prominentHoverBackground")]
    pub status_bar_item_prominent_hover_background : String,
    #[serde(rename = "statusBar.border")]
    pub status_bar_border : String,
    #[serde(rename = "titleBar.activeBackground")]
    pub title_bar_active_background : String,
    #[serde(rename = "titleBar.activeForeground")]
    pub title_bar_active_foreground : String,
    #[serde(rename = "titleBar.inactiveBackground")]
    pub title_bar_inactive_background : String,
    #[serde(rename = "titleBar.inactiveForeground")]
    pub title_bar_inactive_foreground : String,
    #[serde(rename = "notificationCenterHeader.foreground")]
    pub notification_center_header_foreground : String,
    #[serde(rename = "notificationCenterHeader.background")]
    pub notification_center_header_background : String,
    #[serde(rename = "extensionButton.prominentForeground")]
    pub extension_button_prominent_foreground : String,
    #[serde(rename = "extensionButton.prominentBackground")]
    pub extension_button_prominent_background : String,
    #[serde(rename = "extensionButton.prominentHoverBackground")]
    pub extension_button_prominent_hover_background : String,
    #[serde(rename = "pickerGroup.border")]
    pub picker_group_border : String,
    #[serde(rename = "pickerGroup.foreground")]
    pub picker_group_foreground : String,
    #[serde(rename = "terminal.ansiBrightBlack")]
    pub terminal_ansi_bright_black : String,
    #[serde(rename = "terminal.ansiBlack")]
    pub terminal_ansi_black : String,
    #[serde(rename = "terminal.ansiBlue")]
    pub terminal_ansi_blue : String,
    #[serde(rename = "terminal.ansiBrightBlue")]
    pub terminal_ansi_bright_blue : String,
    #[serde(rename = "terminal.ansiBrightCyan")]
    pub terminal_ansi_bright_cyan : String,
    #[serde(rename = "terminal.ansiCyan")]
    pub terminal_ansi_cyan : String,
    #[serde(rename = "terminal.ansiBrightMagenta")]
    pub terminal_ansi_bright_magenta : String,
    #[serde(rename = "terminal.ansiMagenta")]
    pub terminal_ansi_magenta : String,
    #[serde(rename = "terminal.ansiBrightRed")]
    pub terminal_ansi_bright_red : String,
    #[serde(rename = "terminal.ansiRed")]
    pub terminal_ansi_red : String,
    #[serde(rename = "terminal.ansiYellow")]
    pub terminal_ansi_yellow : String,
    #[serde(rename = "terminal.ansiBrightYellow")]
    pub terminal_ansi_bright_yellow : String,
    #[serde(rename = "terminal.ansiBrightGreen")]
    pub terminal_ansi_bright_green : String,
    #[serde(rename = "terminal.ansiGreen")]
    pub terminal_ansi_green : String,
    #[serde(rename = "terminal.ansiWhite")]
    pub terminal_ansi_white : String,
    #[serde(rename = "terminal.selectionBackground")]
    pub terminal_selection_background : String,
    #[serde(rename = "terminalCursor.background")]
    pub terminal_cursor_background : String,
    #[serde(rename = "terminalCursor.foreground")]
    pub terminal_cursor_foreground : String,
    #[serde(rename = "gitDecoration.modifiedResourceForeground")]
    pub git_decoration_modified_resource_foreground : String,
    #[serde(rename = "gitDecoration.deletedResourceForeground")]
    pub git_decoration_deleted_resource_foreground : String,
    #[serde(rename = "gitDecoration.untrackedResourceForeground")]
    pub git_decoration_untracked_resource_foreground : String,
    #[serde(rename = "gitDecoration.conflictingResourceForeground")]
    pub git_decoration_conflicting_resource_foreground : String,
    #[serde(rename = "gitDecoration.submoduleResourceForeground")]
    pub git_decoration_submodule_resource_foreground : String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Theme {
    name : String,
    #[serde(rename = "type")]
    pub theme_type : String,
    pub colors: Colors,
}

impl Default for Theme {
    fn default() -> Self {
        let s = include_str!("themes/mariana.json");
        json::from_str(&s).unwrap()

    }
}

pub const FOCUS_BORDER : Key<Color> = Key::new("focusBorder");
pub const FOREGROUND : Key<Color> = Key::new("foreground");
pub const SELECTION_BACKGROUND : Key<Color> = Key::new("selection.background");
pub const WIDGET_SHADOW : Key<Color> = Key::new("widget.shadow");
pub const TEXT_LINK_ACTIVE_FOREGROUND : Key<Color> = Key::new("textLink.activeForeground");
pub const TEXT_LINK_FOREGROUND : Key<Color> = Key::new("textLink.foreground");
pub const TEXT_PREFORMAT_FOREGROUND : Key<Color> = Key::new("textPreformat.foreground");
pub const BUTTON_BACKGROUND : Key<Color> = Key::new("button.background");
pub const BUTTON_FOREGROUND : Key<Color> = Key::new("button.foreground");
pub const BUTTON_HOVER_BACKGROUND : Key<Color> = Key::new("button.hoverBackground");
pub const DROPDOWN_BACKGROUND : Key<Color> = Key::new("dropdown.background");
pub const DROPDOWN_LIST_BACKGROUND : Key<Color> = Key::new("dropdown.listBackground");
pub const INPUT_BACKGROUND : Key<Color> = Key::new("input.background");
pub const INPUT_BORDER : Key<Color> = Key::new("input.border");
pub const INPUT_FOREGROUND : Key<Color> = Key::new("input.foreground");
pub const INPUT_PLACEHOLDER_FOREGROUND : Key<Color> = Key::new("input.placeholderForeground");
pub const SCROLLBAR_SHADOW : Key<Color> = Key::new("scrollbar.shadow");
pub const SCROLLBAR_SLIDER_ACTIVE_BACKGROUND : Key<Color> = Key::new("scrollbarSlider.activeBackground");
pub const SCROLLBAR_SLIDER_BACKGROUND : Key<Color> = Key::new("scrollbarSlider.background");
pub const SCROLLBAR_SLIDER_HOVER_BACKGROUND : Key<Color> = Key::new("scrollbarSlider.hoverBackground");
pub const BADGE_FOREGROUND : Key<Color> = Key::new("badge.foreground");
pub const BADGE_BACKGROUND : Key<Color> = Key::new("badge.background");
pub const PROGRESS_BAR_BACKGROUND : Key<Color> = Key::new("progressBar.background");
pub const LIST_ACTIVE_SELECTION_BACKGROUND : Key<Color> = Key::new("list.activeSelectionBackground");
pub const LIST_ACTIVE_SELECTION_FOREGROUND : Key<Color> = Key::new("list.activeSelectionForeground");
pub const LIST_INACTIVE_SELECTION_BACKGROUND : Key<Color> = Key::new("list.inactiveSelectionBackground");
pub const LIST_INACTIVE_SELECTION_FOREGROUND : Key<Color> = Key::new("list.inactiveSelectionForeground");
pub const LIST_HOVER_FOREGROUND : Key<Color> = Key::new("list.hoverForeground");
pub const LIST_FOCUS_FOREGROUND : Key<Color> = Key::new("list.focusForeground");
pub const LIST_FOCUS_BACKGROUND : Key<Color> = Key::new("list.focusBackground");
pub const LIST_HOVER_BACKGROUND : Key<Color> = Key::new("list.hoverBackground");
pub const LIST_DROP_BACKGROUND : Key<Color> = Key::new("list.dropBackground");
pub const LIST_HIGHLIGHT_FOREGROUND : Key<Color> = Key::new("list.highlightForeground");
pub const LIST_ERROR_FOREGROUND : Key<Color> = Key::new("list.errorForeground");
pub const LIST_WARNING_FOREGROUND : Key<Color> = Key::new("list.warningForeground");
pub const ACTIVITY_BAR_BACKGROUND : Key<Color> = Key::new("activityBar.background");
pub const ACTIVITY_BAR_DROP_BACKGROUND : Key<Color> = Key::new("activityBar.dropBackground");
pub const ACTIVITY_BAR_FOREGROUND : Key<Color> = Key::new("activityBar.foreground");
pub const ACTIVITY_BAR_BADGE_BACKGROUND : Key<Color> = Key::new("activityBarBadge.background");
pub const ACTIVITY_BAR_BADGE_FOREGROUND : Key<Color> = Key::new("activityBarBadge.foreground");
pub const SIDE_BAR_BACKGROUND : Key<Color> = Key::new("sideBar.background");
pub const SIDE_BAR_FOREGROUND : Key<Color> = Key::new("sideBar.foreground");
pub const SIDE_BAR_SECTION_HEADER_BACKGROUND : Key<Color> = Key::new("sideBarSectionHeader.background");
pub const SIDE_BAR_SECTION_HEADER_FOREGROUND : Key<Color> = Key::new("sideBarSectionHeader.foreground");
pub const SIDE_BAR_TITLE_FOREGROUND : Key<Color> = Key::new("sideBarTitle.foreground");
pub const EDITOR_GROUP_BORDER : Key<Color> = Key::new("editorGroup.border");
pub const EDITOR_GROUP_DROP_BACKGROUND : Key<Color> = Key::new("editorGroup.dropBackground");
pub const EDITOR_GROUP_HEADER_NO_TABS_BACKGROUND : Key<Color> = Key::new("editorGroupHeader.noTabsBackground");
pub const EDITOR_GROUP_HEADER_TABS_BACKGROUND : Key<Color> = Key::new("editorGroupHeader.tabsBackground");
pub const TAB_ACTIVE_BACKGROUND : Key<Color> = Key::new("tab.activeBackground");
pub const TAB_ACTIVE_FOREGROUND : Key<Color> = Key::new("tab.activeForeground");
pub const TAB_BORDER : Key<Color> = Key::new("tab.border");
pub const TAB_ACTIVE_BORDER : Key<Color> = Key::new("tab.activeBorder");
pub const TAB_UNFOCUSED_ACTIVE_BORDER : Key<Color> = Key::new("tab.unfocusedActiveBorder");
pub const TAB_INACTIVE_BACKGROUND : Key<Color> = Key::new("tab.inactiveBackground");
pub const TAB_INACTIVE_FOREGROUND : Key<Color> = Key::new("tab.inactiveForeground");
pub const TAB_UNFOCUSED_ACTIVE_FOREGROUND : Key<Color> = Key::new("tab.unfocusedActiveForeground");
pub const TAB_UNFOCUSED_INACTIVE_FOREGROUND : Key<Color> = Key::new("tab.unfocusedInactiveForeground");
pub const EDITOR_BACKGROUND : Key<Color> = Key::new("editor.background");
pub const EDITOR_FOREGROUND : Key<Color> = Key::new("editor.foreground");
pub const EDITOR_HOVER_HIGHLIGHT_BACKGROUND : Key<Color> = Key::new("editor.hoverHighlightBackground");
pub const EDITOR_FIND_MATCH_BACKGROUND : Key<Color> = Key::new("editor.findMatchBackground");
pub const EDITOR_FIND_MATCH_HIGHLIGHT_BACKGROUND : Key<Color> = Key::new("editor.findMatchHighlightBackground");
pub const EDITOR_FIND_RANGE_HIGHLIGHT_BACKGROUND : Key<Color> = Key::new("editor.findRangeHighlightBackground");
pub const EDITOR_LINE_HIGHLIGHT_BACKGROUND : Key<Color> = Key::new("editor.lineHighlightBackground");
pub const EDITOR_LINE_HIGHLIGHT_BORDER : Key<Color> = Key::new("editor.lineHighlightBorder");
pub const EDITOR_INACTIVE_SELECTION_BACKGROUND : Key<Color> = Key::new("editor.inactiveSelectionBackground");
pub const EDITOR_SELECTION_BACKGROUND : Key<Color> = Key::new("editor.selectionBackground");
pub const EDITOR_SELECTION_HIGHLIGHT_BACKGROUND : Key<Color> = Key::new("editor.selectionHighlightBackground");
pub const EDITOR_RANGE_HIGHLIGHT_BACKGROUND : Key<Color> = Key::new("editor.rangeHighlightBackground");
pub const EDITOR_WORD_HIGHLIGHT_BACKGROUND : Key<Color> = Key::new("editor.wordHighlightBackground");
pub const EDITOR_WORD_HIGHLIGHT_STRONG_BACKGROUND : Key<Color> = Key::new("editor.wordHighlightStrongBackground");
pub const EDITOR_ERROR_FOREGROUND : Key<Color> = Key::new("editorError.foreground");
pub const EDITOR_ERROR_BORDER : Key<Color> = Key::new("editorError.border");
pub const EDITOR_WARNING_FOREGROUND : Key<Color> = Key::new("editorWarning.foreground");
pub const EDITOR_INFO_FOREGROUND : Key<Color> = Key::new("editorInfo.foreground");
pub const EDITOR_WARNING_BORDER : Key<Color> = Key::new("editorWarning.border");
pub const EDITOR_CURSOR_FOREGROUND : Key<Color> = Key::new("editorCursor.foreground");
pub const EDITOR_INDENT_GUIDE_BACKGROUND : Key<Color> = Key::new("editorIndentGuide.background");
pub const EDITOR_LINE_NUMBER_FOREGROUND : Key<Color> = Key::new("editorLineNumber.foreground");
pub const EDITOR_WHITESPACE_FOREGROUND : Key<Color> = Key::new("editorWhitespace.foreground");
pub const EDITOR_OVERVIEW_RULER_BORDER : Key<Color> = Key::new("editorOverviewRuler.border");
pub const EDITOR_OVERVIEW_RULER_CURRENT_CONTENT_FOREGROUND : Key<Color> = Key::new("editorOverviewRuler.currentContentForeground");
pub const EDITOR_OVERVIEW_RULER_INCOMING_CONTENT_FOREGROUND : Key<Color> = Key::new("editorOverviewRuler.incomingContentForeground");
pub const EDITOR_OVERVIEW_RULER_FIND_MATCH_FOREGROUND : Key<Color> = Key::new("editorOverviewRuler.findMatchForeground");
pub const EDITOR_OVERVIEW_RULER_RANGE_HIGHLIGHT_FOREGROUND : Key<Color> = Key::new("editorOverviewRuler.rangeHighlightForeground");
pub const EDITOR_OVERVIEW_RULER_SELECTION_HIGHLIGHT_FOREGROUND : Key<Color> = Key::new("editorOverviewRuler.selectionHighlightForeground");
pub const EDITOR_OVERVIEW_RULER_WORD_HIGHLIGHT_FOREGROUND : Key<Color> = Key::new("editorOverviewRuler.wordHighlightForeground");
pub const EDITOR_OVERVIEW_RULER_WORD_HIGHLIGHT_STRONG_FOREGROUND : Key<Color> = Key::new("editorOverviewRuler.wordHighlightStrongForeground");
pub const EDITOR_OVERVIEW_RULER_MODIFIED_FOREGRUND : Key<Color> = Key::new("editorOverviewRuler.modifiedForeground");
pub const EDITOR_OVERVIEW_RULER_ADDED_FOREGROUND : Key<Color> = Key::new("editorOverviewRuler.addedForeground");
pub const EDITOR_OVERVIEW_RULER_DELETED_FOREGROUND : Key<Color> = Key::new("editorOverviewRuler.deletedForeground");
pub const EDITOR_OVERVIEW_RULER_ERROR_FOREGROUND : Key<Color> = Key::new("editorOverviewRuler.errorForeground");
pub const EDITOR_OVERVIEW_RULER_WARNING_FOREGROUND : Key<Color> = Key::new("editorOverviewRuler.warningForeground");
pub const EDITOR_OVERVIEW_RULER_INFO_FOREGROUND : Key<Color> = Key::new("editorOverviewRuler.infoForeground");
pub const EDITOR_OVERVIEW_RULER_BRACKET_MATCH_FOREGROUND : Key<Color> = Key::new("editorOverviewRuler.bracketMatchForeground");
pub const EDITOR_GUTTER_MODIFIED_BACKGROUND : Key<Color> = Key::new("editorGutter.modifiedBackground");
pub const EDITOR_GUTTER_ADDED_BACKGROUND : Key<Color> = Key::new("editorGutter.addedBackground");
pub const EDITOR_GUTTER_DELETED_BACKGROUND : Key<Color> = Key::new("editorGutter.deletedBackground");
pub const DIFF_EDITOR_INSERTED_TEXT_BACKGROUND : Key<Color> = Key::new("diffEditor.insertedTextBackground");
pub const DIFF_EDITOR_REMOVED_TEXT_BACKGROUND : Key<Color> = Key::new("diffEditor.removedTextBackground");
pub const EDITOR_WIDGET_BACKGROUND : Key<Color> = Key::new("editorWidget.background");
pub const EDITOR_WIDGET_BORDER : Key<Color> = Key::new("editorWidget.border");
pub const EDITOR_SUGGEST_WIDGET_BACKGROUND : Key<Color> = Key::new("editorSuggestWidget.background");
pub const PEEK_VIEW_BORDER : Key<Color> = Key::new("peekView.border");
pub const PEEK_VIEW_EDITOR_MATCH_HIGHLIGHT_BACKGROUND : Key<Color> = Key::new("peekViewEditor.matchHighlightBackground");
pub const PEEK_VIEW_EDITOR_GUTTER_BACKGROUND : Key<Color> = Key::new("peekViewEditorGutter.background");
pub const PEEK_VIEW_EDITOR_BACKGROUND : Key<Color> = Key::new("peekViewEditor.background");
pub const PEEK_VIEW_RESULT_BACKGROUND : Key<Color> = Key::new("peekViewResult.background");
pub const PEEK_VIEW_TITLE_BACKGROUND : Key<Color> = Key::new("peekViewTitle.background");
pub const MERGE_CURRENT_HEADER_BACKGROUND : Key<Color> = Key::new("merge.currentHeaderBackground");
pub const MERGE_CURRENT_CONTENT_BACKGROUND : Key<Color> = Key::new("merge.currentContentBackground");
pub const MERGE_INCOMING_HEADER_BACKGROUND : Key<Color> = Key::new("merge.incomingHeaderBackground");
pub const MERGE_INCOMING_CONTENT_BACKGROUND : Key<Color> = Key::new("merge.incomingContentBackground");
pub const PANEL_BACKGROUND : Key<Color> = Key::new("panel.background");
pub const PANEL_BORDER : Key<Color> = Key::new("panel.border");
pub const PANEL_TITLE_ACTIVE_BORDER : Key<Color> = Key::new("panelTitle.activeBorder");
pub const STATUS_BAR_BACKGROUND : Key<Color> = Key::new("statusBar.background");
pub const STATUS_BAR_DEBUGGING_BACKGROUND : Key<Color> = Key::new("statusBar.debuggingBackground");
pub const STATUS_BAR_DEBUGGING_FOREGROUND : Key<Color> = Key::new("statusBar.debuggingForeground");
pub const STATUS_BAR_NO_FOLDER_FOREGROUND : Key<Color> = Key::new("statusBar.noFolderForeground");
pub const STATUS_BAR_NO_FOLDER_BACKGROUND : Key<Color> = Key::new("statusBar.noFolderBackground");
pub const STATUS_BAR_FOREGROUND : Key<Color> = Key::new("statusBar.foreground");
pub const STATUS_BAR_ITEM_ACTIVE_BACKGROUND : Key<Color> = Key::new("statusBarItem.activeBackground");
pub const STATUS_BAR_ITEM_HOVER_BACKGROUND : Key<Color> = Key::new("statusBarItem.hoverBackground");
pub const STATUS_BAR_ITEM_PROMINENT_BACKGROUND : Key<Color> = Key::new("statusBarItem.prominentBackground");
pub const STATUS_BAR_ITEM_PROMINENT_HOVER_BACKGROUND : Key<Color> = Key::new("statusBarItem.prominentHoverBackground");
pub const STATUS_BAR_BORDER : Key<Color> = Key::new("statusBar.border");
pub const TITLE_BAR_ACTIVE_BACKGROUND : Key<Color> = Key::new("titleBar.activeBackground");
pub const TITLE_BAR_ACTIVE_FOREGROUND : Key<Color> = Key::new("titleBar.activeForeground");
pub const TITLE_BAR_INACTIVE_BACKGROUND : Key<Color> = Key::new("titleBar.inactiveBackground");
pub const TITLE_BAR_INACTIVE_FOREGROUND : Key<Color> = Key::new("titleBar.inactiveForeground");
pub const NOTIFICATION_CENTER_HEADER_FOREGROUND : Key<Color> = Key::new("notificationCenterHeader.foreground");
pub const NOTIFICATION_CENTER_HEADER_BACKGROUND : Key<Color> = Key::new("notificationCenterHeader.background");
pub const EXTENSION_BUTTON_PROMINENT_FOREGROUND : Key<Color> = Key::new("extensionButton.prominentForeground");
pub const EXTENSION_BUTTON_PROMINENT_BACKGROUND : Key<Color> = Key::new("extensionButton.prominentBackground");
pub const EXTENSION_BUTTON_PROMINENT_HOVER_BACKGROUND : Key<Color> = Key::new("extensionButton.prominentHoverBackground");
pub const PICKER_GROUP_BORDER : Key<Color> = Key::new("pickerGroup.border");
pub const PICKER_GROUP_FOREGROUND : Key<Color> = Key::new("pickerGroup.foreground");
pub const TERMINAL_ANSI_BRIGHT_BLACK : Key<Color> = Key::new("terminal.ansiBrightBlack");
pub const TERMINAL_ANSI_BLACK : Key<Color> = Key::new("terminal.ansiBlack");
pub const TERMINAL_ANSI_BLUE : Key<Color> = Key::new("terminal.ansiBlue");
pub const TERMINAL_ANSI_BRIGHT_BLUE : Key<Color> = Key::new("terminal.ansiBrightBlue");
pub const TERMINAL_ANSI_BRIGHT_CYAN : Key<Color> = Key::new("terminal.ansiBrightCyan");
pub const TERMINAL_ANSI_CYAN : Key<Color> = Key::new("terminal.ansiCyan");
pub const TERMINAL_ANSI_BRIGHT_MAGENTA : Key<Color> = Key::new("terminal.ansiBrightMagenta");
pub const TERMINAL_ANSI_MAGENTA : Key<Color> = Key::new("terminal.ansiMagenta");
pub const TERMINAL_ANSI_BRIGHT_RED : Key<Color> = Key::new("terminal.ansiBrightRed");
pub const TERMINAL_ANSI_RED : Key<Color> = Key::new("terminal.ansiRed");
pub const TERMINAL_ANSI_YELLOW : Key<Color> = Key::new("terminal.ansiYellow");
pub const TERMINAL_ANSI_BRIGHT_YELLOW : Key<Color> = Key::new("terminal.ansiBrightYellow");
pub const TERMINAL_ANSI_BRIGHT_GREEN : Key<Color> = Key::new("terminal.ansiBrightGreen");
pub const TERMINAL_ANSI_GREEN : Key<Color> = Key::new("terminal.ansiGreen");
pub const TERMINAL_ANSI_WHITE : Key<Color> = Key::new("terminal.ansiWhite");
pub const TERMINAL_SELECTION_BACKGROUND : Key<Color> = Key::new("terminal.selectionBackground");
pub const TERMINAL_CURSOR_BACKGROUND : Key<Color> = Key::new("terminalCursor.background");
pub const TERMINAL_CURSOR_FOREGROUND : Key<Color> = Key::new("terminalCursor.foreground");
pub const GIT_DECORATION_MODIFIED_RESOURCE_FOREGROUND : Key<Color> = Key::new("gitDecoration.modifiedResourceForeground");
pub const GIT_DECORATION_DELETED_RESOURCE_FOREGROUND : Key<Color> = Key::new("gitDecoration.deletedResourceForeground");
pub const GIT_DECORATION_UNTRACKED_RESOURCE_FOREGROUND : Key<Color> = Key::new("gitDecoration.untrackedResourceForeground");
pub const GIT_DECORATION_CONFLICTING_RESOURCE_FOREGROUND : Key<Color> = Key::new("gitDecoration.conflictingResourceForeground");
pub const GIT_DECORATION_SUBMODULE_RESOURCE_FOREGROUND : Key<Color> = Key::new("gitDecoration.submoduleResourceForeground");

impl Theme {
    pub fn to_env(&self, env: &mut Env) {
        env.set( FOCUS_BORDER, Color::from_hex_str(&self.colors.focus_border).unwrap());
        env.set( FOREGROUND, Color::from_hex_str(&self.colors.foreground).unwrap());
        env.set( SELECTION_BACKGROUND, Color::from_hex_str(&self.colors.selection_background).unwrap());
        env.set( WIDGET_SHADOW, Color::from_hex_str(&self.colors.widget_shadow).unwrap());
        env.set( TEXT_LINK_ACTIVE_FOREGROUND, Color::from_hex_str(&self.colors.text_link_active_foreground).unwrap());
        env.set( TEXT_LINK_FOREGROUND, Color::from_hex_str(&self.colors.text_link_foreground).unwrap());
        env.set( TEXT_PREFORMAT_FOREGROUND, Color::from_hex_str(&self.colors.text_preformat_foreground).unwrap());
        env.set( BUTTON_BACKGROUND, Color::from_hex_str(&self.colors.button_background).unwrap());
        env.set( BUTTON_FOREGROUND, Color::from_hex_str(&self.colors.button_foreground).unwrap());
        env.set( BUTTON_HOVER_BACKGROUND, Color::from_hex_str(&self.colors.button_hover_background).unwrap());
        env.set( DROPDOWN_BACKGROUND, Color::from_hex_str(&self.colors.dropdown_background).unwrap());
        env.set( DROPDOWN_LIST_BACKGROUND, Color::from_hex_str(&self.colors.dropdown_list_background).unwrap());
        env.set( INPUT_BACKGROUND, Color::from_hex_str(&self.colors.input_background).unwrap());
        env.set( INPUT_BORDER, Color::from_hex_str(&self.colors.input_border).unwrap());
        env.set( INPUT_FOREGROUND, Color::from_hex_str(&self.colors.input_foreground).unwrap());
        env.set( INPUT_PLACEHOLDER_FOREGROUND, Color::from_hex_str(&self.colors.input_placeholder_foreground).unwrap());
        env.set( SCROLLBAR_SHADOW, Color::from_hex_str(&self.colors.scrollbar_shadow).unwrap());
        env.set( SCROLLBAR_SLIDER_ACTIVE_BACKGROUND, Color::from_hex_str(&self.colors.scrollbar_slider_active_background).unwrap());
        env.set( SCROLLBAR_SLIDER_BACKGROUND, Color::from_hex_str(&self.colors.scrollbar_slider_background).unwrap());
        env.set( SCROLLBAR_SLIDER_HOVER_BACKGROUND, Color::from_hex_str(&self.colors.scrollbar_slider_hover_background).unwrap());
        env.set( BADGE_FOREGROUND, Color::from_hex_str(&self.colors.badge_foreground).unwrap());
        env.set( BADGE_BACKGROUND, Color::from_hex_str(&self.colors.badge_background).unwrap());
        env.set( PROGRESS_BAR_BACKGROUND, Color::from_hex_str(&self.colors.progress_bar_background).unwrap());
        env.set( LIST_ACTIVE_SELECTION_BACKGROUND, Color::from_hex_str(&self.colors.list_active_selection_background).unwrap());
        env.set( LIST_ACTIVE_SELECTION_FOREGROUND, Color::from_hex_str(&self.colors.list_active_selection_foreground).unwrap());
        env.set( LIST_INACTIVE_SELECTION_BACKGROUND, Color::from_hex_str(&self.colors.list_inactive_selection_background).unwrap());
        env.set( LIST_INACTIVE_SELECTION_FOREGROUND, Color::from_hex_str(&self.colors.list_inactive_selection_foreground).unwrap());
        env.set( LIST_HOVER_FOREGROUND, Color::from_hex_str(&self.colors.list_hover_foreground).unwrap());
        env.set( LIST_FOCUS_FOREGROUND, Color::from_hex_str(&self.colors.list_focus_foreground).unwrap());
        env.set( LIST_FOCUS_BACKGROUND, Color::from_hex_str(&self.colors.list_focus_background).unwrap());
        env.set( LIST_HOVER_BACKGROUND, Color::from_hex_str(&self.colors.list_hover_background).unwrap());
        env.set( LIST_DROP_BACKGROUND, Color::from_hex_str(&self.colors.list_drop_background).unwrap());
        env.set( LIST_HIGHLIGHT_FOREGROUND, Color::from_hex_str(&self.colors.list_highlight_foreground).unwrap());
        env.set( LIST_ERROR_FOREGROUND, Color::from_hex_str(&self.colors.list_error_foreground).unwrap());
        env.set( LIST_WARNING_FOREGROUND, Color::from_hex_str(&self.colors.list_warning_foreground).unwrap());
        env.set( ACTIVITY_BAR_BACKGROUND, Color::from_hex_str(&self.colors.activity_bar_background).unwrap());
        env.set( ACTIVITY_BAR_DROP_BACKGROUND, Color::from_hex_str(&self.colors.activity_bar_drop_background).unwrap());
        env.set( ACTIVITY_BAR_FOREGROUND, Color::from_hex_str(&self.colors.activity_bar_foreground).unwrap());
        env.set( ACTIVITY_BAR_BADGE_BACKGROUND, Color::from_hex_str(&self.colors.activity_bar_badge_background).unwrap());
        env.set( ACTIVITY_BAR_BADGE_FOREGROUND, Color::from_hex_str(&self.colors.activity_bar_badge_foreground).unwrap());
        env.set( SIDE_BAR_BACKGROUND, Color::from_hex_str(&self.colors.side_bar_background).unwrap());
        env.set( SIDE_BAR_FOREGROUND, Color::from_hex_str(&self.colors.side_bar_foreground).unwrap());
        env.set( SIDE_BAR_SECTION_HEADER_BACKGROUND, Color::from_hex_str(&self.colors.side_bar_section_header_background).unwrap());
        env.set( SIDE_BAR_SECTION_HEADER_FOREGROUND, Color::from_hex_str(&self.colors.side_bar_section_header_foreground).unwrap());
        env.set( SIDE_BAR_TITLE_FOREGROUND, Color::from_hex_str(&self.colors.side_bar_title_foreground).unwrap());
        env.set( EDITOR_GROUP_BORDER, Color::from_hex_str(&self.colors.editor_group_border).unwrap());
        env.set( EDITOR_GROUP_DROP_BACKGROUND, Color::from_hex_str(&self.colors.editor_group_drop_background).unwrap());
        env.set( EDITOR_GROUP_HEADER_NO_TABS_BACKGROUND, Color::from_hex_str(&self.colors.editor_group_header_no_tabs_background).unwrap());
        env.set( EDITOR_GROUP_HEADER_TABS_BACKGROUND, Color::from_hex_str(&self.colors.editor_group_header_tabs_background).unwrap());
        env.set( TAB_ACTIVE_BACKGROUND, Color::from_hex_str(&self.colors.tab_active_background).unwrap());
        env.set( TAB_ACTIVE_FOREGROUND, Color::from_hex_str(&self.colors.tab_active_foreground).unwrap());
        env.set( TAB_BORDER, Color::from_hex_str(&self.colors.tab_border).unwrap());
        env.set( TAB_ACTIVE_BORDER, Color::from_hex_str(&self.colors.tab_active_border).unwrap());
        env.set( TAB_UNFOCUSED_ACTIVE_BORDER, Color::from_hex_str(&self.colors.tab_unfocused_active_border).unwrap());
        env.set( TAB_INACTIVE_BACKGROUND, Color::from_hex_str(&self.colors.tab_inactive_background).unwrap());
        env.set( TAB_INACTIVE_FOREGROUND, Color::from_hex_str(&self.colors.tab_inactive_foreground).unwrap());
        env.set( TAB_UNFOCUSED_ACTIVE_FOREGROUND, Color::from_hex_str(&self.colors.tab_unfocused_active_foreground).unwrap());
        env.set( TAB_UNFOCUSED_INACTIVE_FOREGROUND, Color::from_hex_str(&self.colors.tab_unfocused_inactive_foreground).unwrap());
        env.set( EDITOR_BACKGROUND, Color::from_hex_str(&self.colors.editor_background).unwrap());
        env.set( EDITOR_FOREGROUND, Color::from_hex_str(&self.colors.editor_foreground).unwrap());
        env.set( EDITOR_HOVER_HIGHLIGHT_BACKGROUND, Color::from_hex_str(&self.colors.editor_hover_highlight_background).unwrap());
        env.set( EDITOR_FIND_MATCH_BACKGROUND, Color::from_hex_str(&self.colors.editor_find_match_background).unwrap());
        env.set( EDITOR_FIND_MATCH_HIGHLIGHT_BACKGROUND, Color::from_hex_str(&self.colors.editor_find_match_highlight_background).unwrap());
        env.set( EDITOR_FIND_RANGE_HIGHLIGHT_BACKGROUND, Color::from_hex_str(&self.colors.editor_find_range_highlight_background).unwrap());
        env.set( EDITOR_LINE_HIGHLIGHT_BACKGROUND, Color::from_hex_str(&self.colors.editor_line_highlight_background).unwrap());
        env.set( EDITOR_LINE_HIGHLIGHT_BORDER, Color::from_hex_str(&self.colors.editor_line_highlight_border).unwrap());
        env.set( EDITOR_INACTIVE_SELECTION_BACKGROUND, Color::from_hex_str(&self.colors.editor_inactive_selection_background).unwrap());
        env.set( EDITOR_SELECTION_BACKGROUND, Color::from_hex_str(&self.colors.editor_selection_background).unwrap());
        env.set( EDITOR_SELECTION_HIGHLIGHT_BACKGROUND, Color::from_hex_str(&self.colors.editor_selection_highlight_background).unwrap());
        env.set( EDITOR_RANGE_HIGHLIGHT_BACKGROUND, Color::from_hex_str(&self.colors.editor_range_highlight_background).unwrap());
        env.set( EDITOR_WORD_HIGHLIGHT_BACKGROUND, Color::from_hex_str(&self.colors.editor_word_highlight_background).unwrap());
        env.set( EDITOR_WORD_HIGHLIGHT_STRONG_BACKGROUND, Color::from_hex_str(&self.colors.editor_word_highlight_strong_background).unwrap());
        env.set( EDITOR_ERROR_FOREGROUND, Color::from_hex_str(&self.colors.editor_error_foreground).unwrap());
        env.set( EDITOR_ERROR_BORDER, Color::from_hex_str(&self.colors.editor_error_border).unwrap());
        env.set( EDITOR_WARNING_FOREGROUND, Color::from_hex_str(&self.colors.editor_warning_foreground).unwrap());
        env.set( EDITOR_INFO_FOREGROUND, Color::from_hex_str(&self.colors.editor_info_foreground).unwrap());
        env.set( EDITOR_WARNING_BORDER, Color::from_hex_str(&self.colors.editor_warning_border).unwrap());
        env.set( EDITOR_CURSOR_FOREGROUND, Color::from_hex_str(&self.colors.editor_cursor_foreground).unwrap());
        env.set( EDITOR_INDENT_GUIDE_BACKGROUND, Color::from_hex_str(&self.colors.editor_indent_guide_background).unwrap());
        env.set( EDITOR_LINE_NUMBER_FOREGROUND, Color::from_hex_str(&self.colors.editor_line_number_foreground).unwrap());
        env.set( EDITOR_WHITESPACE_FOREGROUND, Color::from_hex_str(&self.colors.editor_whitespace_foreground).unwrap());
        env.set( EDITOR_OVERVIEW_RULER_BORDER, Color::from_hex_str(&self.colors.editor_overview_ruler_border).unwrap());
        env.set( EDITOR_OVERVIEW_RULER_CURRENT_CONTENT_FOREGROUND, Color::from_hex_str(&self.colors.editor_overview_ruler_current_content_foreground).unwrap());
        env.set( EDITOR_OVERVIEW_RULER_INCOMING_CONTENT_FOREGROUND, Color::from_hex_str(&self.colors.editor_overview_ruler_incoming_content_foreground).unwrap());
        env.set( EDITOR_OVERVIEW_RULER_FIND_MATCH_FOREGROUND, Color::from_hex_str(&self.colors.editor_overview_ruler_find_match_foreground).unwrap());
        env.set( EDITOR_OVERVIEW_RULER_RANGE_HIGHLIGHT_FOREGROUND, Color::from_hex_str(&self.colors.editor_overview_ruler_range_highlight_foreground).unwrap());
        env.set( EDITOR_OVERVIEW_RULER_SELECTION_HIGHLIGHT_FOREGROUND, Color::from_hex_str(&self.colors.editor_overview_ruler_selection_highlight_foreground).unwrap());
        env.set( EDITOR_OVERVIEW_RULER_WORD_HIGHLIGHT_FOREGROUND, Color::from_hex_str(&self.colors.editor_overview_ruler_word_highlight_foreground).unwrap());
        env.set( EDITOR_OVERVIEW_RULER_WORD_HIGHLIGHT_STRONG_FOREGROUND, Color::from_hex_str(&self.colors.editor_overview_ruler_word_highlight_strong_foreground).unwrap());
        env.set( EDITOR_OVERVIEW_RULER_MODIFIED_FOREGRUND, Color::from_hex_str(&self.colors.editor_overview_ruler_modified_foregrund).unwrap());
        env.set( EDITOR_OVERVIEW_RULER_ADDED_FOREGROUND, Color::from_hex_str(&self.colors.editor_overview_ruler_added_foreground).unwrap());
        env.set( EDITOR_OVERVIEW_RULER_DELETED_FOREGROUND, Color::from_hex_str(&self.colors.editor_overview_ruler_deleted_foreground).unwrap());
        env.set( EDITOR_OVERVIEW_RULER_ERROR_FOREGROUND, Color::from_hex_str(&self.colors.editor_overview_ruler_error_foreground).unwrap());
        env.set( EDITOR_OVERVIEW_RULER_WARNING_FOREGROUND, Color::from_hex_str(&self.colors.editor_overview_ruler_warning_foreground).unwrap());
        env.set( EDITOR_OVERVIEW_RULER_INFO_FOREGROUND, Color::from_hex_str(&self.colors.editor_overview_ruler_info_foreground).unwrap());
        env.set( EDITOR_OVERVIEW_RULER_BRACKET_MATCH_FOREGROUND, Color::from_hex_str(&self.colors.editor_overview_ruler_bracket_match_foreground).unwrap());
        env.set( EDITOR_GUTTER_MODIFIED_BACKGROUND, Color::from_hex_str(&self.colors.editor_gutter_modified_background).unwrap());
        env.set( EDITOR_GUTTER_ADDED_BACKGROUND, Color::from_hex_str(&self.colors.editor_gutter_added_background).unwrap());
        env.set( EDITOR_GUTTER_DELETED_BACKGROUND, Color::from_hex_str(&self.colors.editor_gutter_deleted_background).unwrap());
        env.set( DIFF_EDITOR_INSERTED_TEXT_BACKGROUND, Color::from_hex_str(&self.colors.diff_editor_inserted_text_background).unwrap());
        env.set( DIFF_EDITOR_REMOVED_TEXT_BACKGROUND, Color::from_hex_str(&self.colors.diff_editor_removed_text_background).unwrap());
        env.set( EDITOR_WIDGET_BACKGROUND, Color::from_hex_str(&self.colors.editor_widget_background).unwrap());
        env.set( EDITOR_WIDGET_BORDER, Color::from_hex_str(&self.colors.editor_widget_border).unwrap());
        env.set( EDITOR_SUGGEST_WIDGET_BACKGROUND, Color::from_hex_str(&self.colors.editor_suggest_widget_background).unwrap());
        env.set( PEEK_VIEW_BORDER, Color::from_hex_str(&self.colors.peek_view_border).unwrap());
        env.set( PEEK_VIEW_EDITOR_MATCH_HIGHLIGHT_BACKGROUND, Color::from_hex_str(&self.colors.peek_view_editor_match_highlight_background).unwrap());
        env.set( PEEK_VIEW_EDITOR_GUTTER_BACKGROUND, Color::from_hex_str(&self.colors.peek_view_editor_gutter_background).unwrap());
        env.set( PEEK_VIEW_EDITOR_BACKGROUND, Color::from_hex_str(&self.colors.peek_view_editor_background).unwrap());
        env.set( PEEK_VIEW_RESULT_BACKGROUND, Color::from_hex_str(&self.colors.peek_view_result_background).unwrap());
        env.set( PEEK_VIEW_TITLE_BACKGROUND, Color::from_hex_str(&self.colors.peek_view_title_background).unwrap());
        env.set( MERGE_CURRENT_HEADER_BACKGROUND, Color::from_hex_str(&self.colors.merge_current_header_background).unwrap());
        env.set( MERGE_CURRENT_CONTENT_BACKGROUND, Color::from_hex_str(&self.colors.merge_current_content_background).unwrap());
        env.set( MERGE_INCOMING_HEADER_BACKGROUND, Color::from_hex_str(&self.colors.merge_incoming_header_background).unwrap());
        env.set( MERGE_INCOMING_CONTENT_BACKGROUND, Color::from_hex_str(&self.colors.merge_incoming_content_background).unwrap());
        env.set( PANEL_BACKGROUND, Color::from_hex_str(&self.colors.panel_background).unwrap());
        env.set( PANEL_BORDER, Color::from_hex_str(&self.colors.panel_border).unwrap());
        env.set( PANEL_TITLE_ACTIVE_BORDER, Color::from_hex_str(&self.colors.panel_title_active_border).unwrap());
        env.set( STATUS_BAR_BACKGROUND, Color::from_hex_str(&self.colors.status_bar_background).unwrap());
        env.set( STATUS_BAR_DEBUGGING_BACKGROUND, Color::from_hex_str(&self.colors.status_bar_debugging_background).unwrap());
        env.set( STATUS_BAR_DEBUGGING_FOREGROUND, Color::from_hex_str(&self.colors.status_bar_debugging_foreground).unwrap());
        env.set( STATUS_BAR_NO_FOLDER_FOREGROUND, Color::from_hex_str(&self.colors.status_bar_no_folder_foreground).unwrap());
        env.set( STATUS_BAR_NO_FOLDER_BACKGROUND, Color::from_hex_str(&self.colors.status_bar_no_folder_background).unwrap());
        env.set( STATUS_BAR_FOREGROUND, Color::from_hex_str(&self.colors.status_bar_foreground).unwrap());
        env.set( STATUS_BAR_ITEM_ACTIVE_BACKGROUND, Color::from_hex_str(&self.colors.status_bar_item_active_background).unwrap());
        env.set( STATUS_BAR_ITEM_HOVER_BACKGROUND, Color::from_hex_str(&self.colors.status_bar_item_hover_background).unwrap());
        env.set( STATUS_BAR_ITEM_PROMINENT_BACKGROUND, Color::from_hex_str(&self.colors.status_bar_item_prominent_background).unwrap());
        env.set( STATUS_BAR_ITEM_PROMINENT_HOVER_BACKGROUND, Color::from_hex_str(&self.colors.status_bar_item_prominent_hover_background).unwrap());
        env.set( STATUS_BAR_BORDER, Color::from_hex_str(&self.colors.status_bar_border).unwrap());
        env.set( TITLE_BAR_ACTIVE_BACKGROUND, Color::from_hex_str(&self.colors.title_bar_active_background).unwrap());
        env.set( TITLE_BAR_ACTIVE_FOREGROUND, Color::from_hex_str(&self.colors.title_bar_active_foreground).unwrap());
        env.set( TITLE_BAR_INACTIVE_BACKGROUND, Color::from_hex_str(&self.colors.title_bar_inactive_background).unwrap());
        env.set( TITLE_BAR_INACTIVE_FOREGROUND, Color::from_hex_str(&self.colors.title_bar_inactive_foreground).unwrap());
        env.set( NOTIFICATION_CENTER_HEADER_FOREGROUND, Color::from_hex_str(&self.colors.notification_center_header_foreground).unwrap());
        env.set( NOTIFICATION_CENTER_HEADER_BACKGROUND, Color::from_hex_str(&self.colors.notification_center_header_background).unwrap());
        env.set( EXTENSION_BUTTON_PROMINENT_FOREGROUND, Color::from_hex_str(&self.colors.extension_button_prominent_foreground).unwrap());
        env.set( EXTENSION_BUTTON_PROMINENT_BACKGROUND, Color::from_hex_str(&self.colors.extension_button_prominent_background).unwrap());
        env.set( EXTENSION_BUTTON_PROMINENT_HOVER_BACKGROUND, Color::from_hex_str(&self.colors.extension_button_prominent_hover_background).unwrap());
        env.set( PICKER_GROUP_BORDER, Color::from_hex_str(&self.colors.picker_group_border).unwrap());
        env.set( PICKER_GROUP_FOREGROUND, Color::from_hex_str(&self.colors.picker_group_foreground).unwrap());
        env.set( TERMINAL_ANSI_BRIGHT_BLACK, Color::from_hex_str(&self.colors.terminal_ansi_bright_black).unwrap());
        env.set( TERMINAL_ANSI_BLACK, Color::from_hex_str(&self.colors.terminal_ansi_black).unwrap());
        env.set( TERMINAL_ANSI_BLUE, Color::from_hex_str(&self.colors.terminal_ansi_blue).unwrap());
        env.set( TERMINAL_ANSI_BRIGHT_BLUE, Color::from_hex_str(&self.colors.terminal_ansi_bright_blue).unwrap());
        env.set( TERMINAL_ANSI_BRIGHT_CYAN, Color::from_hex_str(&self.colors.terminal_ansi_bright_cyan).unwrap());
        env.set( TERMINAL_ANSI_CYAN, Color::from_hex_str(&self.colors.terminal_ansi_cyan).unwrap());
        env.set( TERMINAL_ANSI_BRIGHT_MAGENTA, Color::from_hex_str(&self.colors.terminal_ansi_bright_magenta).unwrap());
        env.set( TERMINAL_ANSI_MAGENTA, Color::from_hex_str(&self.colors.terminal_ansi_magenta).unwrap());
        env.set( TERMINAL_ANSI_BRIGHT_RED, Color::from_hex_str(&self.colors.terminal_ansi_bright_red).unwrap());
        env.set( TERMINAL_ANSI_RED, Color::from_hex_str(&self.colors.terminal_ansi_red).unwrap());
        env.set( TERMINAL_ANSI_YELLOW, Color::from_hex_str(&self.colors.terminal_ansi_yellow).unwrap());
        env.set( TERMINAL_ANSI_BRIGHT_YELLOW, Color::from_hex_str(&self.colors.terminal_ansi_bright_yellow).unwrap());
        env.set( TERMINAL_ANSI_BRIGHT_GREEN, Color::from_hex_str(&self.colors.terminal_ansi_bright_green).unwrap());
        env.set( TERMINAL_ANSI_GREEN, Color::from_hex_str(&self.colors.terminal_ansi_green).unwrap());
        env.set( TERMINAL_ANSI_WHITE, Color::from_hex_str(&self.colors.terminal_ansi_white).unwrap());
        env.set( TERMINAL_SELECTION_BACKGROUND, Color::from_hex_str(&self.colors.terminal_selection_background).unwrap());
        env.set( TERMINAL_CURSOR_BACKGROUND, Color::from_hex_str(&self.colors.terminal_cursor_background).unwrap());
        env.set( TERMINAL_CURSOR_FOREGROUND, Color::from_hex_str(&self.colors.terminal_cursor_foreground).unwrap());
        env.set( GIT_DECORATION_MODIFIED_RESOURCE_FOREGROUND, Color::from_hex_str(&self.colors.git_decoration_modified_resource_foreground).unwrap());
        env.set( GIT_DECORATION_DELETED_RESOURCE_FOREGROUND, Color::from_hex_str(&self.colors.git_decoration_deleted_resource_foreground).unwrap());
        env.set( GIT_DECORATION_UNTRACKED_RESOURCE_FOREGROUND, Color::from_hex_str(&self.colors.git_decoration_untracked_resource_foreground).unwrap());
        env.set( GIT_DECORATION_CONFLICTING_RESOURCE_FOREGROUND, Color::from_hex_str(&self.colors.git_decoration_conflicting_resource_foreground).unwrap());
        env.set( GIT_DECORATION_SUBMODULE_RESOURCE_FOREGROUND, Color::from_hex_str(&self.colors.git_decoration_submodule_resource_foreground).unwrap());
    }
}