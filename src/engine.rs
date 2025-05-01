mod animation_frame;
mod renderer;

pub mod component;
pub mod input;

use crate::{Canvas, Vertex2, error::Result};
use component::Component;
use input::Input;
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};
use web_sys::{CanvasRenderingContext2d, Window};
use web_time::{Duration, SystemTime};

pub struct EngineContext<'a> {
    pub render_ctx: &'a CanvasRenderingContext2d,
    pub delta_time: f32,
    pub input: &'a Input,
}

struct EngineState {
    canvas: Canvas,
    components: RefCell<Vec<Component>>,
    input: RefCell<Input>,
    last_time: RefCell<SystemTime>,
}

// TODO: Two Rcs? What's the criteria for what goes into the engine state and what out of it?
pub struct Engine {
    window: Rc<Window>,
    state: Rc<EngineState>,
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

        let state = EngineState {
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

        animation_frame::request_recursive(
            self.window.clone(),
            Rc::new(move || {
                let elapsed = state
                    .last_time
                    .borrow()
                    .elapsed()
                    .unwrap_or_else(|_| Duration::default());
                *state.last_time.borrow_mut() = SystemTime::now();

                state.canvas.clear();

                // TODO: Resize on resize event to avoid WASM boundary crossing on the hot path
                let window_size = get_window_inner_size(&window)?;
                state.canvas.resize(window_size);

                let mut input = state.input.borrow_mut();

                let ctx = {
                    let delta_time = elapsed.as_millis() as f32 / 1000.0;
                    EngineContext {
                        render_ctx: state.canvas.context(),
                        delta_time,
                        input: &input,
                    }
                };

                Engine::handle_components(state.components.borrow_mut(), &ctx);

                input.transition_states();

                Ok(())
            }),
        )
    }

    fn handle_components(mut components: RefMut<Vec<Component>>, ctx: &EngineContext) {
        {
            let mut components = components.iter_mut().collect::<Vec<&mut Component>>();
            Engine::update_components(components.as_mut_slice(), ctx);
        }

        {
            let components = components.iter().collect::<Vec<&Component>>();
            Engine::render_components(components.as_slice(), ctx.render_ctx);
        }
    }

    fn update_components(components: &mut [&mut Component], ctx: &EngineContext) {
        for component in components {
            let logic = component.logic.as_mut();
            let transform = &mut component.transform;
            logic.on_update(ctx, transform);
        }
    }

    fn render_components(components: &[&Component], ctx: &CanvasRenderingContext2d) {
        for component in components {
            for renderable in &component.renderables {
                renderer::render(ctx, &renderable.vertices, &component.transform);
                (renderable.style)(ctx);
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
