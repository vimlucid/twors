# TwoRS

![development](https://img.shields.io/badge/maintenance%20status-actively%20developed-brightgreen)
![crates.io](https://img.shields.io/crates/v/twors)

Simple Rust 2D game engine based on canvas and WASM

This project aims to be a simple and to-the-point quick way to render some
graphics using Rust! :crab:

I already have some basic features which I'll polish and upload soon.
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
cargo install cargo-make # convenience scripts - see "Makefile.toml" for full list of commands
cargo make pkg # build WASM library
cargo make serve # start a HTTP server to serve an example
```

## Contributing

See [contributing docs](docs/contributing.md)
