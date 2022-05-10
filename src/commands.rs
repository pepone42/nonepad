use std::{
    any::Any,
    borrow::{Borrow, BorrowMut},
    rc::Rc,
    sync::{Arc, Mutex},
};

use druid::{
    im::Vector, Data, Event, EventCtx, FileDialogOptions, HotKey, KeyEvent, RawMods, Selector, SysMods, Widget,
};
use once_cell::sync::Lazy;

use crate::widgets::{
    editor_view::EditorView,
    item,
    text_buffer::{syntax::SYNTAXSET, EditStack},
    window::{NPWindow, NPWindowState},
    DialogResult, Item, PaletteBuilder, PaletteResult,
};

#[derive(Debug, Clone)]
pub struct NPHotKey {
    key1: HotKey,
    key2: Option<HotKey>,
}

pub trait Command<W: Widget<D>, D: Data>: Sync + Send {
    fn new() -> Self
    where
        Self: Sized;
    fn run(&mut self, ctx: &mut EventCtx, widget: &mut W, data: &mut D);
    fn description(&self) -> &str;
    fn shortcut(&self) -> Option<NPHotKey> {
        return None;
    }
    fn before_edit(&mut self, ctx: &mut EventCtx, widget: &mut W, data: &mut D, input: &str) {}
    fn after_edit(&mut self, ctx: &mut EventCtx, widget: &mut W, data: &mut D, input: &str) {}
    fn matches(&self, event: &KeyEvent) -> bool {
        self.shortcut().clone().map(|s| s.key1.matches(event)).unwrap_or(false)
    }
}

macro_rules! register_cmd {
    ($t:ty) => {
        paste::item! {
            #[small_ctor::ctor]
            #[allow(non_snake_case)]
            unsafe fn [< register_command_ $t >] () {
                WC.push(Box::new($t::new()));
            }
        }
    };
}


fn register_wincommands() -> Vec<Box<dyn Command<NPWindow, NPWindowState>>> {
    let mut v: Vec<Box<dyn Command<NPWindow, NPWindowState>>> = Vec::new();
    v.push(Box::new(TestCommand::new()));
    v
}
struct TestCommand;
register_cmd!(TestCommand);
impl Command<NPWindow, NPWindowState> for TestCommand {
    fn new() -> Self
    where
        Self: Sized,
    {
        TestCommand
    }
    fn shortcut(&self) -> Option<NPHotKey> {
        Some(NPHotKey {
            key1: HotKey::new(RawMods::Ctrl, "i"),
            key2: None,
        })
    }

    fn run(&mut self, ctx: &mut EventCtx, widget: &mut NPWindow, data: &mut NPWindowState) {
        widget.alert("hey!").show(ctx);
    }

    fn description(&self) -> &str {
        "une commande de test"
    }
}



static mut WC: Vec<Box<dyn Command<NPWindow, NPWindowState>>> = vec![];



struct WindowCommandsSet {
    commands: Vec<Box<dyn Command<NPWindow, NPWindowState>>>,
}



static WINDOWCOMMANDSSET: Lazy<Mutex<WindowCommandsSet>> = Lazy::new(|| {
    Mutex::new(WindowCommandsSet {
        commands: register_wincommands(),
    })
});

pub struct WindowCommands;

impl UICommandEventHandler<NPWindow, NPWindowState> for WindowCommands {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, window: &mut NPWindow, editor: &mut NPWindowState) {
        match event {
            Event::KeyDown(event) => {
                let cmdset = unsafe{&mut WC};
                for c in cmdset {
                    if c.matches(event.borrow()) {
                        c.run(ctx, window, editor);
                        ctx.set_handled();
                    }
                }
            }
            Event::Command(cmd) if cmd.is(UICOMMAND_CALLBACK) => {
                if let UICommandCallback::Window(f) = cmd.get_unchecked(UICOMMAND_CALLBACK) {
                    f(window, ctx, editor);
                    ctx.set_handled();
                }
            }
            _ => (),
        }
    }
}

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
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, window: &mut W, editor: &mut D);
}

impl UICommandEventHandler<NPWindow, NPWindowState> for CommandSet {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, window: &mut NPWindow, editor: &mut NPWindowState) {
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
                    f(window, ctx, editor);
                    ctx.set_handled();
                }
            }
            _ => (),
        }
    }
}

impl UICommandEventHandler<EditorView, EditStack> for CommandSet {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, window: &mut EditorView, editor: &mut EditStack) {
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
                    f(window, ctx, editor);
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
                .on_select(move |result, ctx, _, _| {
                    if result.index>=viewcmd_start_index {
                        if let Some(ui_cmd) = &VIEWCOMMANDSET.commands.iter().filter(|c| c.show_in_palette).nth(result.index - viewcmd_start_index) {
                            // TODO: Send command to current editor target, not global
                            ctx.submit_command(UICOMMAND_CALLBACK.with(ui_cmd.exec.clone()));
                        }
                    } else {
                        if let Some(ui_cmd) = &WINCOMMANDSET.commands.iter().filter(|c| c.show_in_palette).nth(result.index) {
                            ctx.submit_command(UICOMMAND_CALLBACK.with(ui_cmd.exec.clone()));
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
                            ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(options));
                        }
                    }
                ).show(ctx);
            } else {
                let options = FileDialogOptions::new().show_hidden();
                ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(options));
            }
            true
        });
        PALCMD_SAVE  = ("Save","Ctrl-s",true,
        |_window, ctx, data| {
            if data.editor.filename.is_some() {
                ctx.submit_command(druid::commands::SAVE_FILE);
            } else {
                let options = FileDialogOptions::new().show_hidden();
                ctx.submit_command(druid::commands::SHOW_SAVE_PANEL.with(options))
            }
            return true;
        });
        PALCMD_SAVE_AS  = ("Save As","CtrlShift-s",true,
        |_window, ctx, _data| {
            let options = FileDialogOptions::new().show_hidden();
            ctx.submit_command(druid::commands::SHOW_SAVE_PANEL.with(options));
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
