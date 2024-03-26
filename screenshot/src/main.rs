use image::RgbaImage;

mod handler;
use handler::capture_screen;

fn main() {
    let s = capture_screen().unwrap();

    let img = RgbaImage::from_raw(s.width as u32, s.height as u32, s.data).unwrap();
    img.save("screenshot.bmp").unwrap();
}
