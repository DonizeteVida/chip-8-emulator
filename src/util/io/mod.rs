use anyhow::Result;

#[cfg(debug_assertions)]
pub fn dump(bytes: &[u8]) {
    for (index, byte) in bytes.as_chunks::<2>().0.iter().enumerate() {
        if index % 8 == 0 {
            println!();
            print!("{:08x} ", index * 2)
        }

        print!("{:02x}", byte[1]);
        print!("{:02x} ", byte[0]);
    }

    println!()
}

#[cfg(not(debug_assertions))]
pub fn dump(bytes: &[u8]) {}

#[cfg(debug_assertions)]
fn get_rom_name() -> Result<String> {
    Ok("ibm.ch8".to_owned())
}

#[cfg(not(debug_assertions))]
fn get_rom_name() -> Result<String> {
    use anyhow::Context;

    let rom_name = std::env::args()
        .nth(1)
        .context("ch8 rom name param needed")?;

    Ok(rom_name)
}

pub fn load_rom() -> Result<Vec<u8>> {
    let rom_name = get_rom_name()?;
    let bytes = std::fs::read(rom_name)?;

    dump(&bytes);

    Ok(bytes)
}
