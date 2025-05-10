use twors::{dimensions::Dimensions, prelude::*, shape_factory};

#[derive(Copy, Clone, Debug)]
pub enum DragState {
    Dragged(Vertex2<f32>),
    Resting,
}

#[derive(Component)]
pub struct Bomb {
    drag_state: DragState,

    transform: Transform,
    renderables: Vec<Renderable>,
}

const SIZE: f32 = 40.0;

impl Bomb {
    pub fn new(position: Vertex2<f32>) -> Self {
        Self {
            drag_state: DragState::Resting,

            transform: Transform::from_position(position),
            renderables: vec![Renderable {
                transform: Transform::default(),
                vertices: shape_factory::square(SIZE),
                style: |ctx: &CanvasRenderingContext2d| {
                    ctx.set_fill_style_str("yellow");
                    ctx.set_line_width(1.0);
                    ctx.set_stroke_style_str("black");
                    ctx.stroke();
                    ctx.fill();
                },
                layer: twors::Layer::Two,
            }],
        }
    }

    pub fn drag_state(&self) -> DragState {
        self.drag_state
    }
}

impl ComponentLifecycle for Bomb {
    fn update(&mut self, ctx: &mut Context) {
        let mouse = &ctx.input.mouse;

        let header = Dimensions::new(self.transform.absolute().position, SIZE, SIZE);
        if mouse.is_pressed(Mouse::RMB) && header.contains(mouse.position()) {
            let relative_mouse_pos = mouse.position() - self.transform.absolute().position;
            self.drag_state = DragState::Dragged(relative_mouse_pos);
        }

        if mouse.is_released(Mouse::RMB) && matches!(self.drag_state, DragState::Dragged(_)) {
            self.drag_state = DragState::Resting;
        }
    }
}
