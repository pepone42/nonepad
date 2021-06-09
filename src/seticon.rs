use std::ptr;
use std::{thread, time};
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
    um::winuser::{FindWindowW, LoadIconW, SendMessageW, ICON_BIG, ICON_SMALL, MAKEINTRESOURCEW, WM_SETICON},
};

pub unsafe fn set_icon(id: u16) {
    extern "system" fn enum_windows_callback(hwnd: HWND, l_param: LPARAM) -> BOOL {
      let mut wnd_proc_id: DWORD = 0;
      unsafe {
        GetWindowThreadProcessId(hwnd, &mut wnd_proc_id as *mut DWORD);
        if GetCurrentProcessId() != wnd_proc_id {
          return TRUE;
        }
        set_window_icon(l_param as u16, hwnd);
      }
      return FALSE;
    }
    let mut hwnd: HWND = ptr::null_mut();
    EnumWindows(Some(enum_windows_callback), id as LPARAM);
    // return if hwnd.is_null() {
    //   None
    // } else {
    //   Some(hwnd)
    // }
  }

// winres set_icon or set_icon_with_id must be used in the build for this to work
fn set_window_icon(id: u16, hwnd: HWND) {
    // thread::spawn(move || {

            let hicon = unsafe { LoadIconW(GetModuleHandleW(0 as LPCWSTR), MAKEINTRESOURCEW(id)) };
            if hicon == 0 as HICON {
                eprintln!("No Icon #{} in resource", id);
            }

            unsafe { SendMessageW(hwnd, WM_SETICON, ICON_SMALL as WPARAM, hicon as LPARAM) };
            unsafe { SendMessageW(hwnd, WM_SETICON, ICON_BIG as WPARAM, hicon as LPARAM) };
    //});
}