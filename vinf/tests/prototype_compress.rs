use vinf::{Vinf, basic_hash};

#[test]
fn prototype_compress_emits_blob_and_candidates() -> Result<(), Box<dyn std::error::Error>> {
  let mut v = Vinf::new();

  let out = v.compress(b"hello")?;
  assert!(out.len() >= 50);
  assert_eq!(&out[0..4], b"VINF");
  assert_eq!(out[4], 1u8);
  let orig_len = u64::from_le_bytes(out[8..16].try_into().unwrap());
  assert_eq!(orig_len, 5u64);
  let h = v.last_hash().expect("last_hash set");
  assert_eq!(&out[16..48], &h);
  let count = u16::from_le_bytes(out[48..50].try_into().unwrap());
  assert_eq!(count, 0u16);

  let known = basic_hash(b"hello");
  v.register_known_hash(known);
  let out2 = v.compress(b"hello")?;
  let count2 = u16::from_le_bytes(out2[48..50].try_into().unwrap());
  assert!(count2 >= 1);
  let cand = &out2[50..50 + 32];
  assert_eq!(cand, &known[..]);

  Ok(())
}
