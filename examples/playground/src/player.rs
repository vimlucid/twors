mod eye;
mod head;
mod mouth;

use mouth::Mouth;
use twors::prelude::*;

pub const SIZE: f32 = 60.0;

const SPEED: f32 = 200.0;

#[derive(Component)]
pub struct Player {
    #[child]
    mouth: Mouth,

    pub transform: Transform,
    renderables: Vec<Renderable>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            mouth: Mouth::new(Vertex2::new(0.0, 2.0)),
            transform: Transform::default(),
            renderables: vec![
                head::new(),
                eye::new(Vertex2::new(-15.0, -8.0)),
                eye::new(Vertex2::new(15.0, -8.0)),
            ],
        }
    }
}

impl ComponentLifecycle for Player {
    fn update(&mut self, ctx: &mut Context) {
        if ctx.input.keyboard.is_down(Key::A) {
            self.transform.position += Vertex2 {
                x: -SPEED * ctx.delta_time(),
                y: 0.0,
            };
        }

        if ctx.input.keyboard.is_down(Key::D) {
            self.transform.position += Vertex2 {
                x: SPEED * ctx.delta_time(),
                y: 0.0,
            };
        }

        if ctx.input.keyboard.is_down(Key::W) {
            self.transform.position += Vertex2 {
                x: 0.0,
                y: -SPEED * ctx.delta_time(),
            };
        }

        if ctx.input.keyboard.is_down(Key::S) {
            self.transform.position += Vertex2 {
                x: 0.0,
                y: SPEED * ctx.delta_time(),
            };
        }
    }
}
