use druid::WindowId;

#[cfg(windows)]
mod windows {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::prelude::OsStrExt;

    use winapi::shared::minwindef::BOOL;
    use winapi::shared::minwindef::DWORD;
    use winapi::shared::minwindef::FALSE;
    use winapi::shared::minwindef::TRUE;
    use winapi::um::libloaderapi::GetModuleHandleW;
    use winapi::um::processthreadsapi::GetCurrentProcessId;
    use winapi::um::winnt::LPCWSTR;
    use winapi::um::winuser::EnumWindows;
    use winapi::um::winuser::GetWindowThreadProcessId;
    use winapi::{
        shared::minwindef::{LPARAM, WPARAM},
        shared::windef::{HICON, HWND},
        um::winuser::{LoadIconW, SendMessageW, ICON_BIG, ICON_SMALL, WM_SETICON},
    };

    // winres set_icon or set_icon_with_id must be used in the build for this to work
    pub unsafe fn set_windows_icon() {
        extern "system" fn enum_windows_callback(hwnd: HWND, _l_param: LPARAM) -> BOOL {
            let mut wnd_proc_id: DWORD = 0;
            unsafe {
                GetWindowThreadProcessId(hwnd, &mut wnd_proc_id as *mut DWORD);
                if GetCurrentProcessId() != wnd_proc_id {
                    return TRUE;
                }
                let icon_name: Vec<u16> = OsStr::new( "main_icon" ).encode_wide().chain( once( 0 ) ).collect();
                let hicon = LoadIconW(GetModuleHandleW(0 as LPCWSTR), icon_name.as_ptr());// MAKEINTRESOURCEW(id));
                if hicon == 0 as HICON {
                    eprintln!("No Icon main_icon in resource");
                }

                SendMessageW(hwnd, WM_SETICON, ICON_SMALL as WPARAM, hicon as LPARAM);
                SendMessageW(hwnd, WM_SETICON, ICON_BIG as WPARAM, hicon as LPARAM);
            }
            FALSE
        }

        EnumWindows(Some(enum_windows_callback), 0);

    }
}
#[cfg(windows)]
pub fn set_icon(_win_id: WindowId) {
    unsafe {windows::set_windows_icon()}
}
#[cfg(not(windows))]
pub fn set_icon(win_id: WindowId) {
    // todo
}