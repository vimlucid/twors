use twors::{prelude::*, shape_factory};

pub struct Excrement {
    transform: Transform,
    renderables: Vec<Renderable>,
}

impl Excrement {
    pub fn new(position: Vertex2<f32>) -> Self {
        Self {
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
        }
    }
}

impl Component for Excrement {
    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn renderables(&self) -> &[Renderable] {
        &self.renderables
    }

    fn on_update(&mut self, _: &mut Context) {}

    fn get_children(&mut self) -> Vec<&mut dyn Component> {
        Vec::default()
    }
}
