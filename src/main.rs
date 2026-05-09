mod audio;
mod hardware;
mod io;
mod macros;
mod util;
mod window;

fn main() -> anyhow::Result<()> {
    let mut chip8 = hardware::Chip8::new(io::load_rom()?);
    let mut window = window::Window::new(chip8.width, chip8.height)?;
    let mut audio = audio::Audio::new()?;

    loop {
        let (a, b) = chip8.fetch();
        let (c, d) = chip8.fetch();

        match (a, b, c, d) {
            nibbles!(0, 0, E, 0) => window.clear()?,
            nibbles!(0, 0, E, E) => chip8.ret(),
            nibbles!(1, _, _, _) => chip8.jmp(joinibble!(b, c, d)),
            nibbles!(2, _, _, _) => chip8.call(joinibble!(b, c, d)),
            nibbles!(3, _, _, _) => chip8.eq(b, joinibble!(c d)),
            nibbles!(4, _, _, _) => chip8.nenn(b, joinibble!(c d)),
            nibbles!(5, _, _, 0) => chip8.skip(b, c),
            nibbles!(6, _, _, _) => chip8.setx(b, joinibble!(c d)),
            nibbles!(7, _, _, _) => chip8.addn(b, joinibble!(c d)),
            nibbles!(8, _, _, 0) => chip8.stxy(b, c),
            nibbles!(8, _, _, 1) => chip8.or(b, c),
            nibbles!(8, _, _, 2) => chip8.and(b, c),
            nibbles!(8, _, _, 3) => chip8.xor(b, c),
            nibbles!(8, _, _, 4) => chip8.addx(b, c),
            nibbles!(8, _, _, 5) => chip8.subx(b, c),
            nibbles!(8, _, _, 6) => chip8.shir(b),
            nibbles!(8, _, _, 7) => chip8.suby(b, c),
            nibbles!(8, _, _, E) => chip8.shil(b),
            nibbles!(9, _, _, 0) => chip8.nexy(b, c),
            nibbles!(A, _, _, _) => chip8.seti(joinibble!(b, c, d)),
            nibbles!(B, _, _, _) => chip8.jmpi(joinibble!(b, c, d)),
            nibbles!(C, _, _, _) => chip8.rand(b, joinibble!(c d)),
            nibbles!(D, _, _, _) => chip8.draw(b, c, d, |buffer| window.draw(buffer))?,
            nibbles!(_, _, _, _) => std::panic!("Not Implemented: {:x}{:x}{:x}{:x}", a, b, c, d),
        }

        if !window.pool()? {
            break;
        }
    }

    Ok(())
}
