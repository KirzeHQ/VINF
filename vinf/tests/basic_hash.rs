use vinf::hash::{basic_hash, xof_bytes};

#[test]
fn basic_hash_and_xof_smoke() -> Result<(), Box<dyn std::error::Error>> {
  let data = b"hello world";
  let h1 = basic_hash(data);
  let h2 = basic_hash(data);
  assert_eq!(h1, h2);
  assert!(h1.iter().any(|&b| b != 0));

  let x = xof_bytes(data, 48)?;
  assert_eq!(x.len(), 48);
  assert!(x.iter().any(|&b| b != 0));

  Ok(())
}
