use winapi::{
    shared::{minwindef::DWORD, ntdef::LONG},
    um::{
        wingdi::{
            BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDIBits,
            SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, RGBQUAD,
        },
        winuser::{GetDC, GetDesktopWindow, GetSystemMetrics, ReleaseDC},
    },
};

use std::{
    mem,
    os::raw::c_int,
};

pub struct Screenshot {
    data: Vec<u8>,
    height: usize,
    width: usize,
    row_len: usize,
    pixel_width: usize,
}

pub unsafe fn screenshot() -> Result<Screenshot, String> {
    let wnd_screen = GetDesktopWindow();
    let dc_screen = GetDC(wnd_screen);
    let width = GetSystemMetrics(0);
    let height = GetSystemMetrics(1);

    // create windows bitmap
    let h_dc = CreateCompatibleDC(dc_screen);
    // as in C we have to check that we did not receive a null pointer

    let bit_map = CreateCompatibleBitmap(h_dc, width, height);

    let old_obj = SelectObject(h_dc, mem::transmute(bit_map));

    let res = BitBlt(
        h_dc,
        0,
        0,
        width,
        height,
        dc_screen,
        0,
        0,
        0x00CC0020 | 0x40000000,
    );

    // check error
    if res == 0 {
        return Err("Could not copy screen to buffer".to_string());
    }

    let pixel_width: usize = 4;

    // This struct is actually deprecated
    let mut bit_map_info = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: mem::size_of::<BITMAPINFOHEADER>() as DWORD,
            biWidth: width as LONG,
            biHeight: height as LONG,
            biPlanes: 1,
            biBitCount: 8 * pixel_width as u16,
            biCompression: BI_RGB,
            biSizeImage: (width * height * pixel_width as c_int) as DWORD,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        },
        bmiColors: [RGBQUAD {
            rgbBlue: 0,
            rgbGreen: 0,
            rgbRed: 0,
            rgbReserved: 0,
        }],
    };

    // Create a Vec for image
    let size: usize = (width * height) as usize * pixel_width;
    let mut data: Vec<u8> = Vec::with_capacity(size);
    data.set_len(size);

    // copy bits into Vec
    GetDIBits(
        h_dc,
        bit_map,
        0,
        height as DWORD,
        &mut data[0] as *mut u8 as *mut winapi::ctypes::c_void,
        &mut bit_map_info as *mut BITMAPINFO,
        DIB_RGB_COLORS,
    );

    // Release native image buffers
    ReleaseDC(wnd_screen, dc_screen); // don't need screen anymore
    DeleteDC(h_dc);
    DeleteObject(mem::transmute(bit_map));

    Ok(Screenshot {
        data: data.reverse(),
        height: height as usize,
        width: width as usize,
        row_len: (width * pixel_width as i32) as usize,
        pixel_width,
    })
}

pub fn save_image(path: Stringt)
