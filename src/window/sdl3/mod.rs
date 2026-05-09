use anyhow::Result;

pub struct Window {
    context: sdl3::Sdl,
    canvas: sdl3::render::Canvas<sdl3::video::Window>,
    texture: sdl3::render::Texture,
}

impl Window {
    pub fn new(width: impl Into<u32>, height: impl Into<u32>) -> Result<Self> {
        use sdl3::pixels::PixelFormat;
        use sdl3::sys::pixels::SDL_PixelFormat;

        let width = width.into();
        let height = height.into();

        let context = sdl3::init()?;
        let video = context.video()?;
        let window = video
            .window("chip8 emulator", 1280, 720)
            .resizable()
            .position_centered()
            .build()?;

        let mut canvas = window.into_canvas();
        canvas.set_logical_size(
            width,
            height,
            sdl3::sys::render::SDL_RendererLogicalPresentation::LETTERBOX,
        )?;
        canvas.present();

        let mut texture = canvas.texture_creator().create_texture_streaming(
            unsafe { PixelFormat::from_ll(SDL_PixelFormat::RGB565) },
            width,
            height,
        )?;
        texture.set_scale_mode(sdl3::render::ScaleMode::Nearest);

        Ok(Self {
            context,
            texture,
            canvas,
        })
    }

    pub fn draw(&mut self, bytes: &[u8]) -> Result<()> {
        const RGB565_SIZE: usize = 2;
        const PIXELS_PER_BYTE: usize = u8::BITS as usize;

        self.texture
            .with_lock(None, |texture_buffer: &mut [u8], _: usize| {
                for (byte_offset, byte) in bytes.iter().enumerate() {
                    for pixel_offset in 0..PIXELS_PER_BYTE {
                        let offset = (byte_offset * PIXELS_PER_BYTE * RGB565_SIZE)
                            + (pixel_offset * RGB565_SIZE);

                        let color = if (0x80u8 >> pixel_offset) & byte > 0 {
                            0xFF
                        } else {
                            0x00
                        };

                        texture_buffer[offset] = color;
                        texture_buffer[offset + 1] = color;
                    }
                }
            })?;

        self.render_texture()
    }

    pub fn clear(&mut self) -> Result<()> {
        use sdl3::pixels::Color;

        self.canvas.set_draw_color(Color::RGB(0, 0x4F, 0));
        self.canvas.clear();
        self.canvas.present();

        Ok(())
    }

    pub fn pool(&mut self) -> Result<bool> {
        use sdl3::event::Event;
        use sdl3::keyboard::Keycode;

        let mut event_pump = self.context.event_pump()?;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return Ok(false),
                Event::Window {
                    win_event: sdl3::event::WindowEvent::Resized(width, height),
                    ..
                } => {
                    let window = self.canvas.window_mut();
                    window.set_size(width.try_into()?, height.try_into()?)?;

                    self.render_texture()?;
                }
                _ => {
                    println!("Event {:?}", event)
                }
            }
        }

        Ok(true)
    }

    fn render_texture(&mut self) -> Result<()> {
        use sdl3::pixels::Color;
        use sdl3::rect::Rect;

        self.canvas.set_draw_color(Color::RGB(0, 0x4F, 0));
        self.canvas.clear();

        let render_size = Rect::new(0, 0, self.texture.width(), self.texture.height());
        self.canvas.copy(&self.texture, None, render_size)?;
        self.canvas.present();

        Ok(())
    }
}
