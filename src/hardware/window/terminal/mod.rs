use anyhow::Result;

const WHITE_SQUARE: char = '\u{2588}';

pub struct Window {
    width: u32,
    height: u32,
}

impl Window {
    pub fn new<T: Into<u32>>(width: T, height: T) -> Result<Self> {
        let width = width.into();
        let height = height.into();

        Ok(Self { width, height })
    }

    pub fn draw(&self, buffer: &[u8]) -> Result<()> {
        let pixels_per_unit = u8::BITS as usize;
        let real_width = self.width as usize / pixels_per_unit;

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

    pub fn clear(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn pool(&mut self) -> Result<bool> {
        Ok(true)
    }

    pub fn get_key(&self) -> Result<u8> {
        Ok(0)
    }
}
