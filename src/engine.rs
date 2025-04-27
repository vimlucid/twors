mod animation_frame;
mod input;

pub use input::MouseButton;

use crate::{Canvas, Vertex2, error::Result};
use input::Input;
use std::{cell::RefCell, rc::Rc};
use web_sys::{CanvasRenderingContext2d, Window};
use web_time::{Duration, SystemTime};

pub struct EngineContext<'a> {
    pub render_ctx: &'a CanvasRenderingContext2d,
    pub delta_time: f32,
    pub input: &'a Input,
}

type MainLoopLogic = dyn Fn(&EngineContext) -> Result<()>;

pub fn run(canvas_id: &str, logic: Rc<MainLoopLogic>) -> Result<()> {
    let not_found_msg = |entity: &str| format!("Did not find '{}'", entity);

    let window = web_sys::window().ok_or_else(|| not_found_msg("window"))?;
    let window = Rc::new(window);

    let document = window.document().ok_or_else(|| not_found_msg("document"))?;
    let canvas = document
        .get_element_by_id(canvas_id)
        .ok_or_else(|| not_found_msg(canvas_id))?;
    let canvas = Canvas::new(canvas)?;

    let last_time = Rc::new(RefCell::new(SystemTime::now()));
    let last_time = last_time.clone();

    let input = Input::default();
    input.init(&window)?;
    let input = Rc::new(RefCell::new(input));

    animation_frame::request_recursive(
        window.clone(),
        Rc::new(move || {
            let elapsed = last_time
                .borrow()
                .elapsed()
                .unwrap_or_else(|_| Duration::default());
            *last_time.borrow_mut() = SystemTime::now();

            canvas.clear();

            // TODO: Resize on resize event to avoid WASM boundary crossing on the hot path
            let window_size = get_window_inner_size(&window)?;
            canvas.resize(window_size);

            {
                let delta_time = elapsed.as_millis() as f32 / 1000.0;
                let context = EngineContext {
                    render_ctx: canvas.context(),
                    delta_time,
                    input: &input.borrow(),
                };
                (logic)(&context)?;
            }

            input.borrow_mut().transition_states();

            Ok(())
        }),
    )
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
