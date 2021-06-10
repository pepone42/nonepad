#[rustfmt::skip]

use std::collections::HashMap;

use druid::{Color, Env, Key};
use miniserde::{Deserialize, Serialize, json};
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Colors {
    #[serde(rename = "focusBorder")]
    focus_border : String,
    #[serde(rename = "foreground")]
    foreground : String,
    #[serde(rename = "selection.background")]
    selection_background : String,
    #[serde(rename = "widget.shadow")]
    widget_shadow : String,
    #[serde(rename = "textLink.activeForeground")]
    text_link_active_foreground : String,
    #[serde(rename = "textLink.foreground")]
    text_link_foreground : String,
    #[serde(rename = "textPreformat.foreground")]
    text_preformat_foreground : String,
    #[serde(rename = "button.background")]
    button_background : String,
    #[serde(rename = "button.foreground")]
    button_foreground : String,
    #[serde(rename = "button.hoverBackground")]
    button_hover_background : String,
    #[serde(rename = "dropdown.background")]
    dropdown_background : String,
    #[serde(rename = "dropdown.listBackground")]
    dropdown_list_background : String,
    #[serde(rename = "input.background")]
    input_background : String,
    #[serde(rename = "input.border")]
    input_border : String,
    #[serde(rename = "input.foreground")]
    input_foreground : String,
    #[serde(rename = "input.placeholderForeground")]
    input_placeholder_foreground : String,
    #[serde(rename = "scrollbar.shadow")]
    scrollbar_shadow : String,
    #[serde(rename = "scrollbarSlider.activeBackground")]
    scrollbar_slider_active_background : String,
    #[serde(rename = "scrollbarSlider.background")]
    scrollbar_slider_background : String,
    #[serde(rename = "scrollbarSlider.hoverBackground")]
    scrollbar_slider_hover_background : String,
    #[serde(rename = "badge.foreground")]
    badge_foreground : String,
    #[serde(rename = "badge.background")]
    badge_background : String,
    #[serde(rename = "progressBar.background")]
    progress_bar_background : String,
    #[serde(rename = "list.activeSelectionBackground")]
    list_active_selection_background : String,
    #[serde(rename = "list.activeSelectionForeground")]
    list_active_selection_foreground : String,
    #[serde(rename = "list.inactiveSelectionBackground")]
    list_inactive_selection_background : String,
    #[serde(rename = "list.inactiveSelectionForeground")]
    list_inactive_selection_foreground : String,
    #[serde(rename = "list.hoverForeground")]
    list_hover_foreground : String,
    #[serde(rename = "list.focusForeground")]
    list_focus_foreground : String,
    #[serde(rename = "list.focusBackground")]
    list_focus_background : String,
    #[serde(rename = "list.hoverBackground")]
    list_hover_background : String,
    #[serde(rename = "list.dropBackground")]
    list_drop_background : String,
    #[serde(rename = "list.highlightForeground")]
    list_highlight_foreground : String,
    #[serde(rename = "list.errorForeground")]
    list_error_foreground : String,
    #[serde(rename = "list.warningForeground")]
    list_warning_foreground : String,
    #[serde(rename = "activityBar.background")]
    activity_bar_background : String,
    #[serde(rename = "activityBar.dropBackground")]
    activity_bar_drop_background : String,
    #[serde(rename = "activityBar.foreground")]
    activity_bar_foreground : String,
    #[serde(rename = "activityBarBadge.background")]
    activity_bar_badge_background : String,
    #[serde(rename = "activityBarBadge.foreground")]
    activity_bar_badge_foreground : String,
    #[serde(rename = "sideBar.background")]
    side_bar_background : String,
    #[serde(rename = "sideBar.foreground")]
    side_bar_foreground : String,
    #[serde(rename = "sideBarSectionHeader.background")]
    side_bar_section_header_background : String,
    #[serde(rename = "sideBarSectionHeader.foreground")]
    side_bar_section_header_foreground : String,
    #[serde(rename = "sideBarTitle.foreground")]
    side_bar_title_foreground : String,
    #[serde(rename = "editorGroup.border")]
    editor_group_border : String,
    #[serde(rename = "editorGroup.dropBackground")]
    editor_group_drop_background : String,
    #[serde(rename = "editorGroupHeader.noTabsBackground")]
    editor_group_header_no_tabs_background : String,
    #[serde(rename = "editorGroupHeader.tabsBackground")]
    editor_group_header_tabs_background : String,
    #[serde(rename = "tab.activeBackground")]
    tab_active_background : String,
    #[serde(rename = "tab.activeForeground")]
    tab_active_foreground : String,
    #[serde(rename = "tab.border")]
    tab_border : String,
    #[serde(rename = "tab.activeBorder")]
    tab_active_border : String,
    #[serde(rename = "tab.unfocusedActiveBorder")]
    tab_unfocused_active_border : String,
    #[serde(rename = "tab.inactiveBackground")]
    tab_inactive_background : String,
    #[serde(rename = "tab.inactiveForeground")]
    tab_inactive_foreground : String,
    #[serde(rename = "tab.unfocusedActiveForeground")]
    tab_unfocused_active_foreground : String,
    #[serde(rename = "tab.unfocusedInactiveForeground")]
    tab_unfocused_inactive_foreground : String,
    #[serde(rename = "editor.background")]
    editor_background : String,
    #[serde(rename = "editor.foreground")]
    editor_foreground : String,
    #[serde(rename = "editor.hoverHighlightBackground")]
    editor_hover_highlight_background : String,
    #[serde(rename = "editor.findMatchBackground")]
    editor_find_match_background : String,
    #[serde(rename = "editor.findMatchHighlightBackground")]
    editor_find_match_highlight_background : String,
    #[serde(rename = "editor.findRangeHighlightBackground")]
    editor_find_range_highlight_background : String,
    #[serde(rename = "editor.lineHighlightBackground")]
    editor_line_highlight_background : String,
    #[serde(rename = "editor.lineHighlightBorder")]
    editor_line_highlight_border : String,
    #[serde(rename = "editor.inactiveSelectionBackground")]
    editor_inactive_selection_background : String,
    #[serde(rename = "editor.selectionBackground")]
    editor_selection_background : String,
    #[serde(rename = "editor.selectionHighlightBackground")]
    editor_selection_highlight_background : String,
    #[serde(rename = "editor.rangeHighlightBackground")]
    editor_range_highlight_background : String,
    #[serde(rename = "editor.wordHighlightBackground")]
    editor_word_highlight_background : String,
    #[serde(rename = "editor.wordHighlightStrongBackground")]
    editor_word_highlight_strong_background : String,
    #[serde(rename = "editorError.foreground")]
    editor_error_foreground : String,
    #[serde(rename = "editorError.border")]
    editor_error_border : String,
    #[serde(rename = "editorWarning.foreground")]
    editor_warning_foreground : String,
    #[serde(rename = "editorInfo.foreground")]
    editor_info_foreground : String,
    #[serde(rename = "editorWarning.border")]
    editor_warning_border : String,
    #[serde(rename = "editorCursor.foreground")]
    editor_cursor_foreground : String,
    #[serde(rename = "editorIndentGuide.background")]
    editor_indent_guide_background : String,
    #[serde(rename = "editorLineNumber.foreground")]
    editor_line_number_foreground : String,
    #[serde(rename = "editorWhitespace.foreground")]
    editor_whitespace_foreground : String,
    #[serde(rename = "editorOverviewRuler.border")]
    editor_overview_ruler_border : String,
    #[serde(rename = "editorOverviewRuler.currentContentForeground")]
    editor_overview_ruler_current_content_foreground : String,
    #[serde(rename = "editorOverviewRuler.incomingContentForeground")]
    editor_overview_ruler_incoming_content_foreground : String,
    #[serde(rename = "editorOverviewRuler.findMatchForeground")]
    editor_overview_ruler_find_match_foreground : String,
    #[serde(rename = "editorOverviewRuler.rangeHighlightForeground")]
    editor_overview_ruler_range_highlight_foreground : String,
    #[serde(rename = "editorOverviewRuler.selectionHighlightForeground")]
    editor_overview_ruler_selection_highlight_foreground : String,
    #[serde(rename = "editorOverviewRuler.wordHighlightForeground")]
    editor_overview_ruler_word_highlight_foreground : String,
    #[serde(rename = "editorOverviewRuler.wordHighlightStrongForeground")]
    editor_overview_ruler_word_highlight_strong_foreground : String,
    #[serde(rename = "editorOverviewRuler.modifiedForeground")]
    editor_overview_ruler_modified_foregrund : String,
    #[serde(rename = "editorOverviewRuler.addedForeground")]
    editor_overview_ruler_added_foreground : String,
    #[serde(rename = "editorOverviewRuler.deletedForeground")]
    editor_overview_ruler_deleted_foreground : String,
    #[serde(rename = "editorOverviewRuler.errorForeground")]
    editor_overview_ruler_error_foreground : String,
    #[serde(rename = "editorOverviewRuler.warningForeground")]
    editor_overview_ruler_warning_foreground : String,
    #[serde(rename = "editorOverviewRuler.infoForeground")]
    editor_overview_ruler_info_foreground : String,
    #[serde(rename = "editorOverviewRuler.bracketMatchForeground")]
    editor_overview_ruler_bracket_match_foreground : String,
    #[serde(rename = "editorGutter.modifiedBackground")]
    editor_gutter_modified_background : String,
    #[serde(rename = "editorGutter.addedBackground")]
    editor_gutter_added_background : String,
    #[serde(rename = "editorGutter.deletedBackground")]
    editor_gutter_deleted_background : String,
    #[serde(rename = "diffEditor.insertedTextBackground")]
    diff_editor_inserted_text_background : String,
    #[serde(rename = "diffEditor.removedTextBackground")]
    diff_editor_removed_text_background : String,
    #[serde(rename = "editorWidget.background")]
    editor_widget_background : String,
    #[serde(rename = "editorWidget.border")]
    editor_widget_border : String,
    #[serde(rename = "editorSuggestWidget.background")]
    editor_suggest_widget_background : String,
    #[serde(rename = "peekView.border")]
    peek_view_border : String,
    #[serde(rename = "peekViewEditor.matchHighlightBackground")]
    peek_view_editor_match_highlight_background : String,
    #[serde(rename = "peekViewEditorGutter.background")]
    peek_view_editor_gutter_background : String,
    #[serde(rename = "peekViewEditor.background")]
    peek_view_editor_background : String,
    #[serde(rename = "peekViewResult.background")]
    peek_view_result_background : String,
    #[serde(rename = "peekViewTitle.background")]
    peek_view_title_background : String,
    #[serde(rename = "merge.currentHeaderBackground")]
    merge_current_header_background : String,
    #[serde(rename = "merge.currentContentBackground")]
    merge_current_content_background : String,
    #[serde(rename = "merge.incomingHeaderBackground")]
    merge_incoming_header_background : String,
    #[serde(rename = "merge.incomingContentBackground")]
    merge_incoming_content_background : String,
    #[serde(rename = "panel.background")]
    panel_background : String,
    #[serde(rename = "panel.border")]
    panel_border : String,
    #[serde(rename = "panelTitle.activeBorder")]
    panel_title_active_border : String,
    #[serde(rename = "statusBar.background")]
    status_bar_background : String,
    #[serde(rename = "statusBar.debuggingBackground")]
    status_bar_debugging_background : String,
    #[serde(rename = "statusBar.debuggingForeground")]
    status_bar_debugging_foreground : String,
    #[serde(rename = "statusBar.noFolderForeground")]
    status_bar_no_folder_foreground : String,
    #[serde(rename = "statusBar.noFolderBackground")]
    status_bar_no_folder_background : String,
    #[serde(rename = "statusBar.foreground")]
    status_bar_foreground : String,
    #[serde(rename = "statusBarItem.activeBackground")]
    status_bar_item_active_background : String,
    #[serde(rename = "statusBarItem.hoverBackground")]
    status_bar_item_hover_background : String,
    #[serde(rename = "statusBarItem.prominentBackground")]
    status_bar_item_prominent_background : String,
    #[serde(rename = "statusBarItem.prominentHoverBackground")]
    status_bar_item_prominent_hover_background : String,
    #[serde(rename = "statusBar.border")]
    status_bar_border : String,
    #[serde(rename = "titleBar.activeBackground")]
    title_bar_active_background : String,
    #[serde(rename = "titleBar.activeForeground")]
    title_bar_active_foreground : String,
    #[serde(rename = "titleBar.inactiveBackground")]
    title_bar_inactive_background : String,
    #[serde(rename = "titleBar.inactiveForeground")]
    title_bar_inactive_foreground : String,
    #[serde(rename = "notificationCenterHeader.foreground")]
    notification_center_header_foreground : String,
    #[serde(rename = "notificationCenterHeader.background")]
    notification_center_header_background : String,
    #[serde(rename = "extensionButton.prominentForeground")]
    extension_button_prominent_foreground : String,
    #[serde(rename = "extensionButton.prominentBackground")]
    extension_button_prominent_background : String,
    #[serde(rename = "extensionButton.prominentHoverBackground")]
    extension_button_prominent_hover_background : String,
    #[serde(rename = "pickerGroup.border")]
    picker_group_border : String,
    #[serde(rename = "pickerGroup.foreground")]
    picker_group_foreground : String,
    #[serde(rename = "terminal.ansiBrightBlack")]
    terminal_ansi_bright_black : String,
    #[serde(rename = "terminal.ansiBlack")]
    terminal_ansi_black : String,
    #[serde(rename = "terminal.ansiBlue")]
    terminal_ansi_blue : String,
    #[serde(rename = "terminal.ansiBrightBlue")]
    terminal_ansi_bright_blue : String,
    #[serde(rename = "terminal.ansiBrightCyan")]
    terminal_ansi_bright_cyan : String,
    #[serde(rename = "terminal.ansiCyan")]
    terminal_ansi_cyan : String,
    #[serde(rename = "terminal.ansiBrightMagenta")]
    terminal_ansi_bright_magenta : String,
    #[serde(rename = "terminal.ansiMagenta")]
    terminal_ansi_magenta : String,
    #[serde(rename = "terminal.ansiBrightRed")]
    terminal_ansi_bright_red : String,
    #[serde(rename = "terminal.ansiRed")]
    terminal_ansi_red : String,
    #[serde(rename = "terminal.ansiYellow")]
    terminal_ansi_yellow : String,
    #[serde(rename = "terminal.ansiBrightYellow")]
    terminal_ansi_bright_yellow : String,
    #[serde(rename = "terminal.ansiBrightGreen")]
    terminal_ansi_bright_green : String,
    #[serde(rename = "terminal.ansiGreen")]
    terminal_ansi_green : String,
    #[serde(rename = "terminal.ansiWhite")]
    terminal_ansi_white : String,
    #[serde(rename = "terminal.selectionBackground")]
    terminal_selection_background : String,
    #[serde(rename = "terminalCursor.background")]
    terminal_cursor_background : String,
    #[serde(rename = "terminalCursor.foreground")]
    terminal_cursor_foreground : String,
    #[serde(rename = "gitDecoration.modifiedResourceForeground")]
    git_decoration_modified_resource_foreground : String,
    #[serde(rename = "gitDecoration.deletedResourceForeground")]
    git_decoration_deleted_resource_foreground : String,
    #[serde(rename = "gitDecoration.untrackedResourceForeground")]
    git_decoration_untracked_resource_foreground : String,
    #[serde(rename = "gitDecoration.conflictingResourceForeground")]
    git_decoration_conflicting_resource_foreground : String,
    #[serde(rename = "gitDecoration.submoduleResourceForeground")]
    git_decoration_submodule_resource_foreground : String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Theme {
    name : String,
    #[serde(rename = "type")]
    theme_type : String,
    colors: Colors,
}

impl Default for Theme {
    fn default() -> Self {
        let s = include_str!("themes/mariana.json");
        json::from_str(&s).unwrap()

    }
}

const FOCUS_BORDER : Key<Color> = Key::new("focusBorder");
const FOREGROUND : Key<Color> = Key::new("foreground");
const SELECTION_BACKGROUND : Key<Color> = Key::new("selection.background");
const WIDGET_SHADOW : Key<Color> = Key::new("widget.shadow");
const TEXT_LINK_ACTIVE_FOREGROUND : Key<Color> = Key::new("textLink.activeForeground");
const TEXT_LINK_FOREGROUND : Key<Color> = Key::new("textLink.foreground");
const TEXT_PREFORMAT_FOREGROUND : Key<Color> = Key::new("textPreformat.foreground");
const BUTTON_BACKGROUND : Key<Color> = Key::new("button.background");
const BUTTON_FOREGROUND : Key<Color> = Key::new("button.foreground");
const BUTTON_HOVER_BACKGROUND : Key<Color> = Key::new("button.hoverBackground");
const DROPDOWN_BACKGROUND : Key<Color> = Key::new("dropdown.background");
const DROPDOWN_LIST_BACKGROUND : Key<Color> = Key::new("dropdown.listBackground");
const INPUT_BACKGROUND : Key<Color> = Key::new("input.background");
const INPUT_BORDER : Key<Color> = Key::new("input.border");
const INPUT_FOREGROUND : Key<Color> = Key::new("input.foreground");
const INPUT_PLACEHOLDER_FOREGROUND : Key<Color> = Key::new("input.placeholderForeground");
const SCROLLBAR_SHADOW : Key<Color> = Key::new("scrollbar.shadow");
const SCROLLBAR_SLIDER_ACTIVE_BACKGROUND : Key<Color> = Key::new("scrollbarSlider.activeBackground");
const SCROLLBAR_SLIDER_BACKGROUND : Key<Color> = Key::new("scrollbarSlider.background");
const SCROLLBAR_SLIDER_HOVER_BACKGROUND : Key<Color> = Key::new("scrollbarSlider.hoverBackground");
const BADGE_FOREGROUND : Key<Color> = Key::new("badge.foreground");
const BADGE_BACKGROUND : Key<Color> = Key::new("badge.background");
const PROGRESS_BAR_BACKGROUND : Key<Color> = Key::new("progressBar.background");
const LIST_ACTIVE_SELECTION_BACKGROUND : Key<Color> = Key::new("list.activeSelectionBackground");
const LIST_ACTIVE_SELECTION_FOREGROUND : Key<Color> = Key::new("list.activeSelectionForeground");
const LIST_INACTIVE_SELECTION_BACKGROUND : Key<Color> = Key::new("list.inactiveSelectionBackground");
const LIST_INACTIVE_SELECTION_FOREGROUND : Key<Color> = Key::new("list.inactiveSelectionForeground");
const LIST_HOVER_FOREGROUND : Key<Color> = Key::new("list.hoverForeground");
const LIST_FOCUS_FOREGROUND : Key<Color> = Key::new("list.focusForeground");
const LIST_FOCUS_BACKGROUND : Key<Color> = Key::new("list.focusBackground");
const LIST_HOVER_BACKGROUND : Key<Color> = Key::new("list.hoverBackground");
const LIST_DROP_BACKGROUND : Key<Color> = Key::new("list.dropBackground");
const LIST_HIGHLIGHT_FOREGROUND : Key<Color> = Key::new("list.highlightForeground");
const LIST_ERROR_FOREGROUND : Key<Color> = Key::new("list.errorForeground");
const LIST_WARNING_FOREGROUND : Key<Color> = Key::new("list.warningForeground");
const ACTIVITY_BAR_BACKGROUND : Key<Color> = Key::new("activityBar.background");
const ACTIVITY_BAR_DROP_BACKGROUND : Key<Color> = Key::new("activityBar.dropBackground");
const ACTIVITY_BAR_FOREGROUND : Key<Color> = Key::new("activityBar.foreground");
const ACTIVITY_BAR_BADGE_BACKGROUND : Key<Color> = Key::new("activityBarBadge.background");
const ACTIVITY_BAR_BADGE_FOREGROUND : Key<Color> = Key::new("activityBarBadge.foreground");
const SIDE_BAR_BACKGROUND : Key<Color> = Key::new("sideBar.background");
const SIDE_BAR_FOREGROUND : Key<Color> = Key::new("sideBar.foreground");
const SIDE_BAR_SECTION_HEADER_BACKGROUND : Key<Color> = Key::new("sideBarSectionHeader.background");
const SIDE_BAR_SECTION_HEADER_FOREGROUND : Key<Color> = Key::new("sideBarSectionHeader.foreground");
const SIDE_BAR_TITLE_FOREGROUND : Key<Color> = Key::new("sideBarTitle.foreground");
const EDITOR_GROUP_BORDER : Key<Color> = Key::new("editorGroup.border");
const EDITOR_GROUP_DROP_BACKGROUND : Key<Color> = Key::new("editorGroup.dropBackground");
const EDITOR_GROUP_HEADER_NO_TABS_BACKGROUND : Key<Color> = Key::new("editorGroupHeader.noTabsBackground");
const EDITOR_GROUP_HEADER_TABS_BACKGROUND : Key<Color> = Key::new("editorGroupHeader.tabsBackground");
const TAB_ACTIVE_BACKGROUND : Key<Color> = Key::new("tab.activeBackground");
const TAB_ACTIVE_FOREGROUND : Key<Color> = Key::new("tab.activeForeground");
const TAB_BORDER : Key<Color> = Key::new("tab.border");
const TAB_ACTIVE_BORDER : Key<Color> = Key::new("tab.activeBorder");
const TAB_UNFOCUSED_ACTIVE_BORDER : Key<Color> = Key::new("tab.unfocusedActiveBorder");
const TAB_INACTIVE_BACKGROUND : Key<Color> = Key::new("tab.inactiveBackground");
const TAB_INACTIVE_FOREGROUND : Key<Color> = Key::new("tab.inactiveForeground");
const TAB_UNFOCUSED_ACTIVE_FOREGROUND : Key<Color> = Key::new("tab.unfocusedActiveForeground");
const TAB_UNFOCUSED_INACTIVE_FOREGROUND : Key<Color> = Key::new("tab.unfocusedInactiveForeground");
const EDITOR_BACKGROUND : Key<Color> = Key::new("editor.background");
const EDITOR_FOREGROUND : Key<Color> = Key::new("editor.foreground");
const EDITOR_HOVER_HIGHLIGHT_BACKGROUND : Key<Color> = Key::new("editor.hoverHighlightBackground");
const EDITOR_FIND_MATCH_BACKGROUND : Key<Color> = Key::new("editor.findMatchBackground");
const EDITOR_FIND_MATCH_HIGHLIGHT_BACKGROUND : Key<Color> = Key::new("editor.findMatchHighlightBackground");
const EDITOR_FIND_RANGE_HIGHLIGHT_BACKGROUND : Key<Color> = Key::new("editor.findRangeHighlightBackground");
const EDITOR_LINE_HIGHLIGHT_BACKGROUND : Key<Color> = Key::new("editor.lineHighlightBackground");
const EDITOR_LINE_HIGHLIGHT_BORDER : Key<Color> = Key::new("editor.lineHighlightBorder");
const EDITOR_INACTIVE_SELECTION_BACKGROUND : Key<Color> = Key::new("editor.inactiveSelectionBackground");
const EDITOR_SELECTION_BACKGROUND : Key<Color> = Key::new("editor.selectionBackground");
const EDITOR_SELECTION_HIGHLIGHT_BACKGROUND : Key<Color> = Key::new("editor.selectionHighlightBackground");
const EDITOR_RANGE_HIGHLIGHT_BACKGROUND : Key<Color> = Key::new("editor.rangeHighlightBackground");
const EDITOR_WORD_HIGHLIGHT_BACKGROUND : Key<Color> = Key::new("editor.wordHighlightBackground");
const EDITOR_WORD_HIGHLIGHT_STRONG_BACKGROUND : Key<Color> = Key::new("editor.wordHighlightStrongBackground");
const EDITOR_ERROR_FOREGROUND : Key<Color> = Key::new("editorError.foreground");
const EDITOR_ERROR_BORDER : Key<Color> = Key::new("editorError.border");
const EDITOR_WARNING_FOREGROUND : Key<Color> = Key::new("editorWarning.foreground");
const EDITOR_INFO_FOREGROUND : Key<Color> = Key::new("editorInfo.foreground");
const EDITOR_WARNING_BORDER : Key<Color> = Key::new("editorWarning.border");
const EDITOR_CURSOR_FOREGROUND : Key<Color> = Key::new("editorCursor.foreground");
const EDITOR_INDENT_GUIDE_BACKGROUND : Key<Color> = Key::new("editorIndentGuide.background");
const EDITOR_LINE_NUMBER_FOREGROUND : Key<Color> = Key::new("editorLineNumber.foreground");
const EDITOR_WHITESPACE_FOREGROUND : Key<Color> = Key::new("editorWhitespace.foreground");
const EDITOR_OVERVIEW_RULER_BORDER : Key<Color> = Key::new("editorOverviewRuler.border");
const EDITOR_OVERVIEW_RULER_CURRENT_CONTENT_FOREGROUND : Key<Color> = Key::new("editorOverviewRuler.currentContentForeground");
const EDITOR_OVERVIEW_RULER_INCOMING_CONTENT_FOREGROUND : Key<Color> = Key::new("editorOverviewRuler.incomingContentForeground");
const EDITOR_OVERVIEW_RULER_FIND_MATCH_FOREGROUND : Key<Color> = Key::new("editorOverviewRuler.findMatchForeground");
const EDITOR_OVERVIEW_RULER_RANGE_HIGHLIGHT_FOREGROUND : Key<Color> = Key::new("editorOverviewRuler.rangeHighlightForeground");
const EDITOR_OVERVIEW_RULER_SELECTION_HIGHLIGHT_FOREGROUND : Key<Color> = Key::new("editorOverviewRuler.selectionHighlightForeground");
const EDITOR_OVERVIEW_RULER_WORD_HIGHLIGHT_FOREGROUND : Key<Color> = Key::new("editorOverviewRuler.wordHighlightForeground");
const EDITOR_OVERVIEW_RULER_WORD_HIGHLIGHT_STRONG_FOREGROUND : Key<Color> = Key::new("editorOverviewRuler.wordHighlightStrongForeground");
const EDITOR_OVERVIEW_RULER_MODIFIED_FOREGRUND : Key<Color> = Key::new("editorOverviewRuler.modifiedForeground");
const EDITOR_OVERVIEW_RULER_ADDED_FOREGROUND : Key<Color> = Key::new("editorOverviewRuler.addedForeground");
const EDITOR_OVERVIEW_RULER_DELETED_FOREGROUND : Key<Color> = Key::new("editorOverviewRuler.deletedForeground");
const EDITOR_OVERVIEW_RULER_ERROR_FOREGROUND : Key<Color> = Key::new("editorOverviewRuler.errorForeground");
const EDITOR_OVERVIEW_RULER_WARNING_FOREGROUND : Key<Color> = Key::new("editorOverviewRuler.warningForeground");
const EDITOR_OVERVIEW_RULER_INFO_FOREGROUND : Key<Color> = Key::new("editorOverviewRuler.infoForeground");
const EDITOR_OVERVIEW_RULER_BRACKET_MATCH_FOREGROUND : Key<Color> = Key::new("editorOverviewRuler.bracketMatchForeground");
const EDITOR_GUTTER_MODIFIED_BACKGROUND : Key<Color> = Key::new("editorGutter.modifiedBackground");
const EDITOR_GUTTER_ADDED_BACKGROUND : Key<Color> = Key::new("editorGutter.addedBackground");
const EDITOR_GUTTER_DELETED_BACKGROUND : Key<Color> = Key::new("editorGutter.deletedBackground");
const DIFF_EDITOR_INSERTED_TEXT_BACKGROUND : Key<Color> = Key::new("diffEditor.insertedTextBackground");
const DIFF_EDITOR_REMOVED_TEXT_BACKGROUND : Key<Color> = Key::new("diffEditor.removedTextBackground");
const EDITOR_WIDGET_BACKGROUND : Key<Color> = Key::new("editorWidget.background");
const EDITOR_WIDGET_BORDER : Key<Color> = Key::new("editorWidget.border");
const EDITOR_SUGGEST_WIDGET_BACKGROUND : Key<Color> = Key::new("editorSuggestWidget.background");
const PEEK_VIEW_BORDER : Key<Color> = Key::new("peekView.border");
const PEEK_VIEW_EDITOR_MATCH_HIGHLIGHT_BACKGROUND : Key<Color> = Key::new("peekViewEditor.matchHighlightBackground");
const PEEK_VIEW_EDITOR_GUTTER_BACKGROUND : Key<Color> = Key::new("peekViewEditorGutter.background");
const PEEK_VIEW_EDITOR_BACKGROUND : Key<Color> = Key::new("peekViewEditor.background");
const PEEK_VIEW_RESULT_BACKGROUND : Key<Color> = Key::new("peekViewResult.background");
const PEEK_VIEW_TITLE_BACKGROUND : Key<Color> = Key::new("peekViewTitle.background");
const MERGE_CURRENT_HEADER_BACKGROUND : Key<Color> = Key::new("merge.currentHeaderBackground");
const MERGE_CURRENT_CONTENT_BACKGROUND : Key<Color> = Key::new("merge.currentContentBackground");
const MERGE_INCOMING_HEADER_BACKGROUND : Key<Color> = Key::new("merge.incomingHeaderBackground");
const MERGE_INCOMING_CONTENT_BACKGROUND : Key<Color> = Key::new("merge.incomingContentBackground");
const PANEL_BACKGROUND : Key<Color> = Key::new("panel.background");
const PANEL_BORDER : Key<Color> = Key::new("panel.border");
const PANEL_TITLE_ACTIVE_BORDER : Key<Color> = Key::new("panelTitle.activeBorder");
const STATUS_BAR_BACKGROUND : Key<Color> = Key::new("statusBar.background");
const STATUS_BAR_DEBUGGING_BACKGROUND : Key<Color> = Key::new("statusBar.debuggingBackground");
const STATUS_BAR_DEBUGGING_FOREGROUND : Key<Color> = Key::new("statusBar.debuggingForeground");
const STATUS_BAR_NO_FOLDER_FOREGROUND : Key<Color> = Key::new("statusBar.noFolderForeground");
const STATUS_BAR_NO_FOLDER_BACKGROUND : Key<Color> = Key::new("statusBar.noFolderBackground");
const STATUS_BAR_FOREGROUND : Key<Color> = Key::new("statusBar.foreground");
const STATUS_BAR_ITEM_ACTIVE_BACKGROUND : Key<Color> = Key::new("statusBarItem.activeBackground");
const STATUS_BAR_ITEM_HOVER_BACKGROUND : Key<Color> = Key::new("statusBarItem.hoverBackground");
const STATUS_BAR_ITEM_PROMINENT_BACKGROUND : Key<Color> = Key::new("statusBarItem.prominentBackground");
const STATUS_BAR_ITEM_PROMINENT_HOVER_BACKGROUND : Key<Color> = Key::new("statusBarItem.prominentHoverBackground");
const STATUS_BAR_BORDER : Key<Color> = Key::new("statusBar.border");
const TITLE_BAR_ACTIVE_BACKGROUND : Key<Color> = Key::new("titleBar.activeBackground");
const TITLE_BAR_ACTIVE_FOREGROUND : Key<Color> = Key::new("titleBar.activeForeground");
const TITLE_BAR_INACTIVE_BACKGROUND : Key<Color> = Key::new("titleBar.inactiveBackground");
const TITLE_BAR_INACTIVE_FOREGROUND : Key<Color> = Key::new("titleBar.inactiveForeground");
const NOTIFICATION_CENTER_HEADER_FOREGROUND : Key<Color> = Key::new("notificationCenterHeader.foreground");
const NOTIFICATION_CENTER_HEADER_BACKGROUND : Key<Color> = Key::new("notificationCenterHeader.background");
const EXTENSION_BUTTON_PROMINENT_FOREGROUND : Key<Color> = Key::new("extensionButton.prominentForeground");
const EXTENSION_BUTTON_PROMINENT_BACKGROUND : Key<Color> = Key::new("extensionButton.prominentBackground");
const EXTENSION_BUTTON_PROMINENT_HOVER_BACKGROUND : Key<Color> = Key::new("extensionButton.prominentHoverBackground");
const PICKER_GROUP_BORDER : Key<Color> = Key::new("pickerGroup.border");
const PICKER_GROUP_FOREGROUND : Key<Color> = Key::new("pickerGroup.foreground");
const TERMINAL_ANSI_BRIGHT_BLACK : Key<Color> = Key::new("terminal.ansiBrightBlack");
const TERMINAL_ANSI_BLACK : Key<Color> = Key::new("terminal.ansiBlack");
const TERMINAL_ANSI_BLUE : Key<Color> = Key::new("terminal.ansiBlue");
const TERMINAL_ANSI_BRIGHT_BLUE : Key<Color> = Key::new("terminal.ansiBrightBlue");
const TERMINAL_ANSI_BRIGHT_CYAN : Key<Color> = Key::new("terminal.ansiBrightCyan");
const TERMINAL_ANSI_CYAN : Key<Color> = Key::new("terminal.ansiCyan");
const TERMINAL_ANSI_BRIGHT_MAGENTA : Key<Color> = Key::new("terminal.ansiBrightMagenta");
const TERMINAL_ANSI_MAGENTA : Key<Color> = Key::new("terminal.ansiMagenta");
const TERMINAL_ANSI_BRIGHT_RED : Key<Color> = Key::new("terminal.ansiBrightRed");
const TERMINAL_ANSI_RED : Key<Color> = Key::new("terminal.ansiRed");
const TERMINAL_ANSI_YELLOW : Key<Color> = Key::new("terminal.ansiYellow");
const TERMINAL_ANSI_BRIGHT_YELLOW : Key<Color> = Key::new("terminal.ansiBrightYellow");
const TERMINAL_ANSI_BRIGHT_GREEN : Key<Color> = Key::new("terminal.ansiBrightGreen");
const TERMINAL_ANSI_GREEN : Key<Color> = Key::new("terminal.ansiGreen");
const TERMINAL_ANSI_WHITE : Key<Color> = Key::new("terminal.ansiWhite");
const TERMINAL_SELECTION_BACKGROUND : Key<Color> = Key::new("terminal.selectionBackground");
const TERMINAL_CURSOR_BACKGROUND : Key<Color> = Key::new("terminalCursor.background");
const TERMINAL_CURSOR_FOREGROUND : Key<Color> = Key::new("terminalCursor.foreground");
const GIT_DECORATION_MODIFIED_RESOURCE_FOREGROUND : Key<Color> = Key::new("gitDecoration.modifiedResourceForeground");
const GIT_DECORATION_DELETED_RESOURCE_FOREGROUND : Key<Color> = Key::new("gitDecoration.deletedResourceForeground");
const GIT_DECORATION_UNTRACKED_RESOURCE_FOREGROUND : Key<Color> = Key::new("gitDecoration.untrackedResourceForeground");
const GIT_DECORATION_CONFLICTING_RESOURCE_FOREGROUND : Key<Color> = Key::new("gitDecoration.conflictingResourceForeground");
const GIT_DECORATION_SUBMODULE_RESOURCE_FOREGROUND : Key<Color> = Key::new("gitDecoration.submoduleResourceForeground");

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