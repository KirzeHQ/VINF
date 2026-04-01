use crate::errors::Error;
use crate::hash::{NODE_DIGEST_BYTES, basic_hash};
use crate::hash::{Layer, partial_match_up_to};

pub struct Vinf {
  last_hash: Option<[u8; NODE_DIGEST_BYTES]>,
  known_hashes: Vec<[u8; NODE_DIGEST_BYTES]>,
  last_candidates: Vec<[u8; NODE_DIGEST_BYTES]>,
}

impl Default for Vinf {
  fn default() -> Self {
    Self::new()
  }
}

impl Vinf {
  pub fn new() -> Self {
    Vinf { last_hash: None, known_hashes: Vec::new(), last_candidates: Vec::new() }
  }

  pub fn last_hash(&self) -> Option<[u8; NODE_DIGEST_BYTES]> {
    self.last_hash
  }

  pub fn register_known_hash(&mut self, h: [u8; NODE_DIGEST_BYTES]) {
    self.known_hashes.push(h);
  }

  pub fn last_candidates(&self) -> Vec<[u8; NODE_DIGEST_BYTES]> {
    self.last_candidates.clone()
  }

  fn find_partial_matches(&self, h: &[u8; NODE_DIGEST_BYTES]) -> Vec<[u8; NODE_DIGEST_BYTES]> {
    let mut scored: Vec<(u8, [u8; NODE_DIGEST_BYTES])> = Vec::new();
    for k in &self.known_hashes {
      let score = if partial_match_up_to(h, k, Layer::Z) {
        3
      } else if partial_match_up_to(h, k, Layer::Y) {
        2
      } else if partial_match_up_to(h, k, Layer::X) {
        1
      } else {
        0
      };
      if score > 0 {
        scored.push((score, *k));
      }
    }
    scored.sort_by(|a, b| b.0.cmp(&a.0));
    scored.into_iter().map(|(_, h)| h).collect()
  }

  pub fn compress(&mut self, data: &[u8]) -> Result<Vec<u8>, Error> {
    let h = basic_hash(data);
    self.last_hash = Some(h);

    
    let candidates = self.find_partial_matches(&h);
    self.last_candidates = candidates;

    
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
