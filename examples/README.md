# Examples

## Single: 子 (Full)

```bash
cargo run -- \
  --recipes testdata/recipes/sample.jsonl \
  --graphics testdata/graphics/sample.jsonl \
  --char 子 --mode full -o examples/zi-full.svg
```

Open `zi-full.svg` in Safari/Chrome (Preview.app is unreliable).

## Batch (sample set)

```bash
cargo run -- \
  --recipes testdata/recipes/sample.jsonl \
  --graphics testdata/graphics/sample.jsonl \
  --out-dir examples/batch --mode full
```

Optional subset: `--chars 子,气` or `--chars-file list.txt` (one character per line).
