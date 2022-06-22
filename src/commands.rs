use std::{borrow::Borrow, ffi::OsStr, fmt::Display, rc::Rc};

use druid::{
    im::Vector, Application, ClipboardFormat, Event, EventCtx, FileDialogOptions, HotKey, KeyEvent, Selector, SysMods,
};
use once_cell::sync::Lazy;

use crate::widgets::{
    editor_view::EditorView,
    item,
    text_buffer::{syntax::SYNTAXSET, EditStack},
    view_switcher::ViewId,
    window::{NPWindow, NPWindowState},
    DialogResult, Item, PaletteBuilder, PaletteItemResult, 
};

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
                    f(window, ctx, editor);
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
                    if result.index >= viewcmd_start_index {
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
        PALCMD_OPEN  = ("Open","Ctrl-o", true,
        |window, ctx, data| {
            let options = FileDialogOptions::new().show_hidden();
            ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(options));
            true
        });
        PALCMD_NEW  = ("New","Ctrl-n", true,
        |window, ctx, data| {
            ctx.submit_command(crate::widgets::view_switcher::NEW_EDITVIEW);
            true
        });
        PALCMD_LIST_VIEW = ("Opened files","Ctrl-p", true,
        |window, ctx, data| {
            let items = data.views.editors.iter().map(|e| e.1);
            let indexes : Vec<_> = data.views.editors.iter().map(|e| *e.0).collect();
            //items.sort_by(|l,r| l.cmp(r));
            window.palette().items(items).on_select(move |result, ctx, window, data| {
                dbg!(&result);
                dbg!(&indexes);
                data.views.select_view(indexes[result.index]);

                } ).show(ctx);
            true
        })

    }
}

viewcmd! {
    VIEWCOMMANDSET = {
        PALCMD_CHANGE_LANGUAGE = ("Change language mode","CtrlShift-l", true,
        |view, ctx, _data| {
            let languages = SYNTAXSET.syntaxes().iter().map(|l| l.name.clone() );//.enumerate();//.collect();
            view.palette().items(languages)
                .title("Set Language mode to")
                .on_select(
                    |result: PaletteItemResult, _ctx, _win, data| {
                        data.file.syntax = SYNTAXSET.find_syntax_by_name(&result.name).unwrap();
                    }
                ).show(ctx);
            true
        });
        PALCMD_CHANGE_TYPE_TYPE = ("Change indentation","", true,
        |view, ctx, _data| {
            view.palette().items(["Tabs","Spaces"])
                .title("Indent using")
                .on_select(
                    |result: PaletteItemResult, _ctx, _win, data| {
                        if result.index == 0 {
                            data.file.indentation = crate::widgets::text_buffer::Indentation::Tab(4);
                        } else {
                            data.file.indentation = crate::widgets::text_buffer::Indentation::Space(4);
                        }
                    }
                ).show(ctx);
            true
        });
        // PALCMD_OPEN  = ("Open","Ctrl-o", true,
        // |view, ctx, data| {
        //     if data.is_dirty() {
        //         view.dialog().title("Discard unsaved change?").on_select(
        //             |result, ctx, _, _| {
        //                 if result == DialogResult::Ok {
        //                     let options = FileDialogOptions::new().show_hidden();
        //                     ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(options));
        //                 }
        //             }
        //         ).show(ctx);
        //     } else {
        //         let options = FileDialogOptions::new().show_hidden();
        //         ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(options));
        //     }
        //     true
        // });
        PALCMD_SAVE  = ("Save","Ctrl-s",true,
        |_view, ctx, data| {
            if data.filename.is_some() {
                ctx.submit_command(druid::commands::SAVE_FILE);
            } else {
                let options = FileDialogOptions::new().show_hidden();
                ctx.submit_command(druid::commands::SHOW_SAVE_PANEL.with(options))
            }
            return true;
        });
        PALCMD_SAVE_AS  = ("Save As","CtrlShift-s",true,
        |_view, ctx, _data| {
            let options = FileDialogOptions::new().show_hidden();
            ctx.submit_command(druid::commands::SHOW_SAVE_PANEL.with(options));
            return true;
        });
        PALCMD_GOTO_LINE  = ("Navigate to line","Ctrl-g", true,
        |view, ctx, _|{
            view.palette().title("Navigate to line").on_select(|result,ctx,ev,editor| {
                if let Ok(line) = result.name.parse::<usize>() {
                    ev.navigate_to_line(ctx,editor,line.into() );
                }
            }).show(ctx);
            return true;
        });
        SEARCH = ("Search","Ctrl-f", true,
        |_, ctx, editor| {
            ctx.submit_command(crate::widgets::bottom_panel::SHOW_SEARCH_PANEL.with(editor.main_cursor_selected_text()));
            return true;
        });
        DUPLICATE_CURSOR_SELECTION = ("Duplicate cursor","Ctrl-d", false,
        |_, _, editor| {
            editor
                .buffer
                .duplicate_cursor_from_str(&editor.main_cursor_selected_text());
                return true;
        });
        COPY = ("Copy selections to clipboard","Ctrl-c", false,
        |_,_,editor| {
            Application::global().clipboard().put_string(editor.selected_text());
            return true;
        });
        CUT = ("Cut selections to clipboard","Ctrl-x", false,
        |_,_,editor| {
            Application::global().clipboard().put_string(editor.selected_text());
            editor.delete();
            return true;
        });
        PASTE = ("Paste from clipboard","Ctrl-v", false,
        |_,_,editor| {
            let clipboard = Application::global().clipboard();
            let supported_types = &[ClipboardFormat::TEXT];
            let best_available_type = clipboard.preferred_format(supported_types);
            if let Some(format) = best_available_type {
                let data = clipboard
                    .get_format(format)
                    .expect("I promise not to unwrap in production");
                editor.insert(String::from_utf8_lossy(&data).as_ref());
            }
            return true;
        });
        UNDO = ("Undo","Ctrl-z", false,
        |_,_,editor| {
            editor.undo();
            return true;
        });
        REDO = ("redo","Ctrl-y", false,
        |_,_,editor| {
            editor.redo();
            return true;
        });
        SELECT_ALL = ("Select all text","Ctrl-a", true,
        |_,_,editor| {
            editor.select_all();
            return true;
        });
    }
}
