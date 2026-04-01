use vinf::hash::{
  LAYER_SIZES, Layer, NODE_DIGEST_BYTES, get_layer_slice, partial_layer_match, partial_match_up_to,
};

#[test]
fn partial_hash_matching() {
  let mut a = [0u8; NODE_DIGEST_BYTES];
  let mut b = [0u8; NODE_DIGEST_BYTES];

  let sizes = LAYER_SIZES;
  let offs = [
    0usize,
    sizes[0],
    sizes[0] + sizes[1],
    sizes[0] + sizes[1] + sizes[2],
  ];

  for i in offs[0]..offs[1] {
    a[i] = 1;
    b[i] = 1;
  }

  for i in offs[1]..offs[2] {
    a[i] = 2;
    b[i] = 3;
  }

  for i in offs[2]..offs[3] {
    a[i] = 4;
    b[i] = 4;
  }

  assert!(partial_layer_match(&a, &b, Layer::X));
  assert!(!partial_layer_match(&a, &b, Layer::Y));
  assert!(partial_layer_match(&a, &b, Layer::Z));

  assert!(partial_match_up_to(&a, &b, Layer::X));
  assert!(!partial_match_up_to(&a, &b, Layer::Y));
  assert!(!partial_match_up_to(&a, &b, Layer::Z));

  let sx = get_layer_slice(&a, Layer::X);
  let sy = get_layer_slice(&a, Layer::Y);
  let sz = get_layer_slice(&a, Layer::Z);
  assert_eq!(sx.len(), sizes[0]);
  assert_eq!(sy.len(), sizes[1]);
  assert_eq!(sz.len(), sizes[2]);
}
