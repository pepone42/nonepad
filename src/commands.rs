use std::{borrow::Borrow, sync::Arc, rc::Rc};

use druid::{im::Vector, Command, EventCtx, FileDialogOptions, HotKey, KeyEvent, Selector, SysMods, Target, WidgetId};
use once_cell::sync::Lazy;
use rfd::MessageDialog;

use crate::widgets::{
    editor_view::EditorView,
    text_buffer::{syntax::SYNTAXSET, EditStack},
    window::{NPWindow, NPWindowState},
    Item,
};


#[derive(Clone)]
pub enum UICommandType {
    Editor(Rc<dyn Fn(usize, Arc<String>, &mut EventCtx, &mut EditorView, &mut EditStack)>),
    Window(Rc<dyn Fn(usize, Arc<String>, &mut EventCtx, &mut NPWindow, &mut NPWindowState)>),
}

pub const SHOW_SEARCH_PANEL: Selector<String> = Selector::new("nonepad.bottom_panel.show_search");
pub const SHOW_PALETTE_PANEL: Selector<(WidgetId, &str, Vector<Item>, UICommandType)> =
    Selector::new("nonepad.bottom_panel.show_palette");
pub const SEND_STRING_DATA: Selector<String> = Selector::new("nonepad.all.send_data");
pub const CLOSE_BOTTOM_PANEL: Selector<()> = Selector::new("nonepad.bottom_panel.close");
pub const RESET_HELD_STATE: Selector<()> = Selector::new("nonepad.all.reste_held_state");
pub const REQUEST_NEXT_SEARCH: Selector<String> = Selector::new("nonepad.editor.request_next_search");
pub const GIVE_FOCUS: Selector<()> = Selector::new("nonepad.all.give_focus");
pub const SELECT_LINE: Selector<(usize, bool)> = Selector::new("nonepad.editor.select_line");
pub const SCROLL_TO: Selector<(Option<f64>, Option<f64>)> = Selector::new("nonepad.editor.scroll_to_rect");
pub const HIGHLIGHT: Selector<(usize, usize)> = Selector::new("nonepad.editor.highlight");
pub const PALETTE_CALLBACK: Selector<(usize, Arc<String>, UICommandType)> =
    Selector::new("nonepad.editor.execute_command");
pub const CLOSE_PALETTE: Selector<()> = Selector::new("nonepad.palette.close");
pub struct UICommand {
    pub description: String,
    pub show_in_palette: bool,
    shortcut: Option<druid::HotKey>,
    exec: fn(&mut NPWindow, &mut EventCtx, &mut NPWindowState) -> bool,
}

impl UICommand {
    pub fn new(
        description: &str,
        show_in_palette: bool,
        shortcut: Option<druid::HotKey>,
        exec: fn(&mut NPWindow, &mut EventCtx, &mut NPWindowState) -> bool,
    ) -> Self {
        Self {
            description: description.to_owned(),
            show_in_palette,
            shortcut,
            exec,
        }
    }
    pub fn exec(&self, ctx: &mut EventCtx, editor_view: &mut NPWindow, editor: &mut NPWindowState) {
        (self.exec)(editor_view, ctx, editor);
    }
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
        window: &mut NPWindow,
        editor: &mut NPWindowState,
    ) {
        for c in &COMMANDSET.commands {
            if c.matches(event.borrow()) {
                (c.exec)(window, ctx, editor);
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

macro_rules! uicmd {
    ($commandset:ident = { $($command:ident = ($description:literal,$hotkey:literal, $v:expr, $b:expr));+ $(;)? } ) => {
        pub static $commandset: Lazy<UICommandSet> = Lazy::new(|| {
            let mut v = UICommandSet::new();
            $(v.commands.push(UICommand::new($description, $v,string_to_hotkey($hotkey), $b ));)+
            v
        });
    };
}

uicmd! {
    COMMANDSET = {
        PALCMD_CHANGE_LANGUAGE = ("Change language mode","CtrlShift-l", true,
        |_window, ctx, _data| {
            let languages: Vector<Item> = SYNTAXSET.syntaxes().iter().map(|l| Item::new(&l.name,&format!("File extensions : [{}]",l.file_extensions.join(", ")) )).collect();
            ctx.show_palette("Set Language mode to", languages, UICommandType::Editor(Rc::new(|_idx,name, _ctx, _editor_view, data: &mut EditStack| {
                data.file.syntax = SYNTAXSET.find_syntax_by_name(&name).unwrap();
            })));
            true
        });
        PALCMD_CHANGE_TYPE_TYPE = ("Change indentation","", true,
        |_window, ctx, _data| {
            let choice: Vector<Item> = ["Tabs","Spaces"].iter().map(|t| Item::new(t,&"")).collect();
            ctx.show_palette("Indent using", choice, UICommandType::Editor(Rc::new(|idx, _name, _ctx, _editor_view, data: &mut EditStack| {
                if idx == 0 {
                    data.file.indentation = crate::widgets::text_buffer::Indentation::Tab(4);
                } else {
                    data.file.indentation = crate::widgets::text_buffer::Indentation::Space(4);
                }
            })));
            true
        });
        PALCMD_OPEN = ("Open","Ctrl-o", true,
        |_window, ctx, data| {
            if data.editor.is_dirty() {
                let choice: Vector<Item> = ["Yes","No"].iter().map(|t| Item::new(t,&"")).collect();
                ctx.show_palette("Discard unsaved change?",choice,UICommandType::Editor(Rc::new(|idx, _name, ctx: &mut EventCtx, _editor_view, data| {
                    if idx == 0 {
                        let options = FileDialogOptions::new().show_hidden();
                        ctx.submit_command(Command::new(druid::commands::SHOW_OPEN_PANEL, options, Target::Auto));
                    }
                })));
            } else {
                let options = FileDialogOptions::new().show_hidden();
                ctx.submit_command(Command::new(druid::commands::SHOW_OPEN_PANEL, options, Target::Auto));
            }
            // && !MessageDialog::new()
            //     .set_level(rfd::MessageLevel::Warning)
            //     .set_title("Are you sure?")
            //     .set_description("Discard unsaved change?")
            //     .set_buttons(rfd::MessageButtons::YesNo)
            //     .show()
            // {
            //     return true;
            // }

            // let options = FileDialogOptions::new().show_hidden();
            // ctx.submit_command(Command::new(druid::commands::SHOW_OPEN_PANEL, options, Target::Auto));
            true
        });
        PALCMD_SAVE = ("Save","Ctrl-s",true,
        |_window, ctx, data| {
            if data.editor.filename.is_some() {
                ctx.submit_command(Command::new(druid::commands::SAVE_FILE, (), Target::Auto));
            } else {
                let options = FileDialogOptions::new().show_hidden();
                ctx.submit_command(Command::new(druid::commands::SHOW_SAVE_PANEL, options, Target::Auto))
            }
            return true;
        });
        PALCMD_SAVE_AS = ("Save As","CtrlShift-s",true,
        |_window, ctx, _data| {
            let options = FileDialogOptions::new().show_hidden();
            ctx.submit_command(Command::new(druid::commands::SHOW_SAVE_PANEL, options, Target::Auto));
            return true;
        });
    }
}

pub trait ShowPalette {
    fn show_palette(&mut self, title: &'static str, items: Vector<Item>, callback: UICommandType);
}

impl ShowPalette for EventCtx<'_, '_> {
    fn show_palette(&mut self, title: &'static str, items: Vector<Item>, callback: UICommandType) {
        self.submit_command(Command::new(
            SHOW_PALETTE_PANEL,
            (self.widget_id(), title, items, callback),
            Target::Auto,
        ));
    }
}
