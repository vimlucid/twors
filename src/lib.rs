#![allow(dead_code)] // TODO: Remove after initial setup

pub mod error;

use crate::error::{Error, Result};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn draw(canvas_id: &str) -> Result<()> {
    let not_found_err = |entity: &str| Error::from_string(format!("Did not find '{}'", entity));

    let window = web_sys::window().ok_or_else(|| not_found_err("window"))?;
    let document = window.document().ok_or_else(|| not_found_err("document"))?;
    let canvas = document
        .get_element_by_id(canvas_id)
        .ok_or_else(|| not_found_err(canvas_id))?;
    let canvas: HtmlCanvasElement =
        canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| {
                Error::from_string(format!("Element with ID '{}' is not a canvas", canvas_id))
            })?;

    let context = canvas
        .get_context("2d")
        .map_err(|err| {
            Error::from_string(format!(
                "Error encountered while getting canvas context: {:?}",
                err
            ))
        })?
        .ok_or_else(|| Error::from_str("Could not find canvas context"))?;
    let context = context
        .dyn_into::<CanvasRenderingContext2d>()
        .map_err(|_| Error::from_str("Could not cast into rendering context"))?;

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
