use crate::{
    bomb::Bomb,
    player::{self, Player},
};
use twors::{dimensions::Dimensions, prelude::*, shape_factory};

const SIZE: f32 = 400.0;

#[derive(Component)]
pub struct Battlefield {
    #[child]
    player: Player,

    #[children]
    bombs: Vec<Bomb>,

    transform: Transform,
    renderables: Vec<Renderable>,
}

impl ComponentLifecycle for Battlefield {
    fn update(&mut self, ctx: &mut Context) {
        self.restrict_player_within_field();

        if ctx.input.mouse.is_pressed(Mouse::LMB) {
            self.bombs.push(Bomb::new(self.player.transform.position));
        }
    }
}

impl Battlefield {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            bombs: Vec::default(),

            transform: Transform::from_position(Vertex2::new(
                SIZE / 2.0 + 100.0,
                SIZE / 2.0 + 300.0,
            )),
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

    fn restrict_player_within_field(&mut self) {
        let player_dim = Dimensions::new(
            self.player.transform.absolute().position,
            player::SIZE,
            player::SIZE,
        );
        let field_dim = Dimensions::new(self.transform.absolute().position, SIZE, SIZE);

        let player_transform = self.player.transform.absolute().clone();
        let player_transform =
            Battlefield::restrict_position(player_transform, &player_dim, &field_dim);
        self.player.transform.set_absolute(&player_transform);
    }

    fn restrict_position(
        mut player: Transform,
        player_dim: &Dimensions,
        field_dim: &Dimensions,
    ) -> Transform {
        player.position.x =
            Battlefield::restrict_horizontal(player.position.x, player_dim, field_dim);
        player.position.y =
            Battlefield::restrict_vertical(player.position.y, player_dim, field_dim);
        player
    }

    fn restrict_horizontal(player_x: f32, player_dim: &Dimensions, field_dim: &Dimensions) -> f32 {
        if player_dim.right() > field_dim.right() {
            field_dim.right() - player_dim.half_width()
        } else if player_dim.left() < field_dim.left() {
            field_dim.left() + player_dim.half_width()
        } else {
            player_x
        }
    }

    fn restrict_vertical(player_y: f32, player_dim: &Dimensions, field_dim: &Dimensions) -> f32 {
        if player_dim.top() < field_dim.top() {
            field_dim.top() + player_dim.half_height()
        } else if player_dim.bottom() > field_dim.bottom() {
            field_dim.bottom() - player_dim.half_height()
        } else {
            player_y
        }
    }
}
