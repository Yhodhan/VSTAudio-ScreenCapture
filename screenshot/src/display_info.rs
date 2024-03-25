// use std::ffi::OsString;
// use std::io::Error;
use std::mem;
// use std::os::windows::ffi::OsStringExt;
use std::ptr;

use winapi::shared::minwindef::{BOOL, LPARAM, TRUE};
use winapi::shared::windef::{HDC, HMONITOR, LPRECT};
use winapi::um::winuser::{EnumDisplayMonitors, GetMonitorInfoW, MONITORINFOEXW};

pub fn enumarate_monitors() -> Vec<MONITORINFOEXW> {
    let mut monitors = Vec::<MONITORINFOEXW>::new();
    let data = &mut monitors as *mut _;

    let result = unsafe {
        EnumDisplayMonitors(
            ptr::null_mut(),
            ptr::null(),
            Some(enum_monitor),
            data as LPARAM,
        )
    };

    if result != TRUE {
        panic!("Could not read screens");
    }
    monitors
}

unsafe extern "system" fn enum_monitor(monitor: HMONITOR, _: HDC, _: LPRECT, data: LPARAM) -> BOOL {
    let monitors: &mut Vec<MONITORINFOEXW> = mem::transmute(data);

    let mut monitor_info: MONITORINFOEXW = mem::zeroed();
    monitor_info.cbSize = mem::size_of::<MONITORINFOEXW>() as u32;

    let info_ptr = <*mut _>::cast(&mut monitor_info);

    let result = GetMonitorInfoW(monitor, info_ptr);
    if result == TRUE {
        monitors.push(monitor_info);
    }
    TRUE
}
