# TwoRS

![development](https://img.shields.io/badge/maintenance%20status-actively%20developed-brightgreen)
![crates.io](https://img.shields.io/crates/v/twors)

Easily render 2D graphics in a canvas - entirely powered by Rust!!! &#x1F980;

## &#x1F60E; Why Rust/WASM?

[It's use case](https://webassembly.org/docs/use-cases/) is either the reuse of code written in another language or to offload
heavy computations to the near-native execution speeds of WASM.

However this is not free - traversing the WASM boundary means going trough [some glue code](https://rustwasm.github.io/wasm-bindgen/contributing/design/index.html)
and copying data from JS to the [WASM module memory](https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Concepts)
and back. In the case of strings it's even more expensive - because
[JS strings are UTF-16](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String#utf-16_characters_unicode_code_points_and_grapheme_clusters)
and [Rust strings use UTF-8](https://doc.rust-lang.org/std/string/struct.String.html) any string
passtrough needs to go trough a copy
[AND a re-encode](https://rustwasm.github.io/wasm-bindgen/reference/types/str.html?highlight=utf-16#utf-16-vs-utf-8).

`twors` doesn't have any complicated physics to offload to WASM at this point in time - neither
is it making use of some advanced pre-existing Rust library. So why Rust then?

*Simply because Rust is the best!!!*

## &#x26A1; Quick start

The `playground` crate in this repo contains basic code for a moving rectangle and some input
handling.

- Take a look at the [playground lib.rs](./examples/playground/src/lib.rs) for a sneak peek!

- Experiment by modifying `experiments/playground` and running the code:
  ```bash
  cargo install cargo-make # only necessary once

  # compile, build the WASM module, run a HTTP server - rinse and repeat automatically on code changes
  cargo make watch
  ```

- Set up rendering in your own crate by visiting [the quick start guide](./docs/quick_start.md)

## &#x1F4DD; Roadmap

- [x] Adaptive canvas resolution on resize
- [x] Delta time
- [x] Keyboard and mouse input handling
- [ ] Component system (with `init()` and `update()` lifecycle methods)
- [ ] Transform (with inheritance) - translate, scale
- [ ] Collision detection
- [ ] Bezier curves
- [ ] FPS benchmark

## &#x1F4BB; Development

Experiments and manual testing are to be done in the `examples/playground` crate.

```bash
# convenience scripts - see "Makefile.toml" for full list of commands
cargo install cargo-make

# run local pre-commit checks - will be run on "build" automatically
cargo make install-git-hooks

cargo make build # build the "playground" crate as a WASM module
cargo make serve # like "build", but will also start a HTTP server
cargo make watch # like "serve", but will restart the server on changes

# other commands
cargo make test
cargo make clean
cargo make format
cargo make licenses # update licenses.html (run after dependency addition/removal)
```

### Notes

- Make sure to use  `wasm_assert!` instead of `assert!` in non-test code to see error messages
in the browser console.
