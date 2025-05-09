mod animation_frame;
mod canvas;
mod renderer;

pub mod component;
pub mod input;

use crate::{Layer, Transform, Vertex2, engine::canvas::Canvas, error::Result};
use component::Component;
use input::Input;
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};
use web_sys::{CanvasRenderingContext2d, Window};
use web_time::{Duration, SystemTime}; // std::time::SystemTime panics in WASM

/// The context is passed to the `update` callback of [Component](Component)
///
/// It's use is to:
/// - read mouse/keyboard inputs
/// - provide the delta time value
pub struct Context<'a> {
    pub input: &'a Input,
    pub absolute_parent: Transform,

    delta_time: f32,
}

impl<'a> Context<'a> {
    pub fn new(input: &'a Input, absolute_parent: Transform, delta_time: f32) -> Self {
        Self {
            input,
            absolute_parent,
            delta_time,
        }
    }

    /// This is the number of seconds that passed since the last frame in the main loop.
    ///
    /// It's a very small value (e.g. if the application runs at `60` FPS then the delta time will
    /// be `1/60` = `0.016666666666666666`. If it runs at `30` FPS then delta time will be `1/30`.
    /// The higher the FPS - the lower the delta time value.
    ///
    /// Any movement over time should be multiplied by the delta time - this way the speed will be
    /// the same across various hardware even if the FPS differs due to hardware capabilities.
    ///
    /// ## Example
    ///
    /// Let's say player has to move with a speed of `50`.
    /// The calculation for the movement  `50 * delta_time`.
    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }
}

/// This is a separate `State` struct as opposed to flattening its fields in the `Engine` struct
/// because the `State` data crosses the WASM/JS boundary, so it needs to be memory managed from
/// behind an `Rc`. It's a lot more ergonomic to use the `Rc` once on the entire shared state as
/// opposed to repeating it for each field.
struct State {
    canvas: Canvas,
    components: RefCell<Vec<Box<dyn Component>>>,
    input: RefCell<Input>,
    last_time: RefCell<SystemTime>,
}

/// This is the entry point of our application - initialize a logger, pass a collection of
/// components and run the engine.
/// - See [Component](Component) for examples on a component definition.
///
/// ```rust
/// use std::collections::HashMap;
/// use twors::{Engine, Result};
/// use wasm_bindgen::prelude::wasm_bindgen;
///
/// // Make sure to check the "Installation and build" guide on how to call the "entry" function
/// // from JS
/// #[wasm_bindgen]
/// pub fn entry(canvas_id: &str) -> Result<()> {
///     console_log::init().unwrap(); // Setup logging for the browser console
///
///     let components = Vec::default(); // Add components here
///     let engine = Engine::new(canvas_id, components)?;
///     engine.run()?;
///
///     Ok(())
/// }
/// ```
pub struct Engine {
    window: Rc<Window>,
    state: Rc<State>,
}

impl Engine {
    pub fn new(canvas_id: &str, components: Vec<Box<dyn Component>>) -> Result<Self> {
        let not_found_msg = |entity: &str| format!("Did not find '{}'", entity);
        let window = web_sys::window().ok_or_else(|| not_found_msg("window"))?;

        let canvas = {
            let document = window.document().ok_or_else(|| not_found_msg("document"))?;
            let canvas = document
                .get_element_by_id(canvas_id)
                .ok_or_else(|| not_found_msg(canvas_id))?;
            Canvas::new(canvas)?
        };

        let input = Input::default();
        input.init(&window)?;

        let state = State {
            canvas,
            components: RefCell::new(components),
            input: RefCell::new(input),
            last_time: RefCell::new(SystemTime::now()),
        };

        Ok(Self {
            state: Rc::new(state),
            window: Rc::new(window),
        })
    }

    pub fn run(&self) -> Result<()> {
        let window = self.window.clone();
        let state = self.state.clone();
        animation_frame::request_recursive(
            self.window.clone(),
            Rc::new(move || Engine::main_loop(state.clone(), &window)),
        )
    }

    fn main_loop(state: Rc<State>, window: &Window) -> Result<()> {
        // TODO: Resize on resize event to avoid WASM boundary crossing on the hot path
        let window_size = get_window_inner_size(window)?;
        state.canvas.resize(window_size);
        state.canvas.clear();

        // Scope the immutable input borrow to avoid crashing on the
        // mutable borrow afterwards.
        {
            let delta_time = Engine::calc_delta_and_update_last(state.last_time.borrow_mut());
            let input_borrow = state.input.borrow();
            let mut ctx = Context::new(&input_borrow, Transform::default(), delta_time);

            let mut components = state.components.borrow_mut();
            let mut components: Vec<&mut dyn Component> =
                components.iter_mut().map(|cmp| cmp.as_mut() as _).collect();
            Engine::update_components(components.as_mut_slice(), &mut ctx);
        }

        let components = state.components.borrow();
        let components: Vec<&dyn Component> =
            components.iter().map(|cmp| cmp.as_ref() as _).collect();
        Engine::render_layers(components.as_slice(), state.canvas.context());

        state.input.borrow_mut().transition_states();

        Ok(())
    }

    fn calc_delta_and_update_last(mut last_time: RefMut<SystemTime>) -> f32 {
        let elapsed = last_time.elapsed().unwrap_or_else(|_| Duration::default());
        let delta_time = elapsed.as_millis() as f32 / 1000.0;

        *last_time = SystemTime::now();

        delta_time
    }

    fn update_components(components: &mut [&mut dyn Component], ctx: &mut Context) {
        let build_ctx = |component: &dyn Component| {
            let absolute_parent = component.transform().clone() + &ctx.absolute_parent;
            Context::new(ctx.input, absolute_parent, ctx.delta_time())
        };

        // Children must be updated first so that parent components can have the final say in the
        // children's state (since the parents are responsible for the management).
        // Otherwise a child's state in the current frame can get modified by the parent state in
        // the next frame and we would get jittery movement.
        for component in components.iter_mut() {
            let mut ctx = build_ctx(*component);
            let mut children = component.children_mut();
            Engine::update_components(children.as_mut_slice(), &mut ctx);
        }

        for component in components.iter_mut() {
            let mut ctx = build_ctx(*component);
            component.update(&mut ctx);
        }
    }

    fn render_layers(components: &[&dyn Component], render_ctx: &CanvasRenderingContext2d) {
        Engine::render_components(components, render_ctx, Layer::Five, &Transform::default());
        Engine::render_components(components, render_ctx, Layer::Four, &Transform::default());
        Engine::render_components(components, render_ctx, Layer::Three, &Transform::default());
        Engine::render_components(components, render_ctx, Layer::Two, &Transform::default());
        Engine::render_components(components, render_ctx, Layer::One, &Transform::default());
    }

    fn render_components(
        components: &[&dyn Component],
        render_ctx: &CanvasRenderingContext2d,
        layer: Layer,
        parent_transform: &Transform,
    ) {
        for component in components.iter() {
            let layer_filtered_renderables = component
                .renderables()
                .iter()
                .filter(|renderable| renderable.layer == layer);

            let renderable_parent_transform = parent_transform.clone() + component.transform();
            for renderable in layer_filtered_renderables {
                let transform = renderable_parent_transform.clone() + &renderable.transform;
                renderer::render(render_ctx, &renderable.vertices, &transform);
                (renderable.style)(render_ctx);
            }
        }

        for component in components.iter() {
            let children = component.children();
            let children: Vec<&dyn Component> = children.iter().map(|child| &**child).collect();
            let parent_transform = parent_transform.clone() + component.transform();
            Engine::render_components(&children, render_ctx, layer, &parent_transform);
        }
    }
}

fn get_window_inner_size(window: &Window) -> Result<Vertex2<u32>> {
    let width = window
        .inner_width()
        .map_err(|_| "Failed to get window's inner width")?
        .as_f64()
        .ok_or("Failed to convert window's inner width to f64")? as u32;

    let height = window
        .inner_height()
        .map_err(|_| "Failed to get window's inner height")?
        .as_f64()
        .ok_or("Failed to convert window's inner width to f64")? as u32;

    Ok(Vertex2 {
        x: width,
        y: height,
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        // TODO: Write tests
    }
}
