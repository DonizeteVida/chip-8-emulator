use anyhow::{Context, Result};

fn main() -> Result<()> {
    let rom = std::env::args().nth(1).context("ch8 rom not found")?;

    println!("{rom}");

    Ok(())
}
