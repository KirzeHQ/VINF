use vinf::hash::{Graph, Node, build_vinf_hash};

#[test]
fn build_vinf_hash_smoke() -> Result<(), Box<dyn std::error::Error>> {
  let mut graph = Graph::new();
  graph.meta = b"example-meta".to_vec();

  let node = Node::new(1, b"text/plain".to_vec(), b"abc".to_vec());
  graph.nodes.push(node);

  let mut out = Vec::new();

  build_vinf_hash(&mut graph, 1, 64, &mut out)?;

  assert_eq!(out.len(), 64);
  assert!(out.iter().any(|&b| b != 0));
  Ok(())
}
