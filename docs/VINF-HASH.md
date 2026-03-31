# VINF-HASH

This file provides a concise overview of VINF's custom graph-based hashing.  
The complete, authoritative specification is in [VINF-HASH-CUSTOM.md](VINF-HASH-CUSTOM.md).

Key defaults
- Chunk size: `C = 4096` bytes
- Mixing rounds: `R = 8`
- Per-node digest: `NODE_DIGEST_BYTES = 32`
- Default final output length: `L = 10 MiB`
- Algorithm ID: `VINF-HASH-V1`

Notes
- This project uses only the custom VINF primitives defined in the full spec; no external hash or XOF primitives are required.
- For implementation details, canonical serialization, pseudocode, test vectors and security notes, see [VINF-HASH-CUSTOM.md](VINF-HASH-CUSTOM.md).
