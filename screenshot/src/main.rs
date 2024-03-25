mod display_info;
mod screenshot;

fn main() {
    unsafe {
        screenshot::screenshot();
    }
}
