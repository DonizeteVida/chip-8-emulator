mod hardware;
mod io;
mod util;
mod window;

use anyhow::Result;

fn main() -> Result<()> {
    let rom_data = io::load_rom()?;
    let mut window = window::Window::new();
    let mut chip8 = hardware::Chip8::new();

    Ok(())
}
