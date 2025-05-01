use twors::{Component, Context, Key, Logic, Renderable, Transform, Vertex2, shape_factory};
use web_sys::CanvasRenderingContext2d;

pub fn new() -> Component {
    Component {
        transform: Transform::from_position(Vertex2::new(300.0, 200.0)),
        renderables: vec![Renderable {
            vertices: shape_factory::square(30.0),
            style: |ctx: &CanvasRenderingContext2d| {
                ctx.set_fill_style_str("red");
                ctx.set_line_width(1.0);
                ctx.set_stroke_style_str("black");
                ctx.stroke();
                ctx.fill();
            },
        }],
        logic: Box::new(PlayerLogic::default()),
    }
}

const SPEED: f32 = 100.0;

#[derive(Default)]
struct PlayerLogic {}

impl Logic for PlayerLogic {
    fn on_init(&mut self, _: &Context, _: &mut Transform) {}

    fn on_update(&mut self, ctx: &Context, transform: &mut Transform) {
        if ctx.input.keyboard.is_down(Key::A) {
            transform.position += Vertex2 {
                x: -SPEED * ctx.delta_time,
                y: 0.0,
            };
        }

        if ctx.input.keyboard.is_down(Key::D) {
            transform.position += Vertex2 {
                x: SPEED * ctx.delta_time,
                y: 0.0,
            };
        }

        if ctx.input.keyboard.is_down(Key::W) {
            transform.position += Vertex2 {
                x: 0.0,
                y: -SPEED * ctx.delta_time,
            };
        }

        if ctx.input.keyboard.is_down(Key::S) {
            transform.position += Vertex2 {
                x: 0.0,
                y: SPEED * ctx.delta_time,
            };
        }
    }
}
