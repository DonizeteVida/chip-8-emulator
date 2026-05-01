mod util;
mod window;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let rom = std::env::args()
        .nth(1)
        .context("ch8 rom name param needed")?;
    let bytes = std::fs::read(rom)?;
    util::dump(&bytes);

    let mut window = window::Window::new();

    Ok(())
}
