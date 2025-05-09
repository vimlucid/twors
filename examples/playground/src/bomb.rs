use twors::{prelude::*, shape_factory};

#[derive(Component)]
pub struct Bomb {
    transform: Transform,
    renderables: Vec<Renderable>,
}

impl Bomb {
    pub fn new(position: Vertex2<f32>) -> Self {
        Self {
            transform: Transform::from_position(position),
            renderables: vec![Renderable {
                transform: Transform::default(),
                vertices: shape_factory::square(10.0),
                style: |ctx: &CanvasRenderingContext2d| {
                    ctx.set_fill_style_str("brown");
                    ctx.set_line_width(1.0);
                    ctx.set_stroke_style_str("black");
                    ctx.stroke();
                    ctx.fill();
                },
                layer: twors::Layer::Two,
            }],
        }
    }
}

impl ComponentLifecycle for Bomb {
    fn update(&mut self, _: &mut Context) {}
}
