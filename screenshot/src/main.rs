use image::RgbaImage;
// use regex::Regex;
use win_screenshot::prelude::*;

fn main() {
    let buf = capture_display().unwrap();
    let img = RgbaImage::from_raw(buf.width, buf.height, buf.pixels).unwrap();
    img.save("screenshot.bmp").unwrap();
}
