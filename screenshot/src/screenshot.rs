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

// Represent the necessary data for image crate to produce a new picture
pub struct ScreenData {
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>,
}

// This is the main function that deals with the screen capture
// It uses the windows api to access to the screen representation
fn capture_screen() -> Result<ScreenData, String> {
    unsafe {
        // The function creates a Device context that is a data structured that contains information
        // about the drawing attributes of a device such as display which will be used to the pixels data.
        let hdc_screen = GetDC(HWND::default());

        // This checks (and similar ones) if it is a valid references, as this functions are actually wrappers to C/C++ API functions
        // that return pointer to the data structures.
        // if objects were created during the process and one of them fails, the others must be deleted otherwise it can cause memory leaks.
        if hdc_screen.is_invalid() {
            return Err("Could not create Devide context".to_string());
        }

        let hdc = CreateCompatibleDC(hdc_screen);
        if hdc.is_invalid() {
            ReleaseDC(HWND::default(), hdc_screen);
            return Err("Could not create Compatible Device Context".to_string());
        }

        // Get the screen measures that will be used to draw into the devide context.
        let x = GetSystemMetrics(SM_XVIRTUALSCREEN);
        let y = GetSystemMetrics(SM_YVIRTUALSCREEN);
        let width = GetSystemMetrics(SM_CXVIRTUALSCREEN);
        let height = GetSystemMetrics(SM_CYVIRTUALSCREEN);

        // Creates the bitmap that will be use to draw the screenshot.
        let bit_map = CreateCompatibleBitmap(hdc_screen, width, height);
        if bit_map.is_invalid() {
            DeleteDC(hdc);
            ReleaseDC(HWND::default(), hdc_screen);
            return Err("Could not create Bit Map".to_string());
        }

        // The function replaces the old bitmap with the new one just created, then it returns the old map
        // this is required because the created device context contains a default bit map that does not contain the pixels data.
        let old_object = SelectObject(hdc, bit_map);
        if old_object.is_invalid() {
            DeleteDC(hdc);
            DeleteObject(bit_map);
            ReleaseDC(HWND::default(), hdc_screen);
            return Err("Could not select object from Device Context".to_string());
        }

        // The function draws the source bitmap to a destination rectangle.
        // In this case it receives the bitmap of the screen and writes it into the compatible device context.
        // It returns a bool indicating if the operation was succesful.
        let sb = StretchBlt(
            hdc, 0, 0, width, height, hdc_screen, x, y, width, height, SRCCOPY,
        );
        if sb == false {
            DeleteDC(hdc);
            DeleteObject(bit_map);
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

        // The function retrieves the bits of the specified compatible bitmap and copies them into a buffer as a DIB using the specified format
        let gdb = GetDIBits(
            hdc,
            bit_map,
            0,
            height as u32,
            Some(data.as_mut_ptr() as *mut core::ffi::c_void),
            &mut bit_map_info,
            DIB_RGB_COLORS,
        );

        if gdb == 0 || gdb == ERROR_INVALID_PARAMETER.0 as i32 {
            DeleteDC(hdc);
            DeleteObject(bit_map);
            ReleaseDC(HWND::default(), hdc_screen);
            return Err("Could not fetch DI Bits".to_string());
        }

        data.chunks_exact_mut(4).for_each(|c| c.swap(0, 2));

        DeleteDC(hdc);
        DeleteObject(bit_map);
        ReleaseDC(HWND::default(), hdc_screen);

        Ok(ScreenData {
            data,
            height,
            width,
        })
    }
}

pub fn screenshot(extension: &str) -> Result<(), String> {
    let s = capture_screen()?;
    let img = RgbaImage::from_raw(s.width as u32, s.height as u32, s.data)
        .ok_or_else(|| "Could not obtain image from raw data".to_string())?;

    img.save(extension).map_err(|err| err.to_string())?;
    Ok(())
}
