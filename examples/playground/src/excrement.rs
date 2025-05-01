use twors::{prelude::*, shape_factory};

pub fn new(position: Vertex2<f32>) -> Component {
    Component {
        transform: Transform::from_position(position),
        renderables: vec![Renderable {
            vertices: shape_factory::square(10.0),
            style: |ctx: &CanvasRenderingContext2d| {
                ctx.set_fill_style_str("brown");
                ctx.set_line_width(1.0);
                ctx.set_stroke_style_str("black");
                ctx.stroke();
                ctx.fill();
            },
        }],
        logic: Box::new(ExcrementLogic::default()),
    }
}

#[derive(Default)]
struct ExcrementLogic {}

impl Logic for ExcrementLogic {
    fn on_init(&mut self, _: &mut Context, _: &mut Transform) {}

    fn on_update(&mut self, _: &mut Context, _: &mut Transform) {}
}
