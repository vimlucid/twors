use crate::error::Result;
use wasm_bindgen::{JsCast, convert::FromWasmAbi, prelude::Closure};
use web_sys::Window;

pub type EventHandler<T> = Closure<dyn Fn(T)>;

pub fn attach<T: FromWasmAbi + 'static>(
    window: &Window,
    event_name: &str,
    handler: EventHandler<T>,
) -> Result<()> {
    window
        .add_event_listener_with_callback(event_name, handler.as_ref().unchecked_ref())
        .map_err(|_| format!("Failed to attach {} event listener", event_name))?;

    handler.forget();

    Ok(())
}
