mod animation_frame;
mod canvas;
mod renderer;

pub mod component;
pub mod input;

use crate::{Vertex2, engine::canvas::Canvas, error::Result, wasm_assert};
use component::Component;
use input::Input;
use log::info;
use std::{
    cell::{Ref, RefCell, RefMut},
    collections::{HashMap, HashSet},
    rc::Rc,
};
use web_sys::{CanvasRenderingContext2d, Window};
use web_time::{Duration, SystemTime}; // std::time::SystemTime panics in WASM

/// The context is passed to the `on_init` and `on_update` component callbacks.
///
/// It's use is to:
/// - read mouse/keyboard inputs
/// - add/remove components dynamically
pub struct Context<'a> {
    pub input: Ref<'a, Input>,

    delta_time: f32,

    /// We can't modify the components container while mutably iterating on it, so we store the
    /// component additions from the current frame temporarily - they'll be added on the next frame.
    components_to_add: HashMap<String, Component>,

    /// We can't modify the components container while mutably iterating on it, so we store the
    /// component removals from the current frame temporarily - they'll be added on the next frame.
    components_to_remove: HashSet<String>,
}

impl<'a> Context<'a> {
    pub fn new(input: Ref<'a, Input>, delta_time: f32) -> Self {
        Self {
            input,
            delta_time,
            components_to_add: HashMap::default(),
            components_to_remove: HashSet::default(),
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
    /// # Example
    ///
    /// Let's say player has to move with a speed of `50`.
    /// The calculation for the movement  `50 * delta_time`.
    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }

    /// The component will be added after the current frame ends and before the next one begins.
    pub fn add_component(&mut self, name: String, component: Component) {
        let old_component = self.components_to_add.insert(name, component);
        wasm_assert!(old_component.is_none())
    }

    /// The component will be removed after the current frame ends and before the next one begins.
    pub fn remove_component(&mut self, name: String) {
        info!("Removing {}", name);
        let is_new = self.components_to_remove.insert(name);
        wasm_assert!(is_new)
    }
}

/// This is a separate `State` struct as opposed to flattening its fields in the `Engine` struct
/// because the `State` data crosses the WASM/JS boundary, so it needs to be memory managed from
/// behind an `Rc`. It's a lot more ergonomic to use the `Rc` once on the entire shared state as
/// opposed to repeating it for each field.
struct State {
    canvas: Canvas,
    components: RefCell<HashMap<String, Component>>,
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
///     let mut components = HashMap::default(); // Add custom components to this map
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
    pub fn new(canvas_id: &str, components: HashMap<String, Component>) -> Result<Self> {
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
        self.init_components();

        let window = self.window.clone();
        let state = self.state.clone();
        animation_frame::request_recursive(
            self.window.clone(),
            Rc::new(move || Engine::main_loop(state.clone(), &window)),
        )
    }

    fn init_components(&self) {
        let mut ctx = Context::new(self.state.input.borrow(), 0.0);
        for (_, component) in self.state.components.borrow_mut().iter_mut() {
            let logic = component.logic.as_mut();
            let transform = &mut component.transform;
            logic.on_init(&mut ctx, transform);
        }
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
            let mut ctx = Context::new(state.input.borrow(), delta_time);
            let mut components = state.components.borrow_mut();

            Engine::update_components(&mut components, &mut ctx, state.canvas.context());
            Engine::add_components(&mut components, ctx.components_to_add);
            Engine::remove_components(&mut components, ctx.components_to_remove);
        }

        state.input.borrow_mut().transition_states();

        Ok(())
    }

    fn calc_delta_and_update_last(mut last_time: RefMut<SystemTime>) -> f32 {
        let elapsed = last_time.elapsed().unwrap_or_else(|_| Duration::default());
        let delta_time = elapsed.as_millis() as f32 / 1000.0;

        *last_time = SystemTime::now();

        delta_time
    }

    fn update_components(
        components: &mut HashMap<String, Component>,
        ctx: &mut Context,
        render_ctx: &CanvasRenderingContext2d,
    ) {
        // Update
        for (_, component) in components.iter_mut() {
            let logic = component.logic.as_mut();
            let transform = &mut component.transform;
            logic.on_update(ctx, transform);
        }

        // Render
        for (_, component) in components.iter() {
            for renderable in &component.renderables {
                renderer::render(render_ctx, &renderable.vertices, &component.transform);
                (renderable.style)(render_ctx);
            }
        }
    }

    fn add_components(
        components: &mut HashMap<String, Component>,
        components_to_add: HashMap<String, Component>,
    ) {
        for (name, cmp) in components_to_add {
            let old_cmp = components.insert(name, cmp);
            wasm_assert!(old_cmp.is_none())
        }
    }

    fn remove_components(
        components: &mut HashMap<String, Component>,
        components_to_remove: HashSet<String>,
    ) {
        for name in components_to_remove {
            let removed = components.remove(&name);
            wasm_assert!(removed.is_some());
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
