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

  let expected = "9cafe71f122a33a62c2133a8a534f8c01a9c3b9d3aa9d73a84cf30bc8a934eccef03edb9203000eca1fcfca323dea8588dbc264db3c39497424716048aa9a64d";
  assert_eq!(out.len(), 64);
  assert_eq!(hex, expected);
  Ok(())
}
