pub mod core;
pub mod errors;
pub mod hash;
pub mod prelude;
pub mod types;

pub use crate::core::{Vinf, compress, decompress};
pub use crate::errors::Error;
pub use crate::hash::{Graph as VinfGraph, NODE_DIGEST_BYTES, Node as VinfNode, build_vinf_hash};
pub use crate::types::*;

pub const VINF_LIB_VERSION: &str = "0.1.0";

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn smoke() {
    let v = Vinf::new();
    let data = b"hello";
    let enc = v.compress(data).expect("compress should succeed");
    let dec = v.decompress(&enc).expect("decompress should succeed");

    assert!(enc.is_empty());
    assert!(dec.is_empty());
  }
}
