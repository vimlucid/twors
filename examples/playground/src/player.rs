use twors::{prelude::*, shape_factory};

use crate::excrement;

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

const SPEED: f32 = 250.0;

#[derive(Default)]
struct PlayerLogic {
    last_id: usize,
}

impl PlayerLogic {
    fn increment_next_id(&mut self) -> String {
        let last_id = self.last_id;
        self.last_id += 1;
        PlayerLogic::build_excrement_id(last_id)
    }

    fn decrement_next_excrement_id(&mut self) -> Option<String> {
        if self.last_id > 0 {
            self.last_id -= 1;
            return Some(PlayerLogic::build_excrement_id(self.last_id));
        }

        None
    }

    fn build_excrement_id(id: usize) -> String {
        format!("excrement_{}", id)
    }
}

impl Logic for PlayerLogic {
    fn on_init(&mut self, _: &mut Context, _: &mut Transform) {}

    fn on_update(&mut self, ctx: &mut Context, transform: &mut Transform) {
        if ctx.input.mouse.is_pressed(Mouse::LMB) {
            let last_id = self.increment_next_id();
            ctx.add_component(last_id, excrement::new(transform.position));
        }

        if ctx.input.mouse.is_released(Mouse::RMB) {
            if let Some(prev_excrement_id) = self.decrement_next_excrement_id() {
                ctx.remove_component(prev_excrement_id);
            }
        }

        if ctx.input.keyboard.is_down(Key::A) {
            transform.position += Vertex2 {
                x: -SPEED * ctx.delta_time(),
                y: 0.0,
            };
        }

        if ctx.input.keyboard.is_down(Key::D) {
            transform.position += Vertex2 {
                x: SPEED * ctx.delta_time(),
                y: 0.0,
            };
        }

        if ctx.input.keyboard.is_down(Key::W) {
            transform.position += Vertex2 {
                x: 0.0,
                y: -SPEED * ctx.delta_time(),
            };
        }

        if ctx.input.keyboard.is_down(Key::S) {
            transform.position += Vertex2 {
                x: 0.0,
                y: SPEED * ctx.delta_time(),
            };
        }
    }
}
