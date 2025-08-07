use std::io::{Read, Write};

#[derive(Debug)]
pub struct CompressOptions {
    pub quality: u32,
    pub window_size: u32,
}

impl AsRef<CompressOptions> for CompressOptions {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl CompressOptions {
    pub fn quality(mut self, quality: u32) -> Self {
        self.quality = quality;
        self
    }

    pub fn window_size(mut self, window_size: u32) -> Self {
        self.window_size = window_size;
        self
    }
}

impl Default for CompressOptions {
    fn default() -> Self {
        Self {
            quality: 8,
            window_size: 22,
        }
    }
}

pub fn compress<Options: AsRef<CompressOptions>>(
    bytes: &[u8],
    options: Options,
) -> crate::Result<Vec<u8>> {
    let opts = options.as_ref();

    let mut buf = Vec::new();
    let mut writer = brotli::CompressorWriter::new(&mut buf, 4096, opts.quality, opts.window_size);

    writer.write_all(bytes).expect("Failed to write data");
    writer.flush().expect("Failed to flush data");

    drop(writer);

    Ok(buf)
}

pub fn decompress(bytes: &[u8]) -> crate::Result<Vec<u8>> {
    let mut buf = Vec::new();
    let mut reader = brotli::Decompressor::new(bytes, 4096);

    reader.read_to_end(&mut buf).expect("Failed to read data");

    Ok(buf)
}
