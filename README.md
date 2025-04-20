# TwoRS

![development](https://img.shields.io/badge/maintenance%20status-actively%20developed-brightgreen)
![crates.io](https://img.shields.io/crates/v/twors)

This project aims to be a simple and to-the-point quick way to render some
graphics using Rust! :crab:

## What's already implemented

At the moment - nothing. Keep checking every week or two for rapid increments.

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

## :zap: Quick start

Coming soon.

- In the meantime you can take a look at `examples/playground` - this is how you
  would use this crate. In a summary you will need:
    - a `lib` crate with the following setup in `Cargo.toml`:
        ```
        [lib]
        crate-type = ["cdylib"]
        ```
    - Build the WASM module: `wasm-pack build --target web`
    - Serve the resulting `pkg` folder along with an `index.html` (see `examples/playground/assets/index.html`)
        - **NOTE**: Make sure to serve WASM with a mime type `application/wasm` - [miniserve](https://github.com/svenstaro/miniserve)
          does this out of the box!
- To do all of this in a single command for the `playground` example simply run `cargo make serve` and open `http://localhost:8080`

## Development

### Scripts

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
