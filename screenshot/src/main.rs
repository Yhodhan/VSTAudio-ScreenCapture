mod screenshot;

fn main() -> Result<(), String> {
    screenshot::screenshot()?;
    Ok(())
}
