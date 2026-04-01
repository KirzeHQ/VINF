use crate::errors::Error;
use crate::hash::{Layer, partial_match_up_to};
use crate::hash::{NODE_DIGEST_BYTES, basic_hash};

pub struct Vinf {
  last_hash: Option<[u8; NODE_DIGEST_BYTES]>,
  known_hashes: Vec<[u8; NODE_DIGEST_BYTES]>,
  last_candidates: Vec<[u8; NODE_DIGEST_BYTES]>,
  known_data: Vec<([u8; NODE_DIGEST_BYTES], Vec<u8>)>,
}

impl Default for Vinf {
  fn default() -> Self {
    Self::new()
  }
}

impl Vinf {
  pub fn new() -> Self {
    Vinf {
      last_hash: None,
      known_hashes: Vec::new(),
      last_candidates: Vec::new(),
      known_data: Vec::new(),
    }
  }

  pub fn last_hash(&self) -> Option<[u8; NODE_DIGEST_BYTES]> {
    self.last_hash
  }

  pub fn register_known_hash(&mut self, h: [u8; NODE_DIGEST_BYTES]) {
    self.known_hashes.push(h);
  }

  pub fn register_known_data(&mut self, data: &[u8]) -> [u8; NODE_DIGEST_BYTES] {
    let h = basic_hash(data);
    self.known_hashes.push(h);
    self.known_data.push((h, data.to_vec()));
    h
  }

  pub fn last_candidates(&self) -> Vec<[u8; NODE_DIGEST_BYTES]> {
    self.last_candidates.clone()
  }

  pub fn save_vhash<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), Error> {
    if let Some(h) = self.last_hash {
      crate::hash::write_vhash_file(path, &h, &self.last_candidates)?;
      Ok(())
    } else {
      Err(Error::Other("no last_hash available".to_string()))
    }
  }

  pub fn load_vhash<P: AsRef<std::path::Path>>(&mut self, path: P) -> Result<(), Error> {
    let (h, cands) = crate::hash::read_vhash_file(path)?;
    self.known_hashes.push(h);
    for c in cands {
      self.known_hashes.push(c);
    }
    Ok(())
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

    let mut out = Vec::new();
    out.extend_from_slice(b"VINF");
    out.push(1u8);
    out.push(0u8);
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&(data.len() as u64).to_le_bytes());
    out.extend_from_slice(&h);
    let count = candidates.len() as u16;
    out.extend_from_slice(&count.to_le_bytes());
    for c in &candidates {
      out.extend_from_slice(c);
    }
    Ok(out)
  }

  pub fn decompress(&self, _vinf_bytes: &[u8]) -> Result<Vec<u8>, Error> {
    let vinf_bytes = _vinf_bytes;
    if vinf_bytes.len() < 50 {
      return Err(Error::InvalidFormat);
    }
    if &vinf_bytes[0..4] != b"VINF" {
      return Err(Error::InvalidFormat);
    }

    let orig_len = u64::from_le_bytes(vinf_bytes[8..16].try_into().unwrap()) as usize;
    let mut offset = 50usize;
    let count = u16::from_le_bytes(vinf_bytes[48..50].try_into().unwrap()) as usize;
    for _ in 0..count {
      if vinf_bytes.len() < offset + NODE_DIGEST_BYTES {
        return Err(Error::InvalidFormat);
      }
      let mut cand = [0u8; NODE_DIGEST_BYTES];
      cand.copy_from_slice(&vinf_bytes[offset..offset + NODE_DIGEST_BYTES]);
      offset += NODE_DIGEST_BYTES;
      if let Some(data) = self.find_data_by_hash(&cand) {
        let mut out = data.clone();
        out.truncate(orig_len);
        return Ok(out);
      }
    }

    Ok(Vec::new())
  }

  fn find_data_by_hash(&self, h: &[u8; NODE_DIGEST_BYTES]) -> Option<Vec<u8>> {
    for (kh, data) in &self.known_data {
      if kh == h {
        return Some(data.clone());
      }
    }
    None
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
