use anyhow::Result;

pub struct Audio {
    device: cpal::Device,
}

impl Audio {
    pub fn new() -> Result<Self> {
        use anyhow::Context;
        use cpal::traits::HostTrait;

        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .context("Default output device not found")?;

        Ok(Self { device })
    }
}
