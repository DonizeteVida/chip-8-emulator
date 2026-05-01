mod hardware;
mod io;
mod macros;
mod util;
mod window;

fn main() -> anyhow::Result<()> {
    let data = io::load_rom()?;
    let mut window = window::Window::new();
    let mut chip8 = hardware::Chip8::new(&data);

    loop {
        let (a, b) = chip8.fetch();
        let (c, d) = chip8.fetch();

        match (a, b, c, d) {
            nibbles!(0, 0, E, 0) => println!("Clear screen"),
            nibbles!(_, _, _, _) => std::panic!("Not Implemented: {:x}{:x}{:x}{:x}", a, b, c, d),
        }
    }
}
