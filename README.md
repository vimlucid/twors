# TwoRS

![development](https://img.shields.io/badge/maintenance%20status-actively%20developed-brightgreen)
![crates.io](https://img.shields.io/crates/v/twors)

Easily render 2D graphics in a canvas - entirely powered by Rust!!! :crab:

## :zap: Quick start

- Here's how to [quickly render a moving rectangle](./docs/quick_start.md)

## What to expect

At the moment - just a scaffold project with a main loop and a canvas context for drawing.

Roadmap for the next few months (keep checking every week or two for updates):
- [ ] Delta time & adaptive canvas resolution based on browser window size.
- [ ] Keyboard & mouse input.
- [ ] Component system (with `init()` and `update()` lifecycle methods)
- [ ] Component building blocks
    - [ ] Transform for moving & scaling objects (with support for parenting)
    - [ ] Basic 2D rectangle-based collision
    - [ ] A "renderable" model built manually from vertices & some primitive styling
- [ ] Utilities for animating movement over time, drawing curves, etc...

## Development

Building WASM packages via `wasm-pack` requires a special library crate.
Since `twors` is itself a library (and due to the Rust toolchain limitation of having
only a single `lib` crate per package) we can't use the standard `main.rs` binary crate 
for testing - you will need to make use of the `playground` crate in the `examples` folder
to iterate/test your changes.

The cargo scripts below make it very easy to do so:

```bash
# convenience scripts - see "Makefile.toml" for full list of commands
cargo install cargo-make

# run local pre-commit checks - will be run on "build" automatically
cargo make install-git-hooks

cargo make build # build the "playground" crate as a WASM module
cargo make serve # like "build", but will also start a HTTP server
cargo make watch # like "serve", but will restart the server on changes

# other commands
cargo make clean
cargo make format
cargo make licenses # update licenses.html (run after dependency addition/removal)
```
