use twors::{prelude::*, shape_factory};

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
            renderables: vec![Renderable {
                vertices: shape_factory::square(SIZE),
                style: |ctx: &CanvasRenderingContext2d| {
                    ctx.set_fill_style_str("red");
                    ctx.set_line_width(1.0);
                    ctx.set_stroke_style_str("black");
                    ctx.stroke();
                    ctx.fill();
                },
            }],
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

    fn on_update(&mut self, ctx: &mut Context) {
        if ctx.input.mouse.is_pressed(Mouse::LMB) {
            // let last_id = self.next_id + 1;
            // self.next_id += 1;

            // ctx.add_component(last_id, excrement::new(transform.position));
        }

        if ctx.input.mouse.is_released(Mouse::RMB) {
            // if let Some(prev_excrement_id) = self.decrement_next_excrement_id() {
            //     ctx.remove_component(prev_excrement_id);
            // }
        }

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

    fn get_children(&mut self) -> Vec<&mut dyn Component> {
        Vec::default()
    }
}
