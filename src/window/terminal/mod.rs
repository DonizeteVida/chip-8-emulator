const BLACK_SQUARE: u32 = 0x25A0;
const WHITE_SQUARE: u32 = 0x25A1;

pub struct Window;

impl Window {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&mut self, buffer: &[u8]) {
        
    }

    #[cfg(unix)]
    pub fn clear(&mut self) -> anyhow::Result<()> {
        use std::io::Write;
        print!("\x1B[2J\x1B[1;1H");
        Ok(std::io::stdout().flush()?)
    }
}