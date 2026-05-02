const WHITE_SQUARE: char = '\u{2588}';

pub struct Window;

impl Window {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&mut self, buffer: &[u8], width: u8) -> anyhow::Result<()> {
        let pixels_per_unit = u8::BITS as usize;
        let real_width = width as usize / pixels_per_unit;

        for (index, byte) in buffer.iter().enumerate() {
            if index % real_width == 0 {
                println!()
            }

            for pixel_offset in 0..pixels_per_unit {
                if (0x80u8 >> pixel_offset) & byte > 0 {
                    print!("{WHITE_SQUARE}")
                } else {
                    print!(" ")
                }
            }
        }

        Ok(())
    }

    pub fn clear(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}
