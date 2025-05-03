mod excrement;
mod player;
mod sand_field;

use sand_field::SandField;
use twors::{Engine, Result};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn entry(canvas_id: &str) -> Result<()> {
    console_log::init().unwrap();

    let engine = Engine::new(canvas_id, vec![Box::new(SandField::new())])?;
    engine.run()?;

    Ok(())
}
