# TwoRS

![crates.io](https://img.shields.io/crates/v/twors)

Easily render 2D graphics in a canvas using WASM - entirely powered by Rust!!! 🦀

## 🙋 Who is this project for?

This tiny renderer is for people who:
- Need an example on how setup WASM & interact with the canvas
- Play around with Rust the fun way - by making a small 2D game

## 😎 Why Rust/WASM?

[WASM's use case](https://webassembly.org/docs/use-cases/) is either the reuse of code written in another language or to offload
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

⭐ *Simply because Rust is the best!!!* ⭐

## ⚡ Quick start

- To quickly play with `twors` you can run the `examples/playground` crate in this repo
  - Move the player via the **WASD keys** (player can't exit the green area)
  - Spawn items at the players' location via the **left mouse button**
  - Drag spawned items via the **right mouse button**

```bash
git clone https://github.com/vimlucid/twors
cd twors

cargo install cargo-make
cargo make watch # this assumes you have the `cp` command - e.g. it won't work in Windows CMD

# edit the source code in examples/playground and manually refresh http://localhost:8080
```

- To add `twors` to your own crate - check out the [Installation and build](#%EF%B8%8F-installation-and-build) guide

## ✨ Features

✅ `TwoRS`'s goal is to be simple. It provides:

- ✅ A main loop with precalculated [delta time](https://en.wikipedia.org/wiki/Delta_timing)
- ✅ Keyboard/mouse
- ✅ Basic component system (not [ECS](https://en.wikipedia.org/wiki/Entity_component_system))

❌ `TwoRS` does not **currently** have (and aim) to provide:

- a physics engine
- an algorithm library (e.g. pathfinding)
- a sound library
- a UI library

## 🛠️ Installation and build

Compiling a WASM library is a bit more involved than simply executing `cargo run`, but
it's pretty straightforward if you know the steps.

We'll basically:
- set up our crate for WASM compilation
- add the bare minimum code to run the `twors` engine
- add an `index.html` with some JavaScript (just enough to run our WASM library)
- serve all of this goodness via an HTTP server

Once we do this **all of the remaining code can be written entirely in Rust!** \
You are of course free to mix and match as you like.

### **Step 1** - Create a `lib` crate

In order to compile Rust into WASM via [wasm-pack](https://github.com/rustwasm/wasm-pack)
(an amazing WASM compilation helper) it's necessary to have a `lib` crate first

```bash
cargo new twors-demo --lib
```

### **Step 2** - Install `twors`, `wasm_bindgen` and `console_log`

- A **browser-capable** logging backend crate like
  [console_log](https://crates.io/crates/console_log) is needed to enable
  logging in the browser console
    - You will also need a logging frontend crate like [log](https://crates.io/crates/log) to
      be able to print messages yourself.
- [wasm_bindgen](https://crates.io/crates/wasm-bindgen) will generate the necessary JS/Rust
  glue to enhance what can go trough the WASM/JS boundary.
  - Without it our Rust code that's
    called from JS (and vice-versa) would be able to only take numbers as arguments and return
    numbers as a result. With this crate we
    [enable a lot more types](https://rustwasm.github.io/wasm-bindgen/reference/types.html)
    in our JS/WASM boundary API.

```bash
cargo add twors
cargo add wasm_bindgen
cargo add console_log
```

### **Step 3** - Create and run the `twors` engine.

- For now we won't add any components, so we'll see a blank canvas when
  we open our application in the browser.

```rust
use twors::{Engine, Result};
use wasm_bindgen::prelude::wasm_bindgen;
use std::collections::HashMap;
use console_log;

// The "wasm_bindgen" attribute will generate glue both on the JS and on the WASM sides.
// Passing Rust types like `&str` from JS is thanks to the magic of `wasm_bindgen`.
#[wasm_bindgen]
pub fn entry(canvas_id: &str) -> Result<()> {
    console_log::init().unwrap(); // Setup logger frontends to use our browser-capable logger.

    // Pass a list of components to render on the canvas.
    // We'll add an index.html file with said canvas later.
    let engine = Engine::new(canvas_id, Vec::default())?;
    engine.run()?;

    Ok(())
}
```

### **Step 4** - Change the crate type to a [dynamic library intended to be loaded from another language](https://doc.rust-lang.org/reference/linkage.html)

Add a `lib` section to your `Cargo.toml` file (if you don't already have one) and set
the `crate-type` property to `cdylib`.

The `wasm32-unknown-unknown` compile target (what `wasm-pack` makes the Rust compiler use) will
detect this and produce a WASM library.

```text
[lib]
crate-type = ["cdylib"]
```

### **Step 5** - Install `wasm-pack` and build your WASM library

The following `wasm-pack --build` command will produce a `pkg` folder in your crate's root -
this folder will contain the WASM library as well as the JS part of the glue code that's
needed for the WASM-JS communication.

```bash
cargo install wasm-pack
wasm-pack build --target web
```

### **Step 6** - Copy `examples/assets/index.html` to the `pkg` folder in your crate's root

We have compiled our WASM library - now we need to call it from JS. The example `index.html`
will:
- create a full screen canvas
- run our Rust `entry` method (that we created earlier) by passing it the canvas ID.

```bash
# navigate to your crate root and execute the following
cp ./examples/assets/index.html ./pkg
```

> **NOTE:** If you don't have the `cp` command (e.g. if you are using Windows CMD) you can always copy
> the `index.html` manually.

### **Step 7** - Serve the `pkg` directory at the crate root

You will need to serve the WASM library with the `application/wasm` MIME type or the browser
will refuse to run it. A great server that does this automatically is
[miniserve](https://github.com/svenstaro/miniserve)

```bash
cargo intall miniserve
miniserve ./pkg --index index.html
```

## 💻 Development

Experiments and manual testing during development can be done in the `examples/playground` crate.

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
