use std::borrow::Borrow;

use druid::{
    commands, im::Vector, Command, EventCtx, FileDialogOptions, HotKey, KeyEvent, Selector, SysMods, Target, WidgetId,
};
use once_cell::sync::Lazy;
use rfd::MessageDialog;

use crate::{
    editor_view::EditorView,
    text_buffer::EditStack,
    widgets::{Item, PaletteListState},
};

pub const SHOW_SEARCH_PANEL: Selector<String> = Selector::new("nonepad.bottom_panel.show_search");
pub const SHOW_PALETTE_PANEL: Selector<(WidgetId, Vector<Item>, Selector<usize>)> =
    Selector::new("nonepad.bottom_panel.show_palette");
pub const SEND_PALETTE_PANEL_DATA: Selector<(WidgetId, Vector<Item>, Selector<usize>)> =
    Selector::new("nonepad.bottom_panel.show_palette_data");
pub const SEND_STRING_DATA: Selector<String> = Selector::new("nonepad.all.send_data");
pub const CLOSE_BOTTOM_PANEL: Selector<()> = Selector::new("nonepad.bottom_panel.close");
pub const REQUEST_CLOSE_BOTTOM_PANEL: Selector<()> = Selector::new("nonepad.bottom_panel.request_close");
pub const RESET_HELD_STATE: Selector<()> = Selector::new("nonepad.all.reste_held_state");
pub const REQUEST_NEXT_SEARCH: Selector<String> = Selector::new("nonepad.editor.request_next_search");
pub const GIVE_FOCUS: Selector<()> = Selector::new("nonepad.all.give_focus");
pub const SELECT_LINE: Selector<(usize, bool)> = Selector::new("nonepad.editor.select_line");
pub const SCROLL_TO: Selector<(Option<f64>, Option<f64>)> = Selector::new("nonepad.editor.scroll_to_rect");
pub const HIGHLIGHT: Selector<(usize, usize)> = Selector::new("nonepad.editor.highlight");

pub const PALETTE_EXECUTE_COMMAND: Selector<usize> = Selector::new("nonepad.palette.execute_command");

pub trait UICmd {
    fn matches(&self, event: &KeyEvent) -> bool;
}

pub struct UICommand {
    pub description: String,
    pub show_in_palette: bool,
    shortcut: Option<druid::HotKey>,
    exec: fn(&mut EditorView, &mut EventCtx, &mut EditStack) -> bool,
}

impl UICmd for UICommand {
    fn matches(&self, event: &KeyEvent) -> bool {
        self.shortcut.clone().map(|s| s.matches(event)).unwrap_or(false)
    }
}

pub struct UICommandSet {
    pub commands: Vec<UICommand>,
}

impl UICommandSet {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }

    pub fn hotkey_submit(
        &self,
        ctx: &mut EventCtx,
        event: impl Borrow<KeyEvent>,
        editor_view: &mut EditorView,
        editor: &mut EditStack,
    ) {
        for c in &COMMANDSET.commands {
            if c.matches(event.borrow()) {
                (c.exec)(editor_view, ctx, editor);
                //c.submit(ctx)
            }
        }
    }
}

fn string_to_hotkey(input: &str) -> Option<HotKey> {
    let t: Vec<&str> = input.split('-').collect();
    if t.len() != 2 {
        return None;
    }
    let mods = match t[0] {
        "Ctrl" => SysMods::Cmd,
        "CtrlShift" => SysMods::CmdShift,
        "CtrlAlt" => SysMods::AltCmd,
        "Shift" => SysMods::Shift,
        "CtrlAltShift" => SysMods::AltCmdShift,
        _ => SysMods::None,
    };
    if t[0].contains("Shift") {
        Some(HotKey::new(mods, t[1].to_uppercase().as_str()))
    } else {
        Some(HotKey::new(mods, t[1]))
    }
}

// pub const EDITOR_VIEW_UICMD: Selector<Box<dyn FnOnce(EditorView, EventCtx, EditStack)>> =
//     Selector::new("nonepad.editor.uicmd");

macro_rules! uicmd {
    ($commandset:ident = { $($command:ident = ($description:literal,$hotkey:literal, $v:expr, $b:expr));+ $(;)? } ) => {
        //$(pub const $command: Selector<()> = Selector::new(stringify!("nonepad.palcmd",$command));)+

        pub static $commandset: Lazy<UICommandSet> = Lazy::new(|| {
            let mut v = UICommandSet::new();
            $(v.commands.push(UICommand::new($description, $v,string_to_hotkey($hotkey), $b ));)+
            v
        });
    };
}

uicmd! {
    COMMANDSET = {
        PALCMD_CHANGE_LANGUAGE = ("Change the language of the file","CtrlShift-l", true,
        |_editor_view, _ctx, editor| {
            dbg!("youhou!");
            editor.tab();
            true
        });
        PALCMD_CHANGE_TYPE_TYPE = ("Change indentation type","", true,
        |_editor_view, _ctx, editor| {
            dbg!("fafa!");
            editor.tab();
            true
        });
        PALCMD_OPEN = ("Open","Ctrl-o", true,
        |_editor_view, ctx, editor| {
            if editor.is_dirty()
            && !MessageDialog::new()
                .set_level(rfd::MessageLevel::Warning)
                .set_title("Are you sure?")
                .set_description("Discard unsaved change?")
                .set_buttons(rfd::MessageButtons::YesNo)
                .show()
            {
                return true;
            }

            let options = FileDialogOptions::new().show_hidden();
            ctx.submit_command(Command::new(druid::commands::SHOW_OPEN_PANEL, options, Target::Auto));
            true
        });
        PALCMD_SAVE = ("Save","Ctrl-s",true,
        |_editor_view, ctx, editor| {
            if editor.filename.is_some() {
                ctx.submit_command(Command::new(druid::commands::SAVE_FILE, (), Target::Auto));
            } else {
                let options = FileDialogOptions::new().show_hidden();
                ctx.submit_command(Command::new(druid::commands::SHOW_SAVE_PANEL, options, Target::Auto))
            }
            return true;
        });
        PALCMD_SAVE_AS = ("Save As","CtrlShift-s",true,
        |_editor_view, ctx, editor| {
            let options = FileDialogOptions::new().show_hidden();
            ctx.submit_command(Command::new(druid::commands::SHOW_SAVE_PANEL, options, Target::Auto));
            return true;
        });
    }
}

pub trait CommandEmmeterCtx {
    fn submit_command(&mut self, cmd: impl Into<Command>);
}

impl CommandEmmeterCtx for EventCtx<'_, '_> {
    fn submit_command(&mut self, cmd: impl Into<Command>) {
        self.submit_command(cmd);
    }
}

pub trait ShowPalette {
    fn show_palette(&mut self, items: Vector<Item>, description: &'static str, callback_cmd: Selector<usize>);
}

impl ShowPalette for EventCtx<'_, '_> {
    fn show_palette(&mut self, items: Vector<Item>, description: &'static str, callback_cmd: Selector<usize>) {
        self.submit_command(Command::new(
            SHOW_PALETTE_PANEL,
            (self.widget_id(), items, callback_cmd),
            Target::Auto,
        ));
    }
}

impl UICommand {
    pub fn new(
        description: &str,
        show_in_palette: bool,
        shortcut: Option<druid::HotKey>,
        exec: fn(&mut EditorView, &mut EventCtx, &mut EditStack) -> bool,
    ) -> Self {
        Self {
            description: description.to_owned(),
            show_in_palette,
            shortcut,
            exec,
        }
    }
    pub fn exec(&self, ctx: &mut EventCtx, editor_view: &mut EditorView, editor: &mut EditStack) {
        (self.exec)(editor_view, ctx, editor);
    }
    // pub fn submit<C: CommandEmmeterCtx>(
    //     &'static self,
    //     editor_view: &mut EditorView,
    //     ctx: &mut C,
    //     editor: &mut EditStack,
    // ) {
    //     // TODO (mpe): Specify target

    //     ctx.submit_command(Command::new(self.selector, (), Target::Auto))
    // }
}
