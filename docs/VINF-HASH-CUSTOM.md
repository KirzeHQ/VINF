# VINF-HASH

This document is the authoritative VINF custom-hash specification (VINF-HASH-V1).

Purpose
- Produce a deterministic, locality-aware, graph-based fingerprint for VINF graphs.
- Default final fingerprint length: `L = 10 MiB` (configurable). The algorithm is streaming-friendly.

Quick implementer checklist
- Implement `vinf_p(state: &mut [u64; 8], rc: &[[u64;8]; R_P])` — the permutation.
- Implement sponge helpers: `absorb(state, domain: u8, data: &[u8], rc)` and `squeeze(state, out_len, rc, writer)`.
- Implement `h_node(data: &[u8], rc) -> [u8; NODE_DIGEST_BYTES]`.
- Implement top-level `build_vinf_hash(graph: &Graph, rounds: usize, out_len: usize, out: &mut impl Write)`.
- Add deterministic canonical serialization helpers `encode_u64_le`, `encode_len_prefixed`.

Concrete parameters (V1)
- Word size: 64-bit (`u64`), little-endian encoding.
- Internal state: `n_w = 8` words (state = 8 * 8 = 64 bytes).
- Rate words: `r_w = 4` → `rate_bytes = 32`.
- Capacity words: `c_w = 4` (32 bytes).
- Permutation rounds: `R_P = 12`.
- Node digest: `NODE_DIGEST_BYTES = 32`.
- Default output length: `L = 10 * 1024 * 1024` bytes.
- Algorithm ID: `VINF-HASH-V1`.

Round-constant generator (exact)
1. `RC_SEED = 0x9E3779B97F4A7C15u128` (64-bit seed used in 128-bit arithmetic).
2. For each round `r` and lane `i` compute:

   RC(r,i) = low64( RC_SEED * ( (r as u128) * 0x9F + (i as u128 + 1) ) )
   RC(r,i) ^= 0xC0FFEE1234567890u64.wrapping_add(((r as u64) << 8) | (i as u64));

3. Store `rc: [[u64; 8]; R_P]` and reuse it in `vinf_p`.

Permutation: `vinf_p` (implement exactly)
 - Rotation constants: `ROT = [32, 24, 16, 63]`
 - Steps per round r:
   1. Add round constants: `state[i] = state[i].wrapping_add(rc[r][i])` for i=0..7
   2. For i=0..3: `a = state[i]; b = state[i+4]; a = a.wrapping_add(b).wrapping_add(rc[r][i]); b = (b ^ a).rotate_left(ROT[i]); state[i]=a; state[i+4]=b;`
   3. Permute words: `[s0,s5,s2,s7,s4,s1,s6,s3]`
   4. Diffuse: `state[i] ^= state[(i+3)%8]` for i=0..7

Pseudo-Rust (reference)
```rust
const ROT: [u32;4] = [32,24,16,63];
fn vinf_p(state: &mut [u64;8], rc: &[[u64;8]; R_P]){
  for r in 0..R_P{
    for i in 0..8{ state[i] = state[i].wrapping_add(rc[r][i]); }
    for i in 0..4{
      let mut a = state[i];
      let mut b = state[i+4];
      a = a.wrapping_add(b).wrapping_add(rc[r][i]);
      b = (b ^ a).rotate_left(ROT[i]);
      state[i] = a; state[i+4] = b;
    }
    let s = [state[0], state[5], state[2], state[7], state[4], state[1], state[6], state[3]];
    state.copy_from_slice(&s);
    for i in 0..8{ state[i] ^= state[(i+3)%8]; }
  }
}
```

Sponge building blocks (procedural)
- Rate: 32 bytes. Always operate on full `rate_bytes` blocks when absorbing.
- Padding: when final block shorter than `rate_bytes`, append `0x01` then zeros up to `rate_bytes`.
- Domain separation: prefix absorption input with a single domain byte.

Absorb procedure (pseudo)
1. `state = [0u64;8]` (caller may reuse state across multiple Absorb calls if desired).
2. Let `stream = domain_byte || data`.
3. For each `block` of `rate_bytes` from `stream`:
   - XOR `block` into little-endian bytes of words 0..3.
   - Call `vinf_p(state, rc)`.

Squeeze procedure (pseudo)
1. While `out.len < wanted`:
   - Append little-endian bytes of words 0..3 (32 bytes) to `out`.
   - If `out.len >= wanted`, truncate and return.
   - Otherwise, call `vinf_p(state, rc)` and repeat.

Domain bytes
- `DOMAIN_NODE = 0x4E` // 'N' for per-node
- `DOMAIN_MIX  = 0x4D` // 'M' for mixing inputs
- `DOMAIN_FINAL= 0x46` // 'F' for final XOF

Per-node digest `h_node(data)` (exact steps)
1. `state = [0u64;8]`
2. `Absorb(state, DOMAIN_NODE, encode_u64_le(data.len()) || data)`
3. `out = Squeeze(state, NODE_DIGEST_BYTES)` return `out` (32 bytes)

Mixing rounds (how to compute v.hash[r+1])
1. For each round `r` from 0..R-1:
   - For each node `v` (can run in parallel):
     a. Build neighbor list `N(v)`, sorted by `(edge_type: u8, neighbor_id: u64_le)`.
     b. Let `concat_n` = concat of `h_u[r]` for u in N(v).
     c. Let `mix_input = encode_u64_le(concat_n.len()) || concat_n`.
     d. Let `full = DOMAIN_MIX || v.hash[r] || mix_input || encode_u64_le(r)`.
     e. Compute `v.hash[r+1] = h_node(full)`.

Top-level finalization (streaming XOF)
1. After final round, collect `final_hashes` = `[v.hash[R] for v in nodes]` sorted by node id.
2. `state = [0u64;8]`
3. `Absorb(state, DOMAIN_FINAL, encode_meta(graph_meta))`.
4. For each `h` in `final_hashes`: `Absorb(state, 0x00, h)` (0x00 here denotes no extra domain between hashed chunks; you may also call Absorb directly with h if you keep the domain only on the first call).
5. `Squeeze(state, out_len)` and stream to `out_writer`.

Canonical serialization rules (implement exactly)
- `u64` fields are encoded as 8-byte little-endian.
- `len-prefixed blob` = `encode_u64_le(len)` || `bytes`.
- Node input for `h_node`: `id(u64)` || `meta_len(u64)` || `meta_bytes` || `content_len(u64)` || `content_bytes`.
- Edge ordering for neighbor list: sort by `(edge_type: u8, neighbor_id: u64_le)`.

Rust-friendly types (suggested)
```rust
pub const NODE_DIGEST_BYTES: usize = 32;
pub struct Node { pub id: u64, pub metadata: Vec<u8>, pub content_len: u64, pub hash: Vec<[u8; NODE_DIGEST_BYTES]> }
pub enum EdgeType { Adjacent = 0, Header = 1, Semantic = 2 }
pub struct Edge { pub ty: u8, pub to: u64 }
pub struct Graph { pub nodes: Vec<Node>, pub edges: BTreeMap<u64, Vec<Edge>>, pub meta: Vec<u8> }
```

End-to-end pseudocode (concrete)
```text
function build_vinf_hash(graph, rounds=8, out_len=L, out_writer):
  // initialize per-node digest
  for node in graph.nodes:
    // read content (streaming) and compute initial digest
    node.hash[0] = h_node(serialize(node.id) || encode_len(node.metadata.len()) || node.metadata || encode_len(node.content_len) || read_chunk_content(node))

  // mixing rounds
  for r in 0..rounds-1:
    next_map = {}
    for node in graph.nodes in parallel:
      neighbors = get_sorted_neighbors(graph, node.id)
      concat_n = concat([neighbor.hash[r] for neighbor in neighbors])
      full = DOMAIN_MIX || node.hash[r] || encode_u64_le(concat_n.len()) || concat_n || encode_u64_le(r)
      next_map[node.id] = h_node(full)
    for node in graph.nodes:
      node.hash[r+1] = next_map[node.id]

  // finalize
  final_hashes = [node.hash[rounds] for node in graph.nodes sorted by id]
  vinf_xof_finalize(graph.meta, final_hashes.iterator(), out_len, out_writer)

  return success
```

Testing (practical)
- Create a reference `examples/hash_ref.rs` that:
  1. Builds a 1-node graph with metadata `b"text/plain"` and content `b"abc"`.
  2. Calls `build_vinf_hash(..., rounds=1, out_len=64)` and prints hex output.
  3. Commit that hex as the canonical vector. Repeat for empty graph and 2-node test.

Performance notes
- Stream content when computing `h_node`.
- Stream final XOF output to `Write` to avoid buffering large `L` in memory.
- Use `BTreeMap`/`Vec` with sorting for deterministic ordering — avoid `HashMap` iteration order.
