use std::borrow::Borrow;

use druid::{Command, EventCtx, HotKey, KeyEvent, Selector, SysMods, Target, Widget};
use once_cell::sync::Lazy;

use crate::{editor_view::EditorView, text_buffer::EditStack};

pub const SHOW_SEARCH_PANEL: Selector<String> = Selector::new("nonepad.bottom_panel.show_search");
pub const SEND_DATA: Selector<String> = Selector::new("nonepad.all.send_data");
pub const CLOSE_BOTTOM_PANEL: Selector<()> = Selector::new("nonepad.bottom_panel.close");
pub const RESET_HELD_STATE: Selector<()> = Selector::new("nonepad.all.reste_held_state");
pub const REQUEST_NEXT_SEARCH: Selector<String> = Selector::new("nonepad.editor.request_next_search");
pub const GIVE_FOCUS: Selector<()> = Selector::new("nonepad.all.give_focus");
pub const SELECT_LINE: Selector<(usize, bool)> = Selector::new("nonepad.editor.select_line");
pub const SCROLL_TO: Selector<(Option<f64>, Option<f64>)> = Selector::new("nonepad.editor.scroll_to_rect");


pub trait UICmd {
    fn matches(&self, event: &KeyEvent) -> bool;
}

#[derive(Debug)]
pub struct UICommand {
    description: String,
    selector: Selector<()>,
    shortcut: Option<druid::HotKey>,
}

impl UICmd for UICommand {
    fn matches(&self, event: &KeyEvent) -> bool {
        self.shortcut.clone().map(|s| s.matches(event)).unwrap_or(false)
    }
}

pub struct UICommandSet {
    commands: Vec<UICommand>,
}

impl UICommandSet {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }

    pub fn hotkey_submit(&self, ctx: &mut impl CommandEmmeterCtx, event: impl Borrow<KeyEvent>) {
        for c in &COMMANDSET.commands {
            if c.matches(event.borrow()) {
                c.submit(ctx)
            }
        }
    }
}

fn string_to_hotkey(input: &str) -> Option<HotKey> {
    let t: Vec<&str>= input.split('-').collect();
    if t.len()!=2 {
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
    Some(HotKey::new(mods, t[1]))
}

pub const EDITOR_VIEW_UICMD: Selector<Box<dyn FnOnce(EditorView, EventCtx, EditStack)>> = Selector::new("nonepad.editor.uicmd");

macro_rules! uicmd {
    ($commandset:ident = { $($command:ident = ($description:literal,$hotkey:literal, |$self:ident $(:&mut impl Widget)?, $ctx:ident, $data:ident| $b:expr));+ $(;)? } ) => {
        $(pub const $command: Selector<()> = Selector::new(stringify!("nonepad.palcmd",$command));)+
        
        pub static $commandset: Lazy<UICommandSet> = Lazy::new(|| {
            let mut v = UICommandSet::new();
            $(v.commands.push(UICommand::new($description,$command,string_to_hotkey($hotkey)));)+
            v
        });
    };
}

uicmd! {
    COMMANDSET = {
        PALCMD_CHANGE_LANGUAGE = ("Change the language of the file","Ctrl-l", |_s,_c,d| {});
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

impl UICommand {
    pub fn new(description: &str, selector: Selector, shortcut: Option<druid::HotKey>) -> Self {
        Self {
            description: description.to_owned(),
            selector: selector,
            shortcut,
        }
    }
    pub fn submit<C: CommandEmmeterCtx>(&'static self, ctx: &mut C) {
        // TODO (mpe): Specify target
        ctx.submit_command(Command::new(self.selector, (), Target::Auto))
    }
}
