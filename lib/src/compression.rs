use std::io::{Read, Write};

const LG_WINDOW_SIZE: u32 = 21;
const QUALITY: u32 = 11;

pub fn compress(bytes: &[u8]) -> crate::Result<Vec<u8>> {
    let mut buf = Vec::new();
    let mut writer = brotli::CompressorWriter::new(&mut buf, 4096, QUALITY, LG_WINDOW_SIZE);

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
