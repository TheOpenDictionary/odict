use std::{
    error::Error,
    io::{Read, Write},
};

use lz4_flex::frame::{FrameDecoder, FrameEncoder};

pub fn compress(bytes: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let buf = Vec::new();
    let mut enc = FrameEncoder::new(buf);

    enc.write_all(&bytes)?;

    let compressed = enc.finish()?;

    Ok(compressed)
}

pub fn decompress(bytes: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut buf = Vec::new();
    let mut dec = FrameDecoder::new(bytes);

    dec.read_to_end(&mut buf)?;

    Ok(buf)
}
