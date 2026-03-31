use crate::errors::Error;

pub struct Vinf {}

impl Vinf {
    pub fn new() -> Self {
        Vinf {}
    }

    pub fn compress(&self, _data: &[u8]) -> Result<Vec<u8>, Error> {
        Ok(Vec::new())
    }

    pub fn decompress(&self, _vinf_bytes: &[u8]) -> Result<Vec<u8>, Error> {
        Ok(Vec::new())
    }
}

pub fn compress(data: &[u8]) -> Result<Vec<u8>, Error> {
    Vinf::new().compress(data)
}

pub fn decompress(vinf_bytes: &[u8]) -> Result<Vec<u8>, Error> {
    Vinf::new().decompress(vinf_bytes)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coordinate {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}
