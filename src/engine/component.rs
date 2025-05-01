use super::Context;
use crate::Vertex2;
use web_sys::CanvasRenderingContext2d;

const DEFAULT_SCALE: Vertex2<f32> = const { Vertex2::new(1.0, 1.0) };

/// Directly modify the `position` and `scale` fields - the engine will automatically pick up the
/// changes and move your `Renderable`s
#[derive(Debug)]
pub struct Transform {
    pub position: Vertex2<f32>,
    pub scale: Vertex2<f32>,
}

impl Transform {
    pub fn from_position(position: Vertex2<f32>) -> Self {
        Self {
            position,
            scale: DEFAULT_SCALE,
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vertex2::default(),
            scale: DEFAULT_SCALE,
        }
    }
}

/// The `Renderable` describes something that can be visualized.
///
/// The `vertices` field describes the points that will make a shape.
/// - Specify at least three vertices or an `assert` will crash the application.
/// - Use the `shape_factory` for convenience if it supports the shape that you need.
///
/// The `style` field is a function pointer with a 2D canvas context as an argument - simply
/// use the canvas API to style the vertices that were passed to the `vertices` field.
///
/// # Example
/// ```rust
/// use twors::{prelude::*, shape_factory};
///
/// pub fn new() -> Component {
///     Component {
///         transform: Transform::from_position(Vertex2::new(300.0, 200.0)),
///         renderables: vec![Renderable {
///             vertices: shape_factory::square(30.0),
///             style: |ctx: &CanvasRenderingContext2d| {
///                 ctx.set_fill_style_str("red");
///                 ctx.set_line_width(1.0);
///                 ctx.set_stroke_style_str("black");
///                 ctx.stroke();
///                 ctx.fill();
///             },
///         }],
///         logic: Box::new(ComponentLogic::default()),
///     }
/// }
///
/// #[derive(Default)]
/// struct ComponentLogic;
/// impl Logic for ComponentLogic {
///     fn on_init(&mut self, _: &mut Context, _: &mut Transform) {}
///     fn on_update(&mut self, _: &mut Context, _: &mut Transform) {}
/// }
/// ```
pub struct Renderable {
    pub vertices: Vec<Vertex2<f32>>,
    pub style: fn(&CanvasRenderingContext2d),
}

pub trait Logic {
    fn on_init(&mut self, ctx: &mut Context, transform: &mut Transform);
    fn on_update(&mut self, ctx: &mut Context, transform: &mut Transform);
}

pub struct Component {
    pub transform: Transform,
    pub renderables: Vec<Renderable>,
    pub logic: Box<dyn Logic>,
}
