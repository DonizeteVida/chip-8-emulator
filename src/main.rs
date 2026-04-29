use anyhow::{Context, Result};

#[cfg(debug_assertions)]
fn dump(bytes: &[u8]) {
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

fn main() -> Result<()> {
    let rom = std::env::args().nth(1).context("ch8 rom not found")?;
    let bytes = std::fs::read(rom)?;

    #[cfg(debug_assertions)]
    dump(&bytes);

    Ok(())
}
