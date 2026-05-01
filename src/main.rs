mod hardware;
mod io;
mod util;
mod window;

use anyhow::Result;

fn main() -> Result<()> {
    let data = io::load_rom()?;
    let mut window = window::Window::new();
    let mut chip8 = hardware::Chip8::new(&data);

    Ok(())
}
