use anyhow::Result;

use crate::util;

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

    util::dump(&bytes);

    Ok(bytes)
}
