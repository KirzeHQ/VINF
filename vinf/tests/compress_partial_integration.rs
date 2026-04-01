use vinf::Vinf;
use vinf::basic_hash;

#[test]
fn compress_finds_candidates() {
  let mut v = Vinf::new();
  let known = basic_hash(b"candidate");
  v.register_known_hash(known);
  v.compress(b"candidate").expect("compress ok");
  let cands = v.last_candidates();
  assert!(cands.contains(&known));
}
