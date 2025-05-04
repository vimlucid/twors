use crate::{
    excrement::Excrement,
    player::{self, Player},
};
use log::info;
use twors::{dimensions::Dimensions, prelude::*, shape_factory};

const SIZE: f32 = 400.0;

#[derive(Component)]
pub struct SandField {
    #[child]
    player: Player,

    #[children]
    excrements: Vec<Excrement>,

    transform: Transform,
    renderables: Vec<Renderable>,
}

impl ComponentLifecycle for SandField {
    fn update(&mut self, ctx: &mut Context) {
        self.restrict_player_movement();

        if ctx.input.mouse.is_pressed(Mouse::LMB) {
            self.excrements
                .push(Excrement::new(self.player.transform.position));
        }
    }
}

impl SandField {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            excrements: Vec::default(),

            transform: Transform::from_position(Vertex2::new(SIZE / 2.0, SIZE / 2.0)),
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
        let player = Dimensions::new(self.player.transform.position, player::SIZE, player::SIZE);
        let field = Dimensions::new(self.transform.position, SIZE, SIZE);

        info!("{}", player.top());
        if player.right() > field.right() {
            self.player.transform.position.x = field.right() - player.half_width();
        } else if player.left() < field.left() {
            self.player.transform.position.x = field.left() + player.half_width();
        }

        if player.top() < field.top() {
            self.player.transform.position.y = field.top() + player.half_height();
        } else if player.bottom() > field.bottom() {
            self.player.transform.position.y = field.bottom() - player.half_height();
        }
    }
}
