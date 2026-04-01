use std::collections::BTreeMap;
use std::io::{self, Write};

pub const ROT: [u32; 4] = [32, 24, 16, 63];
pub const R_P: usize = 12;
pub const NODE_DIGEST_BYTES: usize = 32;
pub const RATE_BYTES: usize = 32;

pub const DOMAIN_NODE: u8 = 0x4E;
pub const DOMAIN_MIX: u8 = 0x4D;
pub const DOMAIN_FINAL: u8 = 0x46;
pub const DOMAIN_NONE: u8 = 0x00;

const RC_SEED: u128 = 0x9E3779B97F4A7C15u128;
const RC_XOR_CONST: u64 = 0xC0FFEE1234567890u64;

fn gen_round_constants() -> [[u64; 8]; R_P] {
  let mut rc = [[0u64; 8]; R_P];
  for r in 0..R_P {
    for i in 0..8 {
      let mul = ((r as u128) * 0x9Fu128).wrapping_add((i as u128 + 1));
      let v128 = RC_SEED.wrapping_mul(mul);
      let low64 = v128 as u64;
      let mut val = low64;
      val ^= RC_XOR_CONST.wrapping_add(((r as u64) << 8) | (i as u64));
      rc[r][i] = val;
    }
  }
  rc
}

pub fn vinf_p(state: &mut [u64; 8], rc: &[[u64; 8]; R_P]) {
  for r in 0..R_P {
    for i in 0..8 {
      state[i] = state[i].wrapping_add(rc[r][i]);
    }
    for i in 0..4 {
      let mut a = state[i];
      let mut b = state[i + 4];
      a = a.wrapping_add(b).wrapping_add(rc[r][i]);
      b = (b ^ a).rotate_left(ROT[i]);
      state[i] = a;
      state[i + 4] = b;
    }
    let s = [
      state[0], state[5], state[2], state[7], state[4], state[1], state[6], state[3],
    ];
    state.copy_from_slice(&s);
    for i in 0..8 {
      state[i] ^= state[(i + 3) % 8];
    }
  }
}

fn encode_u64_le(x: u64) -> [u8; 8] {
  x.to_le_bytes()
}

fn absorb(state: &mut [u64; 8], domain: u8, data: &[u8], rc: &[[u64; 8]; R_P]) {
  let mut stream = Vec::with_capacity(1 + data.len());
  stream.push(domain);
  stream.extend_from_slice(data);

  let mut pos = 0usize;
  while pos < stream.len() {
    let end = usize::min(pos + RATE_BYTES, stream.len());
    let mut block = [0u8; RATE_BYTES];
    block[..end - pos].copy_from_slice(&stream[pos..end]);
    if end - pos < RATE_BYTES {
      block[end - pos] = 0x01;
    }
    for i in 0..4 {
      let off = i * 8;
      let word = u64::from_le_bytes(block[off..off + 8].try_into().unwrap());
      state[i] ^= word;
    }
    vinf_p(state, rc);
    pos += RATE_BYTES;
  }
}

fn squeeze(
  state: &mut [u64; 8],
  wanted: usize,
  rc: &[[u64; 8]; R_P],
  writer: &mut impl Write,
) -> io::Result<()> {
  let mut out = Vec::with_capacity(wanted);
  while out.len() < wanted {
    for i in 0..4 {
      out.extend_from_slice(&state[i].to_le_bytes());
    }
    if out.len() >= wanted {
      break;
    }
    vinf_p(state, rc);
  }
  writer.write_all(&out[..wanted])
}

pub fn h_node(data: &[u8], rc: &[[u64; 8]; R_P]) -> [u8; NODE_DIGEST_BYTES] {
  let mut state = [0u64; 8];
  let mut input = Vec::with_capacity(8 + data.len());
  input.extend_from_slice(&encode_u64_le(data.len() as u64));
  input.extend_from_slice(data);
  absorb(&mut state, DOMAIN_NODE, &input, rc);
  let mut out = Vec::with_capacity(NODE_DIGEST_BYTES);
  squeeze(&mut state, NODE_DIGEST_BYTES, rc, &mut out).expect("squeeze failed");
  let mut arr = [0u8; NODE_DIGEST_BYTES];
  arr.copy_from_slice(&out[..NODE_DIGEST_BYTES]);
  arr
}

#[derive(Debug, Clone)]
pub struct Node {
  pub id: u64,
  pub metadata: Vec<u8>,
  pub content: Vec<u8>,
  pub hashs: Vec<[u8; NODE_DIGEST_BYTES]>,
}

impl Node {
  pub fn new(id: u64, metadata: Vec<u8>, content: Vec<u8>) -> Self {
    Self {
      id,
      metadata,
      content,
      hashs: Vec::new(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Edge {
  pub ty: u8,
  pub to: u64,
}

#[derive(Debug, Clone)]
pub struct Graph {
  pub nodes: Vec<Node>,
  pub edges: BTreeMap<u64, Vec<Edge>>,
  pub meta: Vec<u8>,
}

impl Graph {
  pub fn new() -> Self {
    Self {
      nodes: Vec::new(),
      edges: BTreeMap::new(),
      meta: Vec::new(),
    }
  }
}

pub fn build_vinf_hash(
  graph: &mut Graph,
  rounds: usize,
  out_len: usize,
  out: &mut impl Write,
) -> io::Result<()> {
  let rc = gen_round_constants();

  for node in &mut graph.nodes {
    let mut node_input = Vec::new();
    node_input.extend_from_slice(&node.id.to_le_bytes());
    node_input.extend_from_slice(&encode_u64_le(node.metadata.len() as u64));
    node_input.extend_from_slice(&node.metadata);
    node_input.extend_from_slice(&encode_u64_le(node.content.len() as u64));
    node_input.extend_from_slice(&node.content);
    let h0 = h_node(&node_input, &rc);
    node.hashs.push(h0);
  }

  for r in 0..rounds {
    let mut next_map: BTreeMap<u64, [u8; NODE_DIGEST_BYTES]> = BTreeMap::new();
    for node in &graph.nodes {
      let mut neighbors = graph.edges.get(&node.id).cloned().unwrap_or_else(Vec::new);
      neighbors.sort_by_key(|e| (e.ty, e.to));
      let mut concat = Vec::new();
      for n in &neighbors {
        if let Some(nei) = graph.nodes.iter().find(|x| x.id == n.to) {
          if let Some(h) = nei.hashs.get(r) {
            concat.extend_from_slice(h);
          }
        }
      }
      let mut mix_input = Vec::new();
      mix_input.extend_from_slice(&encode_u64_le(concat.len() as u64));
      mix_input.extend_from_slice(&concat);
      let mut full = Vec::new();
      full.push(DOMAIN_MIX);
      full.extend_from_slice(&node.hashs[r]);
      full.extend_from_slice(&mix_input);
      full.extend_from_slice(&encode_u64_le(r as u64));
      let nxt = h_node(&full, &rc);
      next_map.insert(node.id, nxt);
    }
    for node in &mut graph.nodes {
      if let Some(nxt) = next_map.get(&node.id) {
        node.hashs.push(*nxt);
      } else {
        node.hashs.push([0u8; NODE_DIGEST_BYTES]);
      }
    }
  }

  let mut sorted_nodes = graph.nodes.clone();
  sorted_nodes.sort_by_key(|n| n.id);
  let final_index = rounds;

  let mut state = [0u64; 8];
  let mut meta_blob = Vec::new();
  meta_blob.extend_from_slice(&encode_u64_le(graph.meta.len() as u64));
  meta_blob.extend_from_slice(&graph.meta);
  absorb(&mut state, DOMAIN_FINAL, &meta_blob, &rc);

  for n in &sorted_nodes {
    let h = n
      .hashs
      .get(final_index)
      .unwrap_or(&[0u8; NODE_DIGEST_BYTES]);
    absorb(&mut state, DOMAIN_NONE, h, &rc);
  }

  squeeze(&mut state, out_len, &rc, out)
}
