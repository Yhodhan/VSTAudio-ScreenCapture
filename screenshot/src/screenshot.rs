use image::RgbaImage;
use std::mem::size_of;
use windows::Win32::Foundation::{ERROR_INVALID_PARAMETER, HWND};
use windows::Win32::Graphics::Gdi::{
    CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC, GetDIBits,
    ReleaseDC, SelectObject, StretchBlt, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS,
    SRCCOPY,
};

use windows::Win32::UI::WindowsAndMessaging::{
    GetSystemMetrics, SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN, SM_XVIRTUALSCREEN, SM_YVIRTUALSCREEN,
};

pub struct ScreenData {
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>,
}

fn capture_screen() -> Result<ScreenData, String> {
    unsafe {
        let hdc_screen = GetDC(HWND::default());
        if hdc_screen.is_invalid() {
            return Err("Could not fetch Devide context".to_string());
        }

        let hdc = CreateCompatibleDC(hdc_screen);
        if hdc.is_invalid() {
            ReleaseDC(HWND::default(), hdc_screen);
            return Err("Could not create Compatible Device Context".to_string());
        }

        let x = GetSystemMetrics(SM_XVIRTUALSCREEN);
        let y = GetSystemMetrics(SM_YVIRTUALSCREEN);
        let width = GetSystemMetrics(SM_CXVIRTUALSCREEN);
        let height = GetSystemMetrics(SM_CYVIRTUALSCREEN);

        let hbmp = CreateCompatibleBitmap(hdc_screen, width, height);
        if hbmp.is_invalid() {
            DeleteDC(hdc);
            ReleaseDC(HWND::default(), hdc_screen);
            return Err("Could not create Bit Map".to_string());
        }

        let old_object = SelectObject(hdc, hbmp);
        if old_object.is_invalid() {
            DeleteDC(hdc);
            DeleteObject(hbmp);
            ReleaseDC(HWND::default(), hdc_screen);
            return Err("Could not select object from Device Context".to_string());
        }

        let sb = StretchBlt(
            hdc, 0, 0, width, height, hdc_screen, x, y, width, height, SRCCOPY,
        );
        if sb == false {
            DeleteDC(hdc);
            DeleteObject(hbmp);
            ReleaseDC(HWND::default(), hdc_screen);
            return Err("Could not create Stretch Blt".to_string());
        }

        let bmih = BITMAPINFOHEADER {
            biSize: size_of::<BITMAPINFOHEADER>() as u32,
            biPlanes: 1,
            biBitCount: 32,
            biWidth: width,
            biHeight: -height,
            biCompression: BI_RGB.0 as u32,
            ..Default::default()
        };

        let mut bit_map_info = BITMAPINFO {
            bmiHeader: bmih,
            ..Default::default()
        };

        let mut data: Vec<u8> = vec![0; (4 * width * height) as usize];

        let gdb = GetDIBits(
            hdc,
            hbmp,
            0,
            height as u32,
            Some(data.as_mut_ptr() as *mut core::ffi::c_void),
            &mut bit_map_info,
            DIB_RGB_COLORS,
        );

        if gdb == 0 || gdb == ERROR_INVALID_PARAMETER.0 as i32 {
            DeleteDC(hdc);
            DeleteObject(hbmp);
            ReleaseDC(HWND::default(), hdc_screen);
            return Err("Could not fetch DI Bits".to_string());
        }

        data.chunks_exact_mut(4).for_each(|c| c.swap(0, 2));

        DeleteDC(hdc);
        DeleteObject(hbmp);
        ReleaseDC(HWND::default(), hdc_screen);

        Ok(ScreenData {
            data,
            height,
            width,
        })
    }
}

pub fn screenshot() -> Result<(), String> {
    let s = capture_screen()?;
    let img = RgbaImage::from_raw(s.width as u32, s.height as u32, s.data)
        .ok_or_else(|| "Could not obtain image from raw data".to_string())?;

    img.save("screenshot.bmp").map_err(|err| err.to_string())?;
    Ok(())
}
