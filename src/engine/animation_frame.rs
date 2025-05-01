use crate::error::Result;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{Window, js_sys::Function};

pub type RequestAnimationFrameCallback = dyn Fn() -> Result<()>;

pub fn request_recursive(
    window: Rc<Window>,
    callback: Rc<RequestAnimationFrameCallback>,
) -> Result<()> {
    // https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html
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
