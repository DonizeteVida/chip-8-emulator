const WHITE_SQUARE: char = '\u{2588}';

pub struct Window;

impl Window {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&mut self, buffer: &[u8], width: u8) -> anyhow::Result<()> {
        let width = width / u8::BITS as u8;
        for i in 0..buffer.len() {
            if i % width as usize == 0 {
                println!()
            }
            let byte = buffer[i];
            for i in 0..u8::BITS as usize {
                if (0b10000000 >> i) & byte > 0 {
                    print!("{WHITE_SQUARE}")
                } else {
                    print!(" ")
                }
            }
        }

        Ok(())
    }

    #[cfg(unix)]
    pub fn clear(&mut self) -> anyhow::Result<()>  {
        use std::io::Write;
        print!("\x1B[2J\x1B[1;1H");
        Ok(std::io::stdout().flush()?)
    }
}