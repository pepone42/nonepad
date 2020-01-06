/// Icons to display on message dialog.
#[derive(PartialEq, Eq, Debug)]
pub enum Icon{
    None,
    Info,
    Question,
    Warning,
    Error
}
/// Buttons on message dialog.
#[derive(PartialEq, Eq, Debug)]
pub enum Buttons{
    Ok,
    OkCancel,
    YesNo,
    AbortRetryIgnore
}
/// Triggered button on message dialog.
#[derive(PartialEq, Eq, Debug)]
pub enum Button{
    Ok,
    Cancel,
    Yes,
    No,
    Abort,
    Retry,
    Ignore
}

/// Shows modal message dialog with custom window caption and message text.
#[cfg(windows)]
pub fn messagebox(text: &str, caption: &str, icon: Icon, buttons: Buttons) -> Option<Button> {
    use winapi::um::winuser::*;
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;

    let text_wide: Vec<u16> = OsStr::new(text).encode_wide().chain(once(0)).collect();
    let caption_wide: Vec<u16> = OsStr::new(caption).encode_wide().chain(once(0)).collect();
    unsafe {
        match MessageBoxW(
            null_mut(), text_wide.as_ptr(), caption_wide.as_ptr(),
            match icon {
                Icon::None => 0,
                Icon::Info => MB_ICONINFORMATION,
                Icon::Question => MB_ICONQUESTION,
                Icon::Warning => MB_ICONWARNING,
                Icon::Error => MB_ICONERROR
            } + match buttons {
                Buttons::Ok => MB_OK,
                Buttons::OkCancel => MB_OKCANCEL,
                Buttons::YesNo => MB_YESNO,
                Buttons::AbortRetryIgnore => MB_ABORTRETRYIGNORE
            }
        ){
            IDOK => Some(Button::Ok),
            IDCANCEL => Some(Button::Cancel),
            IDYES => Some(Button::Yes),
            IDNO => Some(Button::No),
            IDABORT => Some(Button::Abort),
            IDRETRY => Some(Button::Retry),
            IDIGNORE => Some(Button::Ignore),
            _ => None
        }
    }
}
/// Shows modal message dialog with custom window caption and message text.
#[cfg(target_os = "linux")]
pub fn messagebox(text: &str, caption: &str, icon: Icon, buttons: Buttons) -> Option<Button> {
    use native::gtk::*;
    use std::ptr::null;
    use utils::string::str_to_cstr;
    let text_c = str_to_cstr(text);
    let caption_c = str_to_cstr(caption);
    unsafe{
        if !gtk_init_check(0, null()) {
            panic!("Couldn't initialize GTK!");
        }
        let dialog = gtk_message_dialog_new(
            null(), GTK_DIALOG_MODAL,
            match icon {
                Icon::None => GTK_MESSAGE_OTHER,
                Icon::Info => GTK_MESSAGE_INFO,
                Icon::Question => GTK_MESSAGE_QUESTION,
                Icon::Warning => GTK_MESSAGE_WARNING,
                Icon::Error => GTK_MESSAGE_ERROR
            },
            GTK_BUTTONS_NONE,
            text_c.as_ptr()
        );
        const RESPONSE_OK: i32 = -100;
        const RESPONSE_CANCEL: i32 = -101;
        const RESPONSE_YES: i32 = -102;
        const RESPONSE_NO: i32 = -103;
        const RESPONSE_ABORT: i32 = -104;
        const RESPONSE_RETRY: i32 = -105;
        const RESPONSE_IGNORE: i32 = -106;
        match buttons {
            Buttons::Ok => {
                let ok_button_label = str_to_cstr(&tl!("Ok"));
                gtk_dialog_add_button(dialog, ok_button_label.as_ptr(), RESPONSE_OK);
            },
            Buttons::OkCancel => {
                let ok_button_label = str_to_cstr(&tl!("Ok"));
                gtk_dialog_add_button(dialog, ok_button_label.as_ptr(), RESPONSE_OK);
                let cancel_button_label = str_to_cstr(&tl!("Cancel"));
                gtk_dialog_add_button(dialog, cancel_button_label.as_ptr(), RESPONSE_CANCEL);
            },
            Buttons::YesNo => {
                let yes_button_label = str_to_cstr(&tl!("Yes"));
                gtk_dialog_add_button(dialog, yes_button_label.as_ptr(), RESPONSE_YES);
                let no_button_label = str_to_cstr(&tl!("No"));
                gtk_dialog_add_button(dialog, no_button_label.as_ptr(), RESPONSE_NO);
            },
            Buttons::AbortRetryIgnore => {
                let abort_button_label = str_to_cstr(&tl!("Abort"));
                gtk_dialog_add_button(dialog, abort_button_label.as_ptr(), RESPONSE_ABORT);
                let retry_button_label = str_to_cstr(&tl!("Retry"));
                gtk_dialog_add_button(dialog, retry_button_label.as_ptr(), RESPONSE_RETRY);
                let ignore_button_label = str_to_cstr(&tl!("Ignore"));
                gtk_dialog_add_button(dialog, ignore_button_label.as_ptr(), RESPONSE_IGNORE);
            }
        }
        gtk_window_set_title(dialog, caption_c.as_ptr());
        gtk_window_set_keep_above(dialog, true);
        let response = gtk_dialog_run(dialog);
        gtk_widget_destroy(dialog);
        match response {
            RESPONSE_OK => Some(Button::Ok),
            RESPONSE_CANCEL => Some(Button::Cancel),
            RESPONSE_YES => Some(Button::Yes),
            RESPONSE_NO => Some(Button::No),
            RESPONSE_ABORT => Some(Button::Abort),
            RESPONSE_RETRY => Some(Button::Retry),
            RESPONSE_IGNORE => Some(Button::Ignore),
            _ => None
        }
    }
}
/// Shows modal message dialog with custom window caption and message text.
#[cfg(target_os = "macos")]
#[allow(unused)]
pub fn messagebox(text: &str, caption: &str, icon: Icon, buttons: Buttons) -> Option<Button> {

    // TODO
    unimplemented!();
    
}