use std::path::PathBuf;
use vinf::Vinf;
use vinf::basic_hash;

#[test]
fn save_and_load_vhash_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
  let mut v = Vinf::new();

  v.register_known_hash(basic_hash(b"candidate"));
  v.compress(b"candidate")?;

  let mut p = std::env::temp_dir();
  p.push(format!("vinf_test_{}.vhash", std::process::id()));
  let path: PathBuf = p.clone();

  v.save_vhash(&path)?;

  let mut v2 = Vinf::new();
  v2.load_vhash(&path)?;
  v2.compress(b"candidate")?;
  let cands = v2.last_candidates();
  assert!(!cands.is_empty());

  let _ = std::fs::remove_file(&path);
  Ok(())
}
