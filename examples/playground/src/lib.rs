mod excrement;
mod player;

use std::collections::HashMap;
use twors::{Engine, Result};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn entry(canvas_id: &str) -> Result<()> {
    console_log::init().unwrap();

    let mut components = HashMap::default();
    components.insert(String::from("Player"), player::new());

    let engine = Engine::new(canvas_id, components)?;
    engine.run()?;

    Ok(())
}
