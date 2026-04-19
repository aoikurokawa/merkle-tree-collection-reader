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
  -n serde_json       './target/release/merkle-tree-collection-reader --save-path /tmp/merkle-tree-collection/958 --epoch 958 serde-json' \
  -n serde_json_slice './target/release/merkle-tree-collection-reader --save-path /tmp/merkle-tree-collection/958 --epoch 958 serde-json-slice' \
  -n bincode './target/release/merkle-tree-collection-reader --save-path /tmp/merkle-tree-collection/958 --epoch 958 bincode' \
  -n wincode './target/release/merkle-tree-collection-reader --save-path /tmp/merkle-tree-collection/958 --epoch 958 wincode'

Benchmark 1: serde_json
  Time (mean ± σ):     27.910 s ±  0.337 s    [User: 26.572 s, System: 1.325 s]
  Range (min … max):   27.502 s … 28.525 s    20 runs

Benchmark 2: serde_json_slice
  Time (mean ± σ):     10.257 s ±  0.184 s    [User: 9.045 s, System: 1.203 s]
  Range (min … max):   10.052 s … 10.571 s    20 runs

Benchmark 3: bincode
  Time (mean ± σ):      1.806 s ±  0.013 s    [User: 1.658 s, System: 0.144 s]
  Range (min … max):    1.792 s …  1.836 s    20 runs

Benchmark 4: wincode
  Time (mean ± σ):     165.3 ms ±   2.1 ms    [User: 52.5 ms, System: 108.7 ms]
  Range (min … max):   162.6 ms … 171.3 ms    20 runs

Summary
  wincode ran
   10.93 ± 0.16 times faster than bincode
   62.04 ± 1.37 times faster than serde_json_slice
  168.82 ± 2.98 times faster than serde_json
```
