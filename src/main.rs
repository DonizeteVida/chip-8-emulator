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
            nibbles!(0, 0, E, E) => chip8.ret(),
            nibbles!(1, _, _, _) => chip8.jmp(joinibble!(b, c, d)),
            nibbles!(2, _, _, _) => chip8.call(joinibble!(b, c, d)),
            nibbles!(3, _, _, _) => chip8.eq(b, joinibble!(c d)),
            nibbles!(4, _, _, _) => chip8.neq(b, joinibble!(c d)),
            nibbles!(5, _, _, 0) => chip8.skip(b, c),
            nibbles!(6, _, _, _) => chip8.setx(b, joinibble!(c d)),
            nibbles!(7, _, _, _) => chip8.addx(b, joinibble!(c d)),
            nibbles!(A, _, _, _) => chip8.seti(joinibble!(b, c, d)),
            nibbles!(B, _, _, _) => chip8.jmpi(joinibble!(b, c, d)),
            nibbles!(_, _, _, _) => std::panic!("Not Implemented: {:x}{:x}{:x}{:x}", a, b, c, d),
        }
    }
}
