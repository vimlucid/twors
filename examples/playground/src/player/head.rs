use twors::{prelude::*, shape_factory};

pub fn new() -> Renderable {
    Renderable {
        transform: Transform::default(),
        vertices: shape_factory::square(super::SIZE),
        style: |ctx: &CanvasRenderingContext2d| {
            ctx.set_fill_style_str("orange");
            ctx.set_line_width(1.0);
            ctx.set_stroke_style_str("black");
            ctx.stroke();
            ctx.fill();
        },
        layer: twors::Layer::One,
    }
}
