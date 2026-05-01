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
