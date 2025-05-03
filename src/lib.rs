#![doc = include_str!("../README.md")]

mod engine;
mod error;
mod vertex2;
mod wasm_assert;

pub mod dimensions;
pub mod prelude;
pub mod shape_factory;

pub use engine::{
    Context, Engine,
    component::{Component, Layer, Renderable, Transform},
    input::{Key, Mouse},
};
pub use error::{Error, Result};
pub use vertex2::Vertex2;
