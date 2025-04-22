use crate::{Vertex2, error::Result};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, Element, HtmlCanvasElement};

pub struct Canvas {
    element: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
}

impl Canvas {
    pub fn new(element: Element) -> Result<Self> {
        let element = element
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| "Could not cast element into canvas")?;

        let context = element
            .get_context("2d")
            .map_err(|_| "Error getting the canvas context")?;
        let context = context.ok_or("Failed to find a canvas context")?;
        let context = context
            .dyn_into::<CanvasRenderingContext2d>()
            .map_err(|_| "Could not cast canvas into a canvas rendering context")?;

        Ok(Self { element, context })
    }

    pub fn context(&self) -> &CanvasRenderingContext2d {
        &self.context
    }

    pub fn resize(&self, size: Vertex2<u32>) {
        self.element.set_width(size.x);
        self.element.set_height(size.y);
    }

    pub fn clear(&self) {
        self.context.clear_rect(
            0.0,
            0.0,
            self.element.width() as f64,
            self.element.height() as f64,
        );
    }
}
