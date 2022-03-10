use std::{fmt, marker::PhantomData};

#[rustfmt::skip]

use serde::{de, Deserialize, Deserializer, Serialize};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Colors {
    #[serde(rename = "focusBorder")]
    pub focus_border: String,
    #[serde(rename = "foreground")]
    pub foreground: String,
    #[serde(rename = "selection.background")]
    pub selection_background: String,
    #[serde(rename = "widget.shadow")]
    pub widget_shadow: String,
    #[serde(rename = "textLink.activeForeground")]
    pub text_link_active_foreground: String,
    #[serde(rename = "textLink.foreground")]
    pub text_link_foreground: String,
    #[serde(rename = "textPreformat.foreground")]
    pub text_preformat_foreground: String,
    #[serde(rename = "button.background")]
    pub button_background: String,
    #[serde(rename = "button.foreground")]
    pub button_foreground: String,
    #[serde(rename = "button.hoverBackground")]
    pub button_hover_background: String,
    #[serde(rename = "dropdown.background")]
    pub dropdown_background: String,
    #[serde(rename = "dropdown.listBackground")]
    pub dropdown_list_background: String,
    #[serde(rename = "input.background")]
    pub input_background: String,
    #[serde(rename = "input.border")]
    pub input_border: String,
    #[serde(rename = "input.foreground")]
    pub input_foreground: String,
    #[serde(rename = "input.placeholderForeground")]
    pub input_placeholder_foreground: String,
    #[serde(rename = "scrollbar.shadow")]
    pub scrollbar_shadow: String,
    #[serde(rename = "scrollbarSlider.activeBackground")]
    pub scrollbar_slider_active_background: String,
    #[serde(rename = "scrollbarSlider.background")]
    pub scrollbar_slider_background: String,
    #[serde(rename = "scrollbarSlider.hoverBackground")]
    pub scrollbar_slider_hover_background: String,
    #[serde(rename = "badge.foreground")]
    pub badge_foreground: String,
    #[serde(rename = "badge.background")]
    pub badge_background: String,
    #[serde(rename = "progressBar.background")]
    pub progress_bar_background: String,
    #[serde(rename = "list.activeSelectionBackground")]
    pub list_active_selection_background: String,
    #[serde(rename = "list.activeSelectionForeground")]
    pub list_active_selection_foreground: String,
    #[serde(rename = "list.inactiveSelectionBackground")]
    pub list_inactive_selection_background: String,
    #[serde(rename = "list.inactiveSelectionForeground")]
    pub list_inactive_selection_foreground: String,
    #[serde(rename = "list.hoverForeground")]
    pub list_hover_foreground: String,
    #[serde(rename = "list.focusForeground")]
    pub list_focus_foreground: String,
    #[serde(rename = "list.focusBackground")]
    pub list_focus_background: String,
    #[serde(rename = "list.hoverBackground")]
    pub list_hover_background: String,
    #[serde(rename = "list.dropBackground")]
    pub list_drop_background: String,
    #[serde(rename = "list.highlightForeground")]
    pub list_highlight_foreground: String,
    #[serde(rename = "list.errorForeground")]
    pub list_error_foreground: String,
    #[serde(rename = "list.warningForeground")]
    pub list_warning_foreground: String,
    #[serde(rename = "activityBar.background")]
    pub activity_bar_background: String,
    #[serde(rename = "activityBar.dropBackground")]
    pub activity_bar_drop_background: String,
    #[serde(rename = "activityBar.foreground")]
    pub activity_bar_foreground: String,
    #[serde(rename = "activityBarBadge.background")]
    pub activity_bar_badge_background: String,
    #[serde(rename = "activityBarBadge.foreground")]
    pub activity_bar_badge_foreground: String,
    #[serde(rename = "sideBar.background")]
    pub side_bar_background: String,
    #[serde(rename = "sideBar.foreground")]
    pub side_bar_foreground: String,
    #[serde(rename = "sideBarSectionHeader.background")]
    pub side_bar_section_header_background: String,
    #[serde(rename = "sideBarSectionHeader.foreground")]
    pub side_bar_section_header_foreground: String,
    #[serde(rename = "sideBarTitle.foreground")]
    pub side_bar_title_foreground: String,
    #[serde(rename = "editorGroup.border")]
    pub editor_group_border: String,
    #[serde(rename = "editorGroup.dropBackground")]
    pub editor_group_drop_background: String,
    #[serde(rename = "editorGroupHeader.noTabsBackground")]
    pub editor_group_header_no_tabs_background: String,
    #[serde(rename = "editorGroupHeader.tabsBackground")]
    pub editor_group_header_tabs_background: String,
    #[serde(rename = "tab.activeBackground")]
    pub tab_active_background: String,
    #[serde(rename = "tab.activeForeground")]
    pub tab_active_foreground: String,
    #[serde(rename = "tab.border")]
    pub tab_border: String,
    #[serde(rename = "tab.activeBorder")]
    pub tab_active_border: String,
    #[serde(rename = "tab.unfocusedActiveBorder")]
    pub tab_unfocused_active_border: String,
    #[serde(rename = "tab.inactiveBackground")]
    pub tab_inactive_background: String,
    #[serde(rename = "tab.inactiveForeground")]
    pub tab_inactive_foreground: String,
    #[serde(rename = "tab.unfocusedActiveForeground")]
    pub tab_unfocused_active_foreground: String,
    #[serde(rename = "tab.unfocusedInactiveForeground")]
    pub tab_unfocused_inactive_foreground: String,
    #[serde(rename = "editor.background")]
    pub editor_background: String,
    #[serde(rename = "editor.foreground")]
    pub editor_foreground: String,
    #[serde(rename = "editor.hoverHighlightBackground")]
    pub editor_hover_highlight_background: String,
    #[serde(rename = "editor.findMatchBackground")]
    pub editor_find_match_background: String,
    #[serde(rename = "editor.findMatchHighlightBackground")]
    pub editor_find_match_highlight_background: String,
    #[serde(rename = "editor.findRangeHighlightBackground")]
    pub editor_find_range_highlight_background: String,
    #[serde(rename = "editor.lineHighlightBackground")]
    pub editor_line_highlight_background: String,
    #[serde(rename = "editor.lineHighlightBorder")]
    pub editor_line_highlight_border: String,
    #[serde(rename = "editor.inactiveSelectionBackground")]
    pub editor_inactive_selection_background: String,
    #[serde(rename = "editor.selectionBackground")]
    pub editor_selection_background: String,
    #[serde(rename = "editor.selectionHighlightBackground")]
    pub editor_selection_highlight_background: String,
    #[serde(rename = "editor.rangeHighlightBackground")]
    pub editor_range_highlight_background: String,
    #[serde(rename = "editor.wordHighlightBackground")]
    pub editor_word_highlight_background: String,
    #[serde(rename = "editor.wordHighlightStrongBackground")]
    pub editor_word_highlight_strong_background: String,
    #[serde(rename = "editorError.foreground")]
    pub editor_error_foreground: String,
    #[serde(rename = "editorError.border")]
    pub editor_error_border: String,
    #[serde(rename = "editorWarning.foreground")]
    pub editor_warning_foreground: String,
    #[serde(rename = "editorInfo.foreground")]
    pub editor_info_foreground: String,
    #[serde(rename = "editorWarning.border")]
    pub editor_warning_border: String,
    #[serde(rename = "editorCursor.foreground")]
    pub editor_cursor_foreground: String,
    #[serde(rename = "editorIndentGuide.background")]
    pub editor_indent_guide_background: String,
    #[serde(rename = "editorLineNumber.foreground")]
    pub editor_line_number_foreground: String,
    #[serde(rename = "editorWhitespace.foreground")]
    pub editor_whitespace_foreground: String,
    #[serde(rename = "editorOverviewRuler.border")]
    pub editor_overview_ruler_border: String,
    #[serde(rename = "editorOverviewRuler.currentContentForeground")]
    pub editor_overview_ruler_current_content_foreground: String,
    #[serde(rename = "editorOverviewRuler.incomingContentForeground")]
    pub editor_overview_ruler_incoming_content_foreground: String,
    #[serde(rename = "editorOverviewRuler.findMatchForeground")]
    pub editor_overview_ruler_find_match_foreground: String,
    #[serde(rename = "editorOverviewRuler.rangeHighlightForeground")]
    pub editor_overview_ruler_range_highlight_foreground: String,
    #[serde(rename = "editorOverviewRuler.selectionHighlightForeground")]
    pub editor_overview_ruler_selection_highlight_foreground: String,
    #[serde(rename = "editorOverviewRuler.wordHighlightForeground")]
    pub editor_overview_ruler_word_highlight_foreground: String,
    #[serde(rename = "editorOverviewRuler.wordHighlightStrongForeground")]
    pub editor_overview_ruler_word_highlight_strong_foreground: String,
    #[serde(rename = "editorOverviewRuler.modifiedForeground")]
    pub editor_overview_ruler_modified_foregrund: String,
    #[serde(rename = "editorOverviewRuler.addedForeground")]
    pub editor_overview_ruler_added_foreground: String,
    #[serde(rename = "editorOverviewRuler.deletedForeground")]
    pub editor_overview_ruler_deleted_foreground: String,
    #[serde(rename = "editorOverviewRuler.errorForeground")]
    pub editor_overview_ruler_error_foreground: String,
    #[serde(rename = "editorOverviewRuler.warningForeground")]
    pub editor_overview_ruler_warning_foreground: String,
    #[serde(rename = "editorOverviewRuler.infoForeground")]
    pub editor_overview_ruler_info_foreground: String,
    #[serde(rename = "editorOverviewRuler.bracketMatchForeground")]
    pub editor_overview_ruler_bracket_match_foreground: String,
    #[serde(rename = "editorGutter.modifiedBackground")]
    pub editor_gutter_modified_background: String,
    #[serde(rename = "editorGutter.addedBackground")]
    pub editor_gutter_added_background: String,
    #[serde(rename = "editorGutter.deletedBackground")]
    pub editor_gutter_deleted_background: String,
    #[serde(rename = "diffEditor.insertedTextBackground")]
    pub diff_editor_inserted_text_background: String,
    #[serde(rename = "diffEditor.removedTextBackground")]
    pub diff_editor_removed_text_background: String,
    #[serde(rename = "editorWidget.background")]
    pub editor_widget_background: String,
    #[serde(rename = "editorWidget.border")]
    pub editor_widget_border: String,
    #[serde(rename = "editorSuggestWidget.background")]
    pub editor_suggest_widget_background: String,
    #[serde(rename = "peekView.border")]
    pub peek_view_border: String,
    #[serde(rename = "peekViewEditor.matchHighlightBackground")]
    pub peek_view_editor_match_highlight_background: String,
    #[serde(rename = "peekViewEditorGutter.background")]
    pub peek_view_editor_gutter_background: String,
    #[serde(rename = "peekViewEditor.background")]
    pub peek_view_editor_background: String,
    #[serde(rename = "peekViewResult.background")]
    pub peek_view_result_background: String,
    #[serde(rename = "peekViewTitle.background")]
    pub peek_view_title_background: String,
    #[serde(rename = "merge.currentHeaderBackground")]
    pub merge_current_header_background: String,
    #[serde(rename = "merge.currentContentBackground")]
    pub merge_current_content_background: String,
    #[serde(rename = "merge.incomingHeaderBackground")]
    pub merge_incoming_header_background: String,
    #[serde(rename = "merge.incomingContentBackground")]
    pub merge_incoming_content_background: String,
    #[serde(rename = "panel.background")]
    pub panel_background: String,
    #[serde(rename = "panel.border")]
    pub panel_border: String,
    #[serde(rename = "panelTitle.activeBorder")]
    pub panel_title_active_border: String,
    #[serde(rename = "statusBar.background")]
    pub status_bar_background: String,
    #[serde(rename = "statusBar.debuggingBackground")]
    pub status_bar_debugging_background: String,
    #[serde(rename = "statusBar.debuggingForeground")]
    pub status_bar_debugging_foreground: String,
    #[serde(rename = "statusBar.noFolderForeground")]
    pub status_bar_no_folder_foreground: String,
    #[serde(rename = "statusBar.noFolderBackground")]
    pub status_bar_no_folder_background: String,
    #[serde(rename = "statusBar.foreground")]
    pub status_bar_foreground: String,
    #[serde(rename = "statusBarItem.activeBackground")]
    pub status_bar_item_active_background: String,
    #[serde(rename = "statusBarItem.hoverBackground")]
    pub status_bar_item_hover_background: String,
    #[serde(rename = "statusBarItem.prominentBackground")]
    pub status_bar_item_prominent_background: String,
    #[serde(rename = "statusBarItem.prominentHoverBackground")]
    pub status_bar_item_prominent_hover_background: String,
    #[serde(rename = "statusBar.border")]
    pub status_bar_border: String,
    #[serde(rename = "titleBar.activeBackground")]
    pub title_bar_active_background: String,
    #[serde(rename = "titleBar.activeForeground")]
    pub title_bar_active_foreground: String,
    #[serde(rename = "titleBar.inactiveBackground")]
    pub title_bar_inactive_background: String,
    #[serde(rename = "titleBar.inactiveForeground")]
    pub title_bar_inactive_foreground: String,
    #[serde(rename = "notificationCenterHeader.foreground")]
    pub notification_center_header_foreground: String,
    #[serde(rename = "notificationCenterHeader.background")]
    pub notification_center_header_background: String,
    #[serde(rename = "extensionButton.prominentForeground")]
    pub extension_button_prominent_foreground: String,
    #[serde(rename = "extensionButton.prominentBackground")]
    pub extension_button_prominent_background: String,
    #[serde(rename = "extensionButton.prominentHoverBackground")]
    pub extension_button_prominent_hover_background: String,
    #[serde(rename = "pickerGroup.border")]
    pub picker_group_border: String,
    #[serde(rename = "pickerGroup.foreground")]
    pub picker_group_foreground: String,
    #[serde(rename = "terminal.ansiBrightBlack")]
    pub terminal_ansi_bright_black: String,
    #[serde(rename = "terminal.ansiBlack")]
    pub terminal_ansi_black: String,
    #[serde(rename = "terminal.ansiBlue")]
    pub terminal_ansi_blue: String,
    #[serde(rename = "terminal.ansiBrightBlue")]
    pub terminal_ansi_bright_blue: String,
    #[serde(rename = "terminal.ansiBrightCyan")]
    pub terminal_ansi_bright_cyan: String,
    #[serde(rename = "terminal.ansiCyan")]
    pub terminal_ansi_cyan: String,
    #[serde(rename = "terminal.ansiBrightMagenta")]
    pub terminal_ansi_bright_magenta: String,
    #[serde(rename = "terminal.ansiMagenta")]
    pub terminal_ansi_magenta: String,
    #[serde(rename = "terminal.ansiBrightRed")]
    pub terminal_ansi_bright_red: String,
    #[serde(rename = "terminal.ansiRed")]
    pub terminal_ansi_red: String,
    #[serde(rename = "terminal.ansiYellow")]
    pub terminal_ansi_yellow: String,
    #[serde(rename = "terminal.ansiBrightYellow")]
    pub terminal_ansi_bright_yellow: String,
    #[serde(rename = "terminal.ansiBrightGreen")]
    pub terminal_ansi_bright_green: String,
    #[serde(rename = "terminal.ansiGreen")]
    pub terminal_ansi_green: String,
    #[serde(rename = "terminal.ansiWhite")]
    pub terminal_ansi_white: String,
    #[serde(rename = "terminal.selectionBackground")]
    pub terminal_selection_background: String,
    #[serde(rename = "terminalCursor.background")]
    pub terminal_cursor_background: String,
    #[serde(rename = "terminalCursor.foreground")]
    pub terminal_cursor_foreground: String,
    #[serde(rename = "gitDecoration.modifiedResourceForeground")]
    pub git_decoration_modified_resource_foreground: String,
    #[serde(rename = "gitDecoration.deletedResourceForeground")]
    pub git_decoration_deleted_resource_foreground: String,
    #[serde(rename = "gitDecoration.untrackedResourceForeground")]
    pub git_decoration_untracked_resource_foreground: String,
    #[serde(rename = "gitDecoration.conflictingResourceForeground")]
    pub git_decoration_conflicting_resource_foreground: String,
    #[serde(rename = "gitDecoration.submoduleResourceForeground")]
    pub git_decoration_submodule_resource_foreground: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenSetting {
    pub foreground: Option<String>,
    #[serde(rename = "fontStyle")]
    pub font_style: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TokenColors {
    pub name: Option<String>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pub scope: Vec<String>,
    pub settings: TokenSetting,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VSCodeTheme {
    pub name: String,
    #[serde(rename = "type")]
    pub theme_type: String,
    pub colors: Colors,
    #[serde(rename = "tokenColors")]
    pub token_colors: Vec<TokenColors>,
}


impl Default for VSCodeTheme {
    fn default() -> Self {
        let s = include_str!("themes/mariana.json");
        serde_json::from_str(&s).unwrap()
    }
}

fn string_or_seq_string<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrVec(PhantomData<Vec<String>>);

    impl<'de> de::Visitor<'de> for StringOrVec {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or list of strings")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(vec![value.to_owned()])
        }

        fn visit_seq<S>(self, visitor: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            Deserialize::deserialize(de::value::SeqAccessDeserializer::new(visitor))
        }
    }

    deserializer.deserialize_any(StringOrVec(PhantomData))
}
