#[cfg(target_os = "windows")]
use std::io;
mod screenshot;

fn read_format() -> Result<String, String> {
    println!("Choose the screenshot's format [png | bmp]: ");
    let mut format = String::new();
    io::stdin()
        .read_line(&mut format)
        .map_err(|err| err.to_string())?;

    Ok(format)
}

fn return_extension(format: &str) -> Result<String, String> {
    let format = format.strip_suffix("\r\n").or(format.strip_suffix("\n"));

    let extension = match format {
        Some(f) if f == "png" || f == "bmp" => format!("screenshot.{}", f),
        _ => return Err("Unknown or unsupported format".to_string()),
    };

    Ok(extension)
}

fn main() -> Result<(), String> {
    let format = read_format()?;
    let extension = return_extension(&format)?;
    screenshot::screenshot(&extension)?;
    Ok(())
}
