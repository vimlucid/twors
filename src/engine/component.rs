pub mod transform;

use super::Context;
use crate::Vertex2;
use transform::Transform;
use web_sys::CanvasRenderingContext2d;

/// Defines the priority of the [Renderable](Renderable) - lower layers have higher priority (e.g.
/// a `Layer::One` [Renderable](Renderable) will always appear on top of a `Layer::Two`
/// [Renderable](Renderable)
#[derive(Copy, Clone, Eq, PartialEq)]
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

/// The `Component` is the bread and butter of our application - see the methods' documentation
/// and the example below for details.
///
/// ## Example
///
/// ```rust
/// use twors::prelude::*;
///
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
/// impl Component for MyComponent {
///     fn transform(&self) -> &Transform {
///         &self.transform
///     }
///
///     fn renderables(&self) -> &[Renderable] {
///         &self.renderables
///     }
///
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
///
///     fn children(&self) -> Vec<&dyn Component> {
///         Vec::default()
///     }
///
///     fn children_mut(&mut self) -> Vec<&mut dyn Component> {
///         Vec::default()
///     }
/// }
/// ```
pub trait Component {
    /// the location/scale of the component - define it as a `Transform` field in your
    /// component and return it as a reference from this getter. See [Transform](Transform).
    fn transform(&self) -> &Transform;

    /// - `renderables` - the "visual" part of a component.
    ///     - a component that won't be visible can have 0 renderables
    ///     - define multiple renderables and offset them via their transform to compose a combination
    ///       of shapes
    ///     - define it as a `Vec<Renderable>` field in your component and return it as a slice
    fn renderables(&self) -> &[Renderable];

    /// - `update` - this is where the behavior of the component is defined
    ///     - use `&mut self` to update the component state and renderables
    ///     - use the `&mut Context` value to
    ///         - read the delta time for multiplication of values used for movement over time
    ///         - read input from the mouse/keyboard
    fn update(&mut self, ctx: &mut Context);

    /// - `children` - all child components' references (stored as component fields)
    ///   have to be returned via this method for the engine to render their
    ///   renderables. This is done recursively for the children of the children.
    fn children(&self) -> Vec<&dyn Component>;

    /// - `children_mut` - all child components' references (stored as component fields)
    ///   have to be returned via this method for the engine to call their `update`
    ///   method. This is done recursively for the children of the children.
    fn children_mut(&mut self) -> Vec<&mut dyn Component>;
}
