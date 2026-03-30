# VINF

VINF (Virtual INFinite Space) is an experimental compression system that maps data into a multi-dimensional graph instead of raw bytes.

Instead of storing files directly, VINF stores coordinates, dimensions and constraints that describe where the data exists in a virtual space. This allows us to compress files in terabytes into just a few bytes.

## What is VINF?

Traditional compression reduces file size by eliminating redundancy and encoding data more efficiently.

VINF takes a different approach:
- Treats data as points in a virtual infinite space
- Stores coordinates instead of raw data
- Uses graph traversal for hyper-fast decompression
- Encodes metadata (time, type, hash) as dimensions in the graph

## Core Concepts

### Coordinates

Data is represented as positions in VINF space:
```
vinf(u32:u33 < 12;34;56 < 103)
```
- `12;34;56` are spatial coordinates
- `103` is the header dimension (file type)
- `u32:u33` subspace range

### Dimensions

- **Header Dimension**: file type (PNG, ZIP, EXE, etc.)
- **Time Axis**: Creation/modification time
- **Hash Dimension**: integrity verification
- **Custom Dimensions**: user-defined metadata (tags, categories)

### Compression

Compression works by:
1. Calculating the hash of the file
2. Narrowing search space using:
    - Header Dimension
    - Time Axis
    - Partial hash matching
3. Finding coordinates that represent the data

### Decompression

Decompression is extremely fast:
1. Read coordinates
2. Traverse VINF space
3. Reconstruct data directly

### Hashing

VINF uses graph-based hashing:
- Fast comparisons
- Partial hash matching (X/Y/Z layer matching)
- Used to validate coordinate correctness

### `.vinf` Format

VINF stores:
- Coordinates
- Dimensions
- Hashes

Optional:
- `vcoord` for ultra-compressed, coordinate-only storage (sub-byte size)

### Visualization

VINF data can be visualized as a graph:
- X/Y/Z is spatial position
- Time axis by color and animation
- Header by file type classification

Example: (will update these colors after their implementation)
- PNG chunks are blue
- ZIP chunks are green
- EXE sections are red

## Why VINF?

- **Ultra Compression**: Terabyte files can be compressed to just a few bytes
- **Hyper Fast Decompression**: Graph traversal is much faster than traditional decompression
- **Rich Metadata**: Dimensions allow for powerful metadata storage and querying
- **Deterministic**: Same file will always yield the same coordinates
- **Future Proof**: New dimensions can be added without breaking existing data
- **Visualization**: Unique way to visualize data as a graph

## Status

VINF is experimental and in really really early development.  
This is a research project exploring:
- Graph-based compression
- Multi-dimensional data representation
- Alternative storage paradigms (a paradigm is a way of thinking about how to solve a problem)

## CLI (IDEA)

vinf compress <file> - Compress a file into VINF format
vinf decompress <vinf_file> - Decompress a VINF file back to original
vinf visualize <vinf_file> - Visualize the VINF graph in a terminal
vinf inspect <vinf_file> - Show metadata and dimensions of a VINF file

## GUI

A future GUI could allow users to:
- Drag and drop files for compression
- Browse VINF graphs visually
- Search and filter by dimensions (e.g. find all PNGs created in 2023)
- Export VINF data to other formats

## Inspiration

VINF is inspired by:
- Graph theory
- Multi-dimensional maths
- Procedural data generation
- Compression algorithms
- A random thought while I was chugging Monster.

## License

TBD

## Contributing

See CONTRIBUTING.md (will be added later)

## Contact

- Email: (I'll add this later)
- Or just send me a message through GitHub

## Final Thoughts

> What if data doesn't need to be stored as bytes?
> Only located in a virtual graph space?