use crate::{Canvas, Result};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn entry(canvas_id: &str) -> Result<()> {
    let not_found_msg = |entity: &str| format!("Did not find '{}'", entity);

    let window = web_sys::window().ok_or_else(|| not_found_msg("window"))?;
    let document = window.document().ok_or_else(|| not_found_msg("document"))?;
    let canvas = document
        .get_element_by_id(canvas_id)
        .ok_or_else(|| not_found_msg(canvas_id))?;
    let canvas = Canvas::new(canvas)?;

    let context = canvas.context();

    context.set_fill_style_str("red");
    context.set_line_width(1.0);
    context.set_stroke_style_str("green");

    const SIZE: f64 = 50.0;
    let offset = |val: f64| {
        const OFFSET: f64 = 100.0;
        val + OFFSET
    };

    context.begin_path();
    context.move_to(offset(0.0), offset(0.0));
    context.line_to(offset(SIZE), offset(0.0));
    context.line_to(offset(SIZE), offset(SIZE));
    context.line_to(offset(0.0), offset(SIZE));
    context.line_to(offset(0.0), offset(0.0));

    context.stroke();
    context.fill();

    Ok(())
}
