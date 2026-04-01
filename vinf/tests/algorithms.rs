use vinf::Vinf;
use vinf::hash::{Graph, Node, build_vinf_hash};

#[test]
fn build_vinf_hash_rounds_change() -> Result<(), Box<dyn std::error::Error>> {
  let mut g1 = Graph::new();
  g1.meta = b"meta".to_vec();
  g1.nodes
    .push(Node::new(1, b"text/plain".to_vec(), b"abc".to_vec()));
  let mut out1 = Vec::new();
  build_vinf_hash(&mut g1, 1, 64, &mut out1)?;

  let mut g2 = Graph::new();
  g2.meta = b"meta".to_vec();
  g2.nodes
    .push(Node::new(1, b"text/plain".to_vec(), b"abc".to_vec()));
  let mut out2 = Vec::new();
  build_vinf_hash(&mut g2, 2, 64, &mut out2)?;

  assert_eq!(out1.len(), 64);
  assert_eq!(out2.len(), 64);
  assert_ne!(out1, out2);
  Ok(())
}

#[test]
fn compress_decompress_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
  let mut v = Vinf::new();
  let data = b"roundtrip";
  v.register_known_data(data);
  let blob = v.compress(data)?;
  let restored = v.decompress(&blob)?;
  assert_eq!(restored, data.to_vec());
  Ok(())
}
