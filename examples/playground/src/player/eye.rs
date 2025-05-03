use twors::{prelude::*, shape_factory};

pub fn new(position: Vertex2<f32>) -> Renderable {
    Renderable {
        transform: Transform::from_position(position),
        vertices: shape_factory::square(super::SIZE / 5.0),
        style: |ctx: &CanvasRenderingContext2d| {
            ctx.set_fill_style_str("black");
            ctx.set_line_width(1.0);
            ctx.set_stroke_style_str("black");
            ctx.stroke();
            ctx.fill();
        },
        layer: twors::Layer::One,
    }
}
