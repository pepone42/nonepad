use std::{borrow::{Borrow, BorrowMut}, rc::Rc, sync::Arc};

use druid::{
    im::Vector, Command, Event, EventCtx, FileDialogOptions, HotKey, KeyEvent, Selector, SysMods, Target, WidgetId,
};
use once_cell::sync::Lazy;

use crate::widgets::{
    editor_view::EditorView,
    item,
    text_buffer::{syntax::SYNTAXSET, EditStack},
    window::{NPWindow, NPWindowState},
    DialogResult, Item, Palette, PaletteBuilder, PaletteResult,
};

#[derive(Clone)]
pub enum UICommandType {
    Editor(Rc<dyn Fn(PaletteResult, &mut EventCtx, &mut EditorView, &mut EditStack)>),
    Window(Rc<dyn Fn(PaletteResult, &mut EventCtx, &mut NPWindow, &mut NPWindowState)>),
    DialogEditor(Rc<dyn Fn(DialogResult, &mut EventCtx, &mut EditorView, &mut EditStack)>),
    DialogWindow(Rc<dyn Fn(DialogResult, &mut EventCtx, &mut NPWindow, &mut NPWindowState)>),
}

pub const SHOW_SEARCH_PANEL: Selector<String> = Selector::new("nonepad.bottom_panel.show_search");

pub const SEND_STRING_DATA: Selector<String> = Selector::new("nonepad.all.send_data");
pub const CLOSE_BOTTOM_PANEL: Selector<()> = Selector::new("nonepad.bottom_panel.close");
pub const RESET_HELD_STATE: Selector<()> = Selector::new("nonepad.all.reste_held_state");
pub const REQUEST_NEXT_SEARCH: Selector<String> = Selector::new("nonepad.editor.request_next_search");
pub const GIVE_FOCUS: Selector<()> = Selector::new("nonepad.all.give_focus");
pub const SELECT_LINE: Selector<(usize, bool)> = Selector::new("nonepad.editor.select_line");
pub const SCROLL_TO: Selector<(Option<f64>, Option<f64>)> = Selector::new("nonepad.editor.scroll_to_rect");
pub const HIGHLIGHT: Selector<(usize, usize)> = Selector::new("nonepad.editor.highlight");
pub const PALETTE_CALLBACK: Selector<(PaletteResult, UICommandType)> = Selector::new("nonepad.editor.execute_command");
pub const CLOSE_PALETTE: Selector<()> = Selector::new("nonepad.palette.close");
pub const RELOAD_FROM_DISK: Selector<()> = Selector::new("nonepad.editor.reload_from_disk");
pub const FILE_REMOVED: Selector<()> = Selector::new("nonepad.editor.file_removed");
const UICOMMAND_CALLBACK: Selector<UICommandCallback> = Selector::new("nonepad.all.uicommand_callback");


#[derive(Clone)]
enum UICommandCallback {
    Window(fn(&mut NPWindow, &mut EventCtx, &mut NPWindowState) -> bool),
    EditView(fn(&mut EditorView, &mut EventCtx, &mut EditStack) -> bool),
}

struct UICommand {
    pub description: String,
    pub show_in_palette: bool,
    shortcut: Option<druid::HotKey>,
    //exec: fn(&mut NPWindow, &mut EventCtx, &mut NPWindowState) -> bool,
    exec: UICommandCallback,
}

impl UICommand {
    fn new(description: &str, show_in_palette: bool, shortcut: Option<druid::HotKey>, exec: UICommandCallback) -> Self {
        Self {
            description: description.to_owned(),
            show_in_palette,
            shortcut,
            exec,
        }
    }

    fn matches(&self, event: &KeyEvent) -> bool {
        self.shortcut.clone().map(|s| s.matches(event)).unwrap_or(false)
    }
}

// pub trait UICommandExecutor<W, D> {
//     fn exec(&self, ctx: &mut EventCtx, widget: &mut W, state: &mut D);
// }

// impl UICommandExecutor<NPWindow, NPWindowState> for UICommand {
//     fn exec(&self, ctx: &mut EventCtx, win: &mut NPWindow, win_state: &mut NPWindowState) {
//         if let UICommandCallback::Window(c) = self.exec {
//             c(win, ctx, win_state);
//         }
//     }
// }

// impl UICommandExecutor<EditorView, EditStack> for UICommand {
//     fn exec(&self, ctx: &mut EventCtx, editor_view: &mut EditorView, editor: &mut EditStack) {
//         if let UICommandCallback::EditView(c) = self.exec {
//             c(editor_view, ctx, editor);
//         }
//     }
// }

struct UICommandSet {
    commands: Vec<UICommand>,
}

impl UICommandSet {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }
}

pub struct CommandSet;

pub trait UICommandEventHandler<W, D> {
    fn event(&self, ctx: &mut EventCtx, event: &Event, window: &mut W, editor: &mut D);
}

impl UICommandEventHandler<NPWindow, NPWindowState> for CommandSet {
    fn event(&self, ctx: &mut EventCtx, event: &Event, window: &mut NPWindow, editor: &mut NPWindowState) {
        match event {
            Event::KeyDown(event) => {
                for c in &WINCOMMANDSET.commands {
                    if c.matches(event.borrow()) {
                        if let UICommandCallback::Window(c) = c.exec {
                            c(window, ctx, editor);
                            ctx.set_handled();
                        }
                    }
                }
            }
            Event::Command(cmd) if cmd.is(UICOMMAND_CALLBACK) => {
                if let UICommandCallback::Window(f) = cmd.get_unchecked(UICOMMAND_CALLBACK) {
                    f(window,ctx,editor);
                    ctx.set_handled();
                }
                
            }
            _ => (),
        }
    }
}

impl UICommandEventHandler<EditorView, EditStack> for CommandSet {
    fn event(&self, ctx: &mut EventCtx, event: &Event, window: &mut EditorView, editor: &mut EditStack) {
        match event {
            Event::KeyDown(event) => {
                for c in &VIEWCOMMANDSET.commands {
                    if c.matches(event.borrow()) {
                        if let UICommandCallback::EditView(c) = c.exec {
                            c(window, ctx, editor);
                        }
                    }
                }
            }
            Event::Command(cmd) if cmd.is(UICOMMAND_CALLBACK) => {
                if let UICommandCallback::EditView(f) = cmd.get_unchecked(UICOMMAND_CALLBACK) {
                    f(window,ctx,editor);
                    ctx.set_handled();
                }
                
            }
            _ => (),
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
    #[cfg(target_os = "macos")]
    return Some(HotKey::new(mods, t[1]));
    #[cfg(not(target_os = "macos"))]
    if t[0].contains("Shift") {
        Some(HotKey::new(mods, t[1].to_uppercase().as_str()))
    } else {
        Some(HotKey::new(mods, t[1]))
    }
}

macro_rules! wincmd {
    ($commandset:ident = { $($command:ident = ($description:literal,$hotkey:literal, $v:expr, $b:expr));+ $(;)? } ) => {
        static $commandset: Lazy<UICommandSet> = Lazy::new(|| {
            let mut v = UICommandSet::new();
            $(v.commands.push(UICommand::new($description, $v,string_to_hotkey($hotkey), UICommandCallback::Window($b) ));)+
            v
        });
    };

}

macro_rules! viewcmd {
    ($commandset:ident = { $($command:ident = ($description:literal,$hotkey:literal, $v:expr, $b:expr));+ $(;)? } ) => {
        static $commandset: Lazy<UICommandSet> = Lazy::new(|| {
            let mut v = UICommandSet::new();
            $(v.commands.push(UICommand::new($description, $v,string_to_hotkey($hotkey), UICommandCallback::EditView($b) ));)+
            v
        });
    };

}

wincmd! {
    WINCOMMANDSET = {
        PLACMD_SHOW_PAL = ("Show command palette","CtrlShift-p", false,
        |win, ctx, _data| {
            let mut items = Vector::new();
            for c in WINCOMMANDSET.commands.iter().filter(|c| c.show_in_palette) {
                items.push_back(Item::new(&c.description, &""));
            }
            let viewcmd_start_index = items.len();
            for c in VIEWCOMMANDSET.commands.iter().filter(|c| c.show_in_palette) {
                items.push_back(Item::new(&c.description, &""));
            }
            win.palette()
                .items(items)
                .on_select(move |result, ctx, win, data| {
                    if result.index>=viewcmd_start_index {
                        if let Some(ui_cmd) = &VIEWCOMMANDSET.commands.iter().filter(|c| c.show_in_palette).nth(result.index - viewcmd_start_index) {
                            // TODO: Find a more elegent way
                            // TODO: Send command to current editor target, not global
                            ctx.submit_command(Command::new(UICOMMAND_CALLBACK, ui_cmd.exec.clone(), Target::Global));
                        }
                    } else {
                        if let Some(ui_cmd) = &WINCOMMANDSET.commands.iter().filter(|c| c.show_in_palette).nth(result.index) {
                            //ui_cmd.exec(ctx, win, data);
                            ctx.submit_command(Command::new(UICOMMAND_CALLBACK, ui_cmd.exec.clone(), Target::Global));
                        }
                    }
                })
                .show(ctx);
            true
        });
        PALCMD_CHANGE_LANGUAGE = ("Change language mode","CtrlShift-l", true,
        |window, ctx, _data| {
            let languages: Vector<Item> = SYNTAXSET.syntaxes().iter().map(|l| Item::new(&l.name,&format!("File extensions : [{}]",l.file_extensions.join(", ")) )).collect();
            window.palette().items(languages)
                .title("Set Language mode to")
                .on_select(
                    |result: PaletteResult, _ctx, _win, data| {
                        data.editor.file.syntax = SYNTAXSET.find_syntax_by_name(&result.name).unwrap();
                    }
                ).show(ctx);
            true
        });
        PALCMD_CHANGE_TYPE_TYPE = ("Change indentation","", true,
        |window, ctx, _data| {
            window.palette().items(item!["Tabs","Spaces"])
                .title("Indent using")
                .on_select(
                    |result: PaletteResult, _ctx, _win, data| {
                        if result.index == 0 {
                            data.editor.file.indentation = crate::widgets::text_buffer::Indentation::Tab(4);
                        } else {
                            data.editor.file.indentation = crate::widgets::text_buffer::Indentation::Space(4);
                        }
                    }
                ).show(ctx);
            true
        });
        PALCMD_OPEN  = ("Open","Ctrl-o", true,
        |window, ctx, data| {
            if data.editor.is_dirty() {
                window.dialog().title("Discard unsaved change?").on_select(
                    |result, ctx, _, _| {
                        if result == DialogResult::Ok {
                            let options = FileDialogOptions::new().show_hidden();
                            ctx.submit_command(Command::new(druid::commands::SHOW_OPEN_PANEL, options, Target::Auto));
                        }
                    }
                ).show(ctx);
            } else {
                let options = FileDialogOptions::new().show_hidden();
                ctx.submit_command(Command::new(druid::commands::SHOW_OPEN_PANEL, options, Target::Auto));
            }
            true
        });
        PALCMD_SAVE  = ("Save","Ctrl-s",true,
        |_window, ctx, data| {
            if data.editor.filename.is_some() {
                ctx.submit_command(Command::new(druid::commands::SAVE_FILE, (), Target::Auto));
            } else {
                let options = FileDialogOptions::new().show_hidden();
                ctx.submit_command(Command::new(druid::commands::SHOW_SAVE_PANEL, options, Target::Auto))
            }
            return true;
        });
        PALCMD_SAVE_AS  = ("Save As","CtrlShift-s",true,
        |_window, ctx, _data| {
            let options = FileDialogOptions::new().show_hidden();
            ctx.submit_command(Command::new(druid::commands::SHOW_SAVE_PANEL, options, Target::Auto));
            return true;
        });
    }
}

viewcmd! {
    VIEWCOMMANDSET = {
        PALCMD_GOTO_LINE  = ("Navigate to line","Ctrl-g", true,
        |window, ctx, _data|{
            window.palette().title("Navigate to line").on_select(|result,ctx,ev,editor| {
                if let Ok(line) = result.name.parse::<usize>() {
                    ev.navigate_to_line(ctx,editor,line.into() );
                }
            }).show(ctx);
            return true;
        });
    }
}
