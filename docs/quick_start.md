# :zap: Quick start

1. Create a new library crate

```bash
cargo new --lib twors_demo
```

2. Add `twors` as a dependency, along with `wasm-bindgen`, `web-sys` in `Cargo.toml`. You will
most likely also need `log` and `console_log` for logging.

```toml
[dependencies]
wasm-bindgen = "*"
web-sys = "*"
log = "*"
console_log = { version = "*", features = ["color"] }

[dependencies.twors]
git = "https://github.com/vimlucid/twors"
```

3. Copy `examples/playground/src/lib.rs` over to `lib.rs` in your new `twors_demo` crate.

4. Change the crate type in `Cargo.toml` by adding the following:

```
[lib]
crate-type = ["cdylib"]
```

5. Install `wasm-pack` and build (run at the root of your crate) - this should create a `pkg` folder in the crate root with the packaged application

```bash
cargo install wasm-pack
wasm-pack build --target web
```

6. Copy `examples/playground/assets/index.html` over to the newly generated `pkg` folder from step `5`.

7. Serve with a HTTP server supporting `application/wasm` MIME types for WASM files. [miniserve](https://github.com/svenstaro/miniserve) is a great choice:

```bash
cargo intall miniserve
miniserve ./pkg --index index.html
```

8. Enjoy a moving square at `http://localhost:8080`
