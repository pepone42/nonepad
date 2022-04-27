use std::{borrow::Borrow, rc::Rc, sync::Arc};

use druid::{im::Vector, Command, EventCtx, FileDialogOptions, HotKey, KeyEvent, Selector, SysMods, Target, WidgetId};
use once_cell::sync::Lazy;

use crate::widgets::{
    editor_view::EditorView,
    text_buffer::{syntax::SYNTAXSET, EditStack},
    window::{NPWindow, NPWindowState},
    Item,
};

#[derive(Clone)]
pub enum UICommandType {
    Editor(Rc<dyn Fn(PaletteResult, &mut EventCtx, &mut EditorView, &mut EditStack)>),
    Window(Rc<dyn Fn(PaletteResult, &mut EventCtx, &mut NPWindow, &mut NPWindowState)>),
    DialogEditor(Rc<dyn Fn(DialogResult, &mut EventCtx, &mut EditorView, &mut EditStack)>),
    DialogWindow(Rc<dyn Fn(DialogResult, &mut EventCtx, &mut NPWindow, &mut NPWindowState)>),
}

pub const SHOW_SEARCH_PANEL: Selector<String> = Selector::new("nonepad.bottom_panel.show_search");
pub const SHOW_PALETTE_FOR_EDITOR: Selector<(WidgetId, String, Option<Vector<Item>>, Option<Rc<dyn Fn(PaletteResult, &mut EventCtx, &mut EditorView, &mut EditStack)>>)> = Selector::new("nonepad.palette.show_for_editor");
pub const SHOW_PALETTE_FOR_WINDOW: Selector<(WidgetId, String, Option<Vector<Item>>, Option<Rc<dyn Fn(PaletteResult, &mut EventCtx, &mut NPWindow, &mut NPWindowState)>>)> = Selector::new("nonepad.palette.show_for_window");
pub const SHOW_DIALOG_FOR_EDITOR: Selector<(WidgetId, String, Option<Vector<Item>>, Option<Rc<dyn Fn(DialogResult, &mut EventCtx, &mut EditorView, &mut EditStack)>>)> = Selector::new("nonepad.dialog.show_for_editor");
pub const SHOW_DIALOG_FOR_WINDOW: Selector<(WidgetId, String, Option<Vector<Item>>, Option<Rc<dyn Fn(DialogResult, &mut EventCtx, &mut NPWindow, &mut NPWindowState)>>)> = Selector::new("nonepad.dialog.show_for_window");


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
    #[cfg(target_os = "macos")]
    return Some(HotKey::new(mods, t[1]));
    #[cfg(not(target_os = "macos"))]
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
            Palette::new().items(languages)
                .title("Set Language mode to")
                .on_select(
                    |result: PaletteResult, _ctx, _editor_view, data: &mut EditStack| {
                        data.file.syntax = SYNTAXSET.find_syntax_by_name(&result.name).unwrap();
                    }
                ).show(ctx);
            true
        });
        PALCMD_CHANGE_TYPE_TYPE = ("Change indentation","", true,
        |_window, ctx, _data| {
            Palette::new().items(item!["Tabs","Spaces"])
                .title("Indent using")
                .on_select(
                    |result: PaletteResult, _ctx, _editor_view, data: &mut EditStack| {
                        if result.index == 0 {
                            data.file.indentation = crate::widgets::text_buffer::Indentation::Tab(4);
                        } else {
                            data.file.indentation = crate::widgets::text_buffer::Indentation::Space(4);
                        }
                    } 
                ).show(ctx);
            true
        });
        PALCMD_OPEN = ("Open","Ctrl-o", true,
        |_window, ctx, data| {
            if data.editor.is_dirty() {
                Palette::new().items(item!["Yes","No"]).title("Discard unsaved change?").on_select(
                    |result: DialogResult, ctx, _: &mut EditorView, _| {
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
        PALCMD_GOTO_LINE = ("Navigate to line","Ctrl-g", true,
        |_window, ctx, _data|{
            Palette::new().title("Navigate to line").on_select(|result: PaletteResult,ctx,ev: &mut EditorView,editor: &mut EditStack| {
                if let Ok(line) = result.name.parse::<usize>() {
                    ev.navigate_to_line(ctx,editor,line.into() );
                }
            }).show(ctx);
            return true;
        });
    }
}

trait ShowPalette<R, W, D> {
    fn show_palette(&mut self, title: String, items: Option<Vector<Item>>, callback: Option<Rc<dyn Fn(R, &mut EventCtx, &mut W, &mut D)>>);
}

impl<'a, 'b, 'c> ShowPalette<PaletteResult, EditorView, EditStack> for EventCtx<'b, 'c> {
    fn show_palette(&mut self, title: String, items: Option<Vector<Item>>, callback: Option<Rc<dyn Fn(PaletteResult, &mut EventCtx, &mut EditorView, &mut EditStack)>>) {
        self.submit_command(Command::new(
            SHOW_PALETTE_FOR_EDITOR,
            (self.widget_id(), title, items, callback),
            Target::Auto,
        ));
    }
}

impl<'a, 'b, 'c> ShowPalette<PaletteResult, NPWindow, NPWindowState> for EventCtx<'b, 'c> {
    fn show_palette(&mut self, title: String, items: Option<Vector<Item>>, callback: Option<Rc<dyn Fn(PaletteResult, &mut EventCtx, &mut NPWindow, &mut NPWindowState)>>) {
        self.submit_command(Command::new(
            SHOW_PALETTE_FOR_WINDOW,
            (self.widget_id(), title, items, callback),
            Target::Auto,
        ));
    }
}

impl<'a, 'b, 'c> ShowPalette<DialogResult, EditorView, EditStack> for EventCtx<'b, 'c> {
    fn show_palette(&mut self, title: String, items: Option<Vector<Item>>, callback: Option<Rc<dyn Fn(DialogResult, &mut EventCtx, &mut EditorView, &mut EditStack)>>) {
        self.submit_command(Command::new(
            SHOW_DIALOG_FOR_EDITOR,
            (self.widget_id(), title, items, callback),
            Target::Auto,
        ));
    }
}

impl<'a, 'b, 'c> ShowPalette<DialogResult, NPWindow, NPWindowState> for EventCtx<'b, 'c> {
    fn show_palette(&mut self, title: String, items: Option<Vector<Item>>, callback: Option<Rc<dyn Fn(DialogResult, &mut EventCtx, &mut NPWindow, &mut NPWindowState)>>) {
        self.submit_command(Command::new(
            SHOW_DIALOG_FOR_WINDOW,
            (self.widget_id(), title, items, callback),
            Target::Auto,
        ));
    }
}

macro_rules! item {
    ($($n : expr), + $(,) ?) => {{
        let mut v = Vector::new();
        $(v.push_back(Item::new($n,"") );)+
        v
    }};
}
pub(crate) use item;


#[derive(Debug,PartialEq,Eq,Clone, Copy)]
pub enum DialogResult {
    Ok,
    Cancel,
}
#[derive(Debug,Clone)]
pub struct PaletteResult {
    pub index: usize,
    pub name: Arc<String>,
}

pub struct Palette<R, W, D> {
    title: Option<String>,
    action: Option<Rc<dyn Fn(R, &mut EventCtx, &mut W, &mut D)>>,
    items: Option<Vector<Item>>,
}


impl<R, W, D> Default for Palette<R, W, D> {
    fn default() -> Self {
        Self { title: Default::default(), action: Default::default(), items: Default::default() }
    }
}

impl<R, W, D> Palette<R, W, D> {
    pub fn new() -> Self {
        Palette::default()
    }
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_owned());
        self
    }
    pub fn on_select(
        mut self,
        action: impl Fn(R, &mut EventCtx, &mut W, &mut D) + 'static,
    ) -> Self {
        self.action = Some(Rc::new(action));
        self
    }
    pub fn items(mut self, items: Vector<Item>) -> Self {
        self.items = Some(items);
        self
    }
    
}

impl Palette<PaletteResult, EditorView, EditStack> {
    pub fn show(self, ctx: &mut EventCtx) {
        ctx.show_palette(self.title.unwrap_or_default(), self.items, self.action);
    }
    pub fn alert(title: &str) -> Self {
        Palette::new().title(title).items(item!["Ok"])
    }
}

impl Palette<PaletteResult, NPWindow, NPWindowState> {
    pub fn show(self, ctx: &mut EventCtx) {
        ctx.show_palette(self.title.unwrap_or_default(), self.items, self.action);
    }
}

impl Palette<DialogResult, EditorView, EditStack> {
    pub fn show(self, ctx: &mut EventCtx) {
        ctx.show_palette(self.title.unwrap_or_default(), self.items, self.action);
    }
}

impl Palette<DialogResult, NPWindow, NPWindowState> {
    pub fn show(self, ctx: &mut EventCtx) {
        ctx.show_palette(self.title.unwrap_or_default(), self.items, self.action);
    }
}