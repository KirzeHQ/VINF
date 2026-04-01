use vinf::hash::{Graph, Node, build_vinf_hash};

#[test]
fn build_vinf_hash_smoke() -> Result<(), Box<dyn std::error::Error>> {
  let mut graph = Graph::new();
  graph.meta = b"example-meta".to_vec();

  let node = Node::new(1, b"text/plain".to_vec(), b"abc".to_vec());
  graph.nodes.push(node);

  let mut out = Vec::new();
  build_vinf_hash(&mut graph, 1, 64, &mut out)?;

  let hex = out.iter().map(|b| format!("{:02x}", b)).collect::<String>();

  let expected = "748ec6c17f8b14a28cf1f9fbdb3c0a28769b712663de83b5da0f0761970aad07992e2608e879647a341bc8c179426607c1a4b480956503651f62edfa9c13628b";
  assert_eq!(out.len(), 64);
  assert_eq!(hex, expected);
  Ok(())
}
