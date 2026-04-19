# Merkle Tree Collection Reader

```bash
mkdir -p /tmp/merkle-tree-collection/958
```

```bash
curl -Lo /tmp/merkle-tree-collection/958/958_merkle_tree_collection.json \
https://storage.googleapis.com/jito-mainnet/958/ny-mainnet-tip-router-1/958-merkle-tree-collection.json
```

```bash
cargo build --release

hyperfine --warmup 3 --min-runs 20 --export-markdown bench.md \
  -n serde_json       './target/release/merkle-tree-collection-reader --save-path /tmp/merkle-tree-collection/958 --epoch 958 serde-json' \
  -n serde_json_slice './target/release/merkle-tree-collection-reader --save-path /tmp/merkle-tree-collection/958 --epoch 958 serde-json-slice'
  
  
# One-time conversion: JSON → .wincode
./target/release/merkle-tree-collection-reader \
  --save-path /tmp/merkle-tree-collection/958 --epoch 958 wincode-convert

# Bench alongside bincode:
hyperfine --warmup 3 --min-runs 20 --export-markdown bench.md \
  -n bincode './target/release/merkle-tree-collection-reader --save-path /tmp/merkle-tree-collection/958 --epoch 958 bincode' \
  -n wincode './target/release/merkle-tree-collection-reader --save-path /tmp/merkle-tree-collection/958 --epoch 958 wincode'
```

### Serde JSON

```bash
Benchmark 1: ./target/release/merkle-tree-collection-reader --save-path /tmp/merkle-tree-collection/958 --epoch 958 serde-json
  Time (mean ± σ):     27.624 s ±  0.269 s    [User: 26.352 s, System: 1.262 s]
  Range (min … max):   27.415 s … 28.339 s    10 runs
```
