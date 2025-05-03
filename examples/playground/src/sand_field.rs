use crate::{
    excrement::Excrement,
    player::{self, Player},
};
use twors::{dimensions::Dimensions, prelude::*, shape_factory};

const SIZE: f32 = 400.0;

pub struct SandField {
    player: Player,
    excrements: Vec<Excrement>,

    transform: Transform,
    renderables: Vec<Renderable>,
}

impl Component for SandField {
    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn renderables(&self) -> &[Renderable] {
        &self.renderables
    }

    fn update(&mut self, ctx: &mut Context) {
        self.restrict_player_movement();

        if ctx.input.mouse.is_pressed(Mouse::LMB) {
            self.excrements
                .push(Excrement::new(self.player.transform.position));
        }
    }

    fn children(&self) -> Vec<&dyn Component> {
        let mut children = vec![&self.player as &dyn Component];
        children.extend(self.excrements.iter().map(|cmp| cmp as &dyn Component));
        children
    }

    fn children_mut(&mut self) -> Vec<&mut dyn Component> {
        let mut children = vec![&mut self.player as &mut dyn Component];
        children.extend(
            self.excrements
                .iter_mut()
                .map(|cmp| cmp as &mut dyn Component),
        );
        children
    }
}

impl SandField {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            excrements: Vec::default(),

            transform: Transform::from_position(Vertex2::new(250.0, 250.0)),
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
                layer: twors::Layer::Five,
            }],
        }
    }

    fn restrict_player_movement(&mut self) {
        let player_dim =
            Dimensions::new(self.player.transform.position, player::SIZE, player::SIZE);
        let field_dim = Dimensions::new(self.transform.position, SIZE, SIZE);

        if player_dim.right() > field_dim.right() {
            self.player.transform.position.x = field_dim.right() - player_dim.half_width();
        } else if player_dim.left() < field_dim.left() {
            self.player.transform.position.x = field_dim.left() + player_dim.half_width();
        }

        if player_dim.top() < field_dim.top() {
            self.player.transform.position.y = field_dim.top() + player_dim.half_height();
        } else if player_dim.bottom() > field_dim.bottom() {
            self.player.transform.position.y = field_dim.bottom() - player_dim.half_height();
        }
    }
}
