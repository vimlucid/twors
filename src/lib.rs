//! # TwoRS
//! Easily render 2D graphics in a canvas - entirely powered by Rust!!!
//!
//! ## Setup
//! Refer to [the github page](https://github.com/vimlucid/twors) for a guide on how to setup
//! "twors" in your project.
//! - Don't copy-paste from these examples until you have a working setup first!
//!
//! ## Hello world
//! Use `engine::run` to start the main loop
//! - the callback will be invoked multiple times per second.
//! - the canvas will be automatically cleared at the beginning of each frame
//! - use the `ctx` variable to handle inputs and to render graphics
//! ```rust
//! pub fn entry(canvas_id: &str) -> Result<()> {
//!     console_log::init().unwrap(); // Setup a logger capable of logging in the browser
//!
//!     engine::run(
//!         canvas_id,
//!         Rc::new(move |ctx| {
//!             // Handle input
//!
//!             // Render
//!
//!             Ok(())
//!         }),
//!     )?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Rendering
//!
//! `ctx.render_ctx` has the Rust bindings for the canvas API
//! ```rust
//! let ctx = ctx.render_ctx;
//! let pos = Vertex2 { x: 100.0, y: 200.0 };
//! let size = Vertex2 { x: 40.0, y: 40.0 };
//!
//! ctx.set_fill_style_str("red");
//! ctx.set_line_width(1.0);
//! ctx.set_stroke_style_str("black");
//!
//! ctx.begin_path();
//! ctx.move_to(pos.x, pos.y);
//! ctx.line_to(pos.x + size.x, pos.y);
//! ctx.line_to(pos.x + size.x, pos.y + size.y);
//! ctx.line_to(pos.x, pos.y + size.y);
//! ctx.line_to(pos.x, pos.y);
//!
//! ctx.stroke();
//! ctx.fill();
//! ```
//!
//! ## Inputs
//!
//! There are two properties on `ctx.input` - `mouse` and `keyboard` - both have similar API.
//!
//! Use the `is_pressed`, `is_released`, `is_down` and `is_up` methods in combination with
//! the `Mouse` and `Key` enums to check for a key's state.
//!
//! ```rust
//! ctx.input.mouse.is_pressed(Mouse::Main)
//! ctx.input.keyboard.is_pressed(Key::A)
//! ```

mod canvas;
mod engine;
mod error;
mod vertex2;
mod wasm_assert;

pub mod shape_factory;

pub use canvas::Canvas;
pub use engine::{
    Engine, EngineContext,
    component::{Component, Logic, Renderable, Transform},
    input::{Key, Mouse},
};
pub use error::{Error, Result};
pub use vertex2::Vertex2;
