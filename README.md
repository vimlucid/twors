# TwoRS

![development](https://img.shields.io/badge/maintenance%20status-actively%20developed-brightgreen)
![crates.io](https://img.shields.io/crates/v/twors)

This project aims to be a simple and to-the-point quick way to render some
graphics using Rust! :crab:

I already have some features which I'll polish and move to this project soon.
Here's a list of what you can expect in the upcoming months as first baby steps:

- [ ] Component system (with `init()` and `update()` lifecycle methods)
- [ ] Keyboard & mouse input
- [ ] Delta time
- [ ] Utilities for animating movement
- [ ] Component building blocks
    - [ ] Transform for moving & scaling objects (with support for parenting)
    - [ ] Basic 2D rectangle-based collision
    - [ ] A "renderable" model built manually from vertices & some primitive styling

## Quick start

Install dependencies
```bash
# convenience scripts - see "Makefile.toml" for full list of commands
cargo install cargo-make

# build WASM module and start a HTTP server to serve it
cargo make serve
```

## Development

```bash
cargo make watch
```

## Contributing

- Make sure to run `cargo make licenses` after introducing a new dependency
