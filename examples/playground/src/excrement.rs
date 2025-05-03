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

impl Component for Excrement {
    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn renderables(&self) -> &[Renderable] {
        &self.renderables
    }

    fn update(&mut self, _: &mut Context) {}

    fn children(&self) -> Vec<&dyn Component> {
        Vec::default()
    }

    fn children_mut(&mut self) -> Vec<&mut dyn Component> {
        Vec::default()
    }
}
