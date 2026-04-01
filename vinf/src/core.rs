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
    self.last_candidates = candidates.clone();

    // Build a minimal VINF prototype blob:
    // layout:
    // 0..4   magic 'VINF'
    // 4      version u8
    // 5      flags u8
    // 6..8   reserved u16
    // 8..16  original length u64 le
    // 16..48 per-file hash (32 bytes)
    // 48..50 candidate_count u16 le
    // 50..   candidate_count * 32 bytes candidate hashes
    let mut out = Vec::new();
    out.extend_from_slice(b"VINF");
    out.push(1u8); // version
    out.push(0u8); // flags
    out.extend_from_slice(&0u16.to_le_bytes()); // reserved
    out.extend_from_slice(&(data.len() as u64).to_le_bytes()); // original length
    out.extend_from_slice(&h); // per-file hash
    let count = candidates.len() as u16;
    out.extend_from_slice(&count.to_le_bytes());
    for c in &candidates {
      out.extend_from_slice(c);
    }
    Ok(out)
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
