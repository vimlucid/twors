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
/// The renderables are part of a [Component](Component)
///
/// # Example
/// ```rust
/// use twors::{prelude::*, shape_factory};
///
/// pub fn renderable() -> Renderable {
///     Renderable {
///         vertices: shape_factory::square(30.0),
///         style: |ctx: &CanvasRenderingContext2d| {
///             ctx.set_fill_style_str("red");
///             ctx.set_line_width(1.0);
///             ctx.set_stroke_style_str("black");
///             ctx.stroke();
///             ctx.fill();
///         },
///     }
/// }
/// ```
pub struct Renderable {
    pub vertices: Vec<Vertex2<f32>>,
    pub style: fn(&CanvasRenderingContext2d),
}

/// Implement this on a struct that will be responsible for managing your component's state
/// and behavior.
///
/// You can then put an single instance of this struct into your custom component to define its
/// behavior - see [Component](Component).
///
/// ```rust
/// use twors::prelude::*;
///
/// struct MyComponentLogic {
///     component_state: i32,
/// }
///
/// impl Logic for MyComponentLogic {
///     fn on_init(&mut self, _: &mut Context, _: &mut Transform) {
///         self.component_state = 123;
///     }
///
///     fn on_update(&mut self, _: &mut Context, _: &mut Transform) {
///         self.component_state += 1;
///     }
/// }
/// ```
///
pub trait Logic {
    fn on_init(&mut self, ctx: &mut Context, transform: &mut Transform);
    fn on_update(&mut self, ctx: &mut Context, transform: &mut Transform);
}

/// The core building block of any `TwoRS` application - everything starts with a component.
/// - A component is something that may be visualized  and that has behavior.
/// - Components go into the [Engine](super::Engine) list of components.
///
/// To construct a component the following will be needed:
/// - `transform` - the components initial transform - see [Transform](Transform)
/// - `renderables` - a list of definitions of how the component will look - all of them will be
///   automatically rendered by the engine on each frame - see [Renderable](Renderable).
/// - `logic` - the component's behavior **and state** - see [Logic](Logic).
///
/// ```rust
/// use twors::prelude::*;
///
/// #[derive(Default)]
/// struct MyComponentLogic;
///
/// impl Logic for MyComponentLogic {
///     fn on_init(&mut self, _: &mut Context, _: &mut Transform) {}
///     fn on_update(&mut self, _: &mut Context, _: &mut Transform) {}
/// }
///
/// pub fn new() -> Component {
///     Component {
///         transform: Transform::default(),
///         renderables: vec![],
///         logic: Box::new(MyComponentLogic::default()),
///     }
/// }
/// ```
///
pub struct Component {
    pub transform: Transform,
    pub renderables: Vec<Renderable>,
    pub logic: Box<dyn Logic>,
}
