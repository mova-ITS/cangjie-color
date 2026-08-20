# cangjie-color

**CangJie letter-code hieroglyph stroke colorizer.**

Turns a curated set of Cangjie recipes (letter → stroke indexes → colours) into
coloured SVGs for teaching aids. Deterministic transform over approved data —
it does **not** invent Cangjie codes or unit splits.

| | |
|---|---|
| **Crate (planned)** | Rust lib + `cj-color` CLI |
| **Lab** | Python prep in `unit-color/` / `datagen/` (local; not all vendored) |
| **Site** | Assets for [cjtut.uk](https://cjtut.uk) tutor; tool credited as generator |
| **License** | Dual: **MIT OR Apache-2.0** |

## Status

Lib + `cj-color` CLI: load samples, Mode views, MMAH→SVG Y flip, single + **batch** write.
Try [examples/zi-full.svg](examples/zi-full.svg) — commands in [examples/README.md](examples/README.md).

Full tutorial packs stay offline/private; only a small sample set ships in-repo.

## Quick links

- Remote: https://github.com/mova-ITS/cangjie-color

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
