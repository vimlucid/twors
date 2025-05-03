mod eye;
mod head;

use twors::prelude::*;

pub const SIZE: f32 = 30.0;

const SPEED: f32 = 200.0;

pub struct Player {
    pub transform: Transform,
    renderables: Vec<Renderable>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            transform: Transform::default(),
            renderables: vec![
                head::new(),
                eye::new(Vertex2::new(-8.0, -8.0)),
                eye::new(Vertex2::new(8.0, -8.0)),
            ],
        }
    }
}

impl Component for Player {
    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn renderables(&self) -> &[Renderable] {
        &self.renderables
    }

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

    fn children(&self) -> Vec<&dyn Component> {
        Vec::default()
    }

    fn children_mut(&mut self) -> Vec<&mut dyn Component> {
        Vec::default()
    }
}
