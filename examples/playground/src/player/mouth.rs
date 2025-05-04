use twors::{prelude::*, shape_factory};

const MAX_SCALE: f32 = 1.5;
const MIN_SCALE: f32 = 1.0;

enum State {
    Opening,
    Closing,
}

#[derive(Component)]
pub struct Mouth {
    state: State,

    transform: Transform,
    renderables: Vec<Renderable>,
}

impl Mouth {
    pub fn new(position: Vertex2<f32>) -> Self {
        Self {
            state: State::Opening,

            transform: Transform::from_position(position),
            renderables: vec![Renderable {
                transform: Transform::from_position(position),
                vertices: shape_factory::rectangle(15.0, 5.0),
                style: |ctx: &CanvasRenderingContext2d| {
                    ctx.set_fill_style_str("black");
                    ctx.set_line_width(1.0);
                    ctx.set_stroke_style_str("black");
                    ctx.stroke();
                    ctx.fill();
                },
                layer: twors::Layer::One,
            }],
        }
    }
}

impl ComponentLifecycle for Mouth {
    fn update(&mut self, ctx: &mut Context) {}
}
