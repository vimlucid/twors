# :zap: Quick start

1. Create a new library crate

```bash
cargo new --lib twors_demo
```

2. Add `twors` as a dependency, along with `wasm-bindgen` and `web-sys` in `Cargo.toml`

```toml
[dependencies]
wasm-bindgen = "*"
web-sys = "*"

[dependencies.twors]
git = "https://github.com/vimlucid/twors"
```

3. Write some rendering code in `lib.rs`

```rust
use std::{cell::RefCell, rc::Rc};
use twors::{Result, Vertex2, main_loop};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
pub fn entry(canvas_id: &str) -> Result<()> {
    let player_pos = Rc::new(RefCell::new(Vertex2 { x: 0.0, y: 0.0 }));
    let player_pos = player_pos.clone();

    main_loop::run(
        canvas_id,
        Rc::new(move |ctx| {
            let mut player_pos = player_pos.borrow_mut();
            player_pos.x += 2.0;
            player_pos.y += 1.0;

            draw_square(ctx, &player_pos, &Vertex2 { x: 40.0, y: 40.0 });

            Ok(())
        }),
    )?;

    Ok(())
}

fn draw_square(ctx: &CanvasRenderingContext2d, pos: &Vertex2<f64>, size: &Vertex2<f64>) {
    ctx.set_fill_style_str("red");
    ctx.set_line_width(1.0);
    ctx.set_stroke_style_str("black");

    ctx.begin_path();
    ctx.move_to(pos.x, pos.y);
    ctx.line_to(pos.x + size.x, pos.y);
    ctx.line_to(pos.x + size.x, pos.y + size.y);
    ctx.line_to(pos.x, pos.y + size.y);
    ctx.line_to(pos.x, pos.y);

    ctx.stroke();
    ctx.fill();
}
```

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

6. Create a suitable `index.html` to call our WASM code and put it in the `pkg` folder

```html
<!DOCTYPE html>
<html lang="en-US">
    <head>
        <meta charset="utf-8" />
        <title>TwoRS demo</title>

        <style>
            * {
                margin: 0;
            }

            html, body {
                height: 100%;
            }

            #render-area {
                display: block;
                width: 100%;
                height: 100%;
            }
        </style>
    </head>

    <body>
        <script type="module">
            import init, { entry } from "./twors_demo.js";
            init().then(() => {
                entry("render-area");
            });
        </script>

        <canvas id="render-area" width="1920" height="1080"></canvas>
    </body>
</html>
```

7. Serve with a HTTP server supporting `application/wasm` MIME types for WASM files. [miniserve](https://github.com/svenstaro/miniserve) is a great choice:

```bash
cargo intall miniserve
miniserve ./pkg --index index.html
```

8. Enjoy a moving square at `http://localhost:8080`
