use crate::errors::Error;
use crate::hash::{basic_hash, NODE_DIGEST_BYTES};

pub struct Vinf {
  last_hash: Option<[u8; NODE_DIGEST_BYTES]>,
}

impl Default for Vinf {
  fn default() -> Self {
    Self::new()
  }
}

impl Vinf {
  pub fn new() -> Self {
    Vinf { last_hash: None }
  }

  pub fn last_hash(&self) -> Option<[u8; NODE_DIGEST_BYTES]> {
    self.last_hash
  }

  pub fn compress(&mut self, data: &[u8]) -> Result<Vec<u8>, Error> {
    
    
    let h = basic_hash(data);
    self.last_hash = Some(h);

    
    Ok(Vec::new())
  }

  pub fn decompress(&self, _vinf_bytes: &[u8]) -> Result<Vec<u8>, Error> {
    Ok(Vec::new())
  }
}

pub fn compress(data: &[u8]) -> Result<Vec<u8>, Error> {
  let mut v = Vinf::new();
  v.compress(data)
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
