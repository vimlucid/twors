pub mod transform;

use super::Context;
use crate::Vertex2;
use transform::Transform;
use web_sys::CanvasRenderingContext2d;

/// Defines the priority of the [Renderable](Renderable) - lower layers have higher priority (e.g.
/// a `Layer::One` [Renderable](Renderable) will always appear on top of a `Layer::Two`
/// [Renderable](Renderable)
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Layer {
    One,
    Two,
    Three,
    Four,
    Five,
}

/// The `Renderable` is a description of something that can be rendered.
/// Multiple renderables can be added to a single [Component](Component)
///
/// Make sure to use the `shape_factory` where possible to avoid specifying lists of vertices
/// manually.
///
/// ## Example
///
/// ```rust
/// use twors::{prelude::*, shape_factory};
///
/// pub fn new() -> Renderable {
///     Renderable {
///         transform: Transform::default(),
///         vertices: shape_factory::square(40.0),
///         style: |ctx: &CanvasRenderingContext2d| {
///             ctx.set_fill_style_str("orange");
///             ctx.set_line_width(1.0);
///             ctx.set_stroke_style_str("black");
///             ctx.stroke();
///             ctx.fill();
///         },
///         layer: twors::Layer::One,
///     }
/// }
/// ```
#[derive(Debug)]
pub struct Renderable {
    pub transform: Transform,

    /// The engine will automatically create a shape from the vertices by using the
    /// canvas' `move_to` and `line_to` methods until the shape is complete. It will
    /// **THEN** call the `style` callback.
    pub vertices: Vec<Vertex2<f32>>,

    /// This callback is passed bindings for the canvas API - use it to style the `vertices`.
    pub style: fn(&CanvasRenderingContext2d),

    /// Rendering priority.
    pub layer: Layer,
}

#[doc(hidden)]
pub trait ComponentGetter {
    fn transform(&self) -> &Transform;
    fn transform_mut(&mut self) -> &mut Transform;

    fn renderables(&self) -> &[Renderable];

    fn children(&self) -> Vec<&dyn Component>;
    fn children_mut(&mut self) -> Vec<&mut dyn Component>;
}

/// The `Component` is the bread and butter of our application - see the methods' documentation
/// and the example below for details.
///
/// ## Example
///
/// ```rust
/// use twors::prelude::*;
///
/// #[derive(Component)]
/// pub struct MyComponent {
///     component_state: i32,
///
///     // Those fields are mandatory for every component
///     transform: Transform,
///     renderables: Vec<Renderable>,
/// }
///
/// impl MyComponent {
///     pub fn new() -> Self {
///         Self {
///             component_state: 0,
///
///             transform: Transform::default(),
///             renderables: Vec::default(), // No renderables - the component will not be visible
///         }
///     }
/// }
///
/// impl ComponentLifecycle for MyComponent {
///     fn update(&mut self, ctx: &mut Context) {
///         // Read mouse & keyboard input
///         if ctx.input.mouse.is_pressed(Mouse::LMB) || ctx.input.keyboard.is_down(Key::A) {}
///
///         // Modify component state
///         self.component_state += 1;
///
///         // Use delta_time for movement
///         const SPEED: f32 = 20.0;
///         self.transform.position.x += SPEED * ctx.delta_time();
///     }
/// }
/// ```
pub trait ComponentLifecycle {
    /// This is where the behavior of the component is defined
    /// - use `&mut self` to update the component state and renderables
    /// - use the `&mut Context` value to
    ///     - read the delta time for multiplication of values used for movement over time
    ///     - read input from the mouse/keyboard
    fn update(&mut self, ctx: &mut Context);
}

pub trait Component: ComponentLifecycle + ComponentGetter {}
impl<T: ComponentLifecycle + ComponentGetter> Component for T {}
