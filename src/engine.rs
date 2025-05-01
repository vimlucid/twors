mod animation_frame;
mod renderer;

pub mod component;
pub mod input;

use crate::{Canvas, Vertex2, error::Result};
use component::Component;
use input::Input;
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};
use web_sys::{CanvasRenderingContext2d, Window};
use web_time::{Duration, SystemTime};

pub struct Context<'a> {
    pub delta_time: f32,
    pub input: Ref<'a, Input>,

    canvas: &'a Canvas,
}

impl<'a> Context<'a> {
    pub fn new(canvas: &'a Canvas, input: Ref<'a, Input>, delta_time: f32) -> Self {
        Self {
            canvas,
            input,
            delta_time,
        }
    }

    pub fn render_ctx(&self) -> &CanvasRenderingContext2d {
        self.canvas.context()
    }
}

struct State {
    canvas: Canvas,
    components: RefCell<Vec<Component>>,
    input: RefCell<Input>,
    last_time: RefCell<SystemTime>,
}

// TODO: Two Rcs? What's the criteria for what goes into the engine state and what out of it?
pub struct Engine {
    window: Rc<Window>,
    state: Rc<State>,
}

impl Engine {
    pub fn new(canvas_id: &str, components: Vec<Component>) -> Result<Self> {
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
        let state = self.state.clone();
        let window = self.window.clone();

        // Scope the immutable input borrow to avoid crashing on the
        // mutable borrow afterwards.
        {
            let ctx = Context::new(&state.canvas, state.input.borrow(), 0.0);
            Engine::init_components(state.components.borrow_mut(), &ctx);
        }

        animation_frame::request_recursive(
            self.window.clone(),
            Rc::new(move || {
                // TODO: Resize on resize event to avoid WASM boundary crossing on the hot path
                let window_size = get_window_inner_size(&window)?;
                state.canvas.resize(window_size);
                state.canvas.clear();

                // Scope the immutable input borrow to avoid crashing on the
                // mutable borrow afterwards.
                {
                    let delta_time =
                        Engine::calc_delta_and_update_last(state.last_time.borrow_mut());
                    let ctx = Context::new(&state.canvas, state.input.borrow(), delta_time);
                    Engine::update_components(state.components.borrow_mut(), &ctx);
                }

                state.input.borrow_mut().transition_states();

                Ok(())
            }),
        )
    }

    fn calc_delta_and_update_last(mut last_time: RefMut<SystemTime>) -> f32 {
        let elapsed = last_time.elapsed().unwrap_or_else(|_| Duration::default());
        let delta_time = elapsed.as_millis() as f32 / 1000.0;

        *last_time = SystemTime::now();

        delta_time
    }

    fn init_components(mut components: RefMut<Vec<Component>>, ctx: &Context) {
        let components = components.iter_mut().collect::<Vec<&mut Component>>();
        for component in components {
            let logic = component.logic.as_mut();
            let transform = &mut component.transform;
            logic.on_init(ctx, transform);
        }
    }

    fn update_components(mut components: RefMut<Vec<Component>>, ctx: &Context) {
        // Update
        for component in components.iter_mut() {
            let logic = component.logic.as_mut();
            let transform = &mut component.transform;
            logic.on_update(ctx, transform);
        }

        // Render
        let render_ctx = ctx.render_ctx(); // Cache to avoid crossing WASM boundary unnecessarily.
        for component in components.iter() {
            for renderable in &component.renderables {
                renderer::render(render_ctx, &renderable.vertices, &component.transform);
                (renderable.style)(render_ctx);
            }
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
