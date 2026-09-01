mod hardware;
mod util;

fn main() -> anyhow::Result<()> {
    let mut cpu = hardware::Cpu::new(util::io::load_rom()?);
    let mut window = hardware::Window::new(cpu.width, cpu.height)?;
    let mut audio = hardware::Audio::new()?;

    loop {
        let (a, b) = cpu.fetch();
        let (c, d) = cpu.fetch();

        match (a, b, c, d) {
            nibbles!(0, 0, E, 0) => window.clear()?,
            nibbles!(0, 0, E, E) => cpu.ret(),
            nibbles!(1, _, _, _) => cpu.jmp(joinibble!(b, c, d)),
            nibbles!(2, _, _, _) => cpu.call(joinibble!(b, c, d)),
            nibbles!(3, _, _, _) => cpu.eq(b, joinibble!(c d)),
            nibbles!(4, _, _, _) => cpu.nenn(b, joinibble!(c d)),
            nibbles!(5, _, _, 0) => cpu.skip(b, c),
            nibbles!(6, _, _, _) => cpu.setx(b, joinibble!(c d)),
            nibbles!(7, _, _, _) => cpu.addn(b, joinibble!(c d)),
            nibbles!(8, _, _, 0) => cpu.stxy(b, c),
            nibbles!(8, _, _, 1) => cpu.or(b, c),
            nibbles!(8, _, _, 2) => cpu.and(b, c),
            nibbles!(8, _, _, 3) => cpu.xor(b, c),
            nibbles!(8, _, _, 4) => cpu.addx(b, c),
            nibbles!(8, _, _, 5) => cpu.subx(b, c),
            nibbles!(8, _, _, 6) => cpu.shir(b),
            nibbles!(8, _, _, 7) => cpu.suby(b, c),
            nibbles!(8, _, _, E) => cpu.shil(b),
            nibbles!(9, _, _, 0) => cpu.nexy(b, c),
            nibbles!(A, _, _, _) => cpu.seti(joinibble!(b, c, d)),
            nibbles!(B, _, _, _) => cpu.jmpi(joinibble!(b, c, d)),
            nibbles!(C, _, _, _) => cpu.rand(b, joinibble!(c d)),
            nibbles!(D, _, _, _) => cpu.draw(b, c, d, |buffer| window.draw(buffer))?,
            nibbles!(F, _, 0, 7) => cpu.getd(b),
            nibbles!(F, _, 0, A) => cpu.getk(b, || window.get_key())?,
            nibbles!(F, _, 1, 5) => cpu.setd(b),
            nibbles!(F, _, 1, 8) => cpu.sets(b),
            nibbles!(_, _, _, _) => std::panic!("Not Implemented: {:x}{:x}{:x}{:x}", a, b, c, d),
        }

        if !window.pool()? {
            break;
        }

        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    Ok(())
}
