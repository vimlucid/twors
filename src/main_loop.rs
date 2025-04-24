use crate::{Canvas, Vertex2, error::Result};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{CanvasRenderingContext2d, Window, js_sys::Function};
use web_time::{Duration, SystemTime};

pub struct EngineContext<'a> {
    pub render_ctx: &'a CanvasRenderingContext2d,
    pub delta_time: f32,
}
type MainLoopLogic = dyn Fn(&EngineContext) -> Result<()>;

pub fn run(canvas_id: &str, logic: Rc<MainLoopLogic>) -> Result<()> {
    let not_found_msg = |entity: &str| format!("Did not find '{}'", entity);
    let window = web_sys::window().ok_or_else(|| not_found_msg("window"))?;
    let document = window.document().ok_or_else(|| not_found_msg("document"))?;
    let canvas = document
        .get_element_by_id(canvas_id)
        .ok_or_else(|| not_found_msg(canvas_id))?;
    let canvas = Canvas::new(canvas)?;

    let last_time = Rc::new(RefCell::new(SystemTime::now()));
    let last_time = last_time.clone();

    let window = Rc::new(window);

    request_animation_frame_recursive(
        window.clone(),
        Rc::new(move || {
            let elapsed = last_time
                .borrow()
                .elapsed()
                .unwrap_or_else(|_| Duration::default());
            *last_time.borrow_mut() = SystemTime::now();

            canvas.clear();

            let window_size = get_window_inner_size(&window)?;
            canvas.resize(window_size); // TODO: Do it on resize event to avoid WASM boundary
            // crossing on the hot path

            let delta_time = elapsed.as_millis() as f32 / 1000.0;
            let context = EngineContext {
                render_ctx: canvas.context(),
                delta_time,
            };
            (logic)(&context)?;

            Ok(())
        }),
    )
}

type RequestAnimationFrameCallback = dyn Fn() -> Result<()>;

fn request_animation_frame_recursive(
    window: Rc<Window>,
    callback: Rc<RequestAnimationFrameCallback>,
) -> Result<()> {
    let callback_ref_alpha = Rc::new(RefCell::new(None));
    let callback_ref_beta = callback_ref_alpha.clone();

    let wasm_window = window;
    let js_window = wasm_window.clone();
    *callback_ref_alpha.borrow_mut() = Some(Closure::<dyn FnMut()>::new(move || {
        match callback() {
            Ok(_) => (),
            Err(_error) => {
                return;
            }
        }

        let closure = callback_ref_beta.borrow();
        let request_result = request_animation_frame(&js_window, closure.as_ref().unwrap());

        match request_result {
            Ok(_) => {}
            Err(_error) => (),
        }
    }));

    let closure = callback_ref_alpha.borrow();
    request_animation_frame(&wasm_window, closure.as_ref().unwrap())?;

    // TODO: Block until an error is returned
    Ok(())
}

fn request_animation_frame(window: &Window, closure: &Closure<dyn FnMut()>) -> Result<()> {
    let closure = closure.as_ref().as_ref().unchecked_ref::<Function>();

    window
        .request_animation_frame(closure)
        .map_err(|_| "Failed to request animation frame")?;
    Ok(())
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
