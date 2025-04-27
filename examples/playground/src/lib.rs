use log::info;
use std::{cell::RefCell, rc::Rc};
use twors::{
    Result, Vertex2,
    engine::{self, Key, Mouse},
};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
pub fn entry(canvas_id: &str) -> Result<()> {
    console_log::init().unwrap();

    let player_pos = Rc::new(RefCell::new(Vertex2 { x: 0.0, y: 0.0 }));
    let player_pos = player_pos.clone();

    const SPEED: f32 = 30.0;
    engine::run(
        canvas_id,
        Rc::new(move |ctx| {
            let mut player_pos = player_pos.borrow_mut();
            player_pos.x += (SPEED * 2.0 * ctx.delta_time) as f64;
            player_pos.y += (SPEED * ctx.delta_time) as f64;

            draw_square(ctx.render_ctx, &player_pos, &Vertex2 { x: 40.0, y: 40.0 });

            if ctx.input.mouse.is_pressed(Mouse::LMB) {
                info!("LMB pressed!");
            }

            if ctx.input.mouse.is_released(Mouse::RMB) {
                info!("RMB released!");
            }

            if ctx.input.keyboard.is_down(Key::ControlLeft) {
                info!("Left CTRL down");
            }

            Ok(())
        }),
    )?;

    Ok(())
}

fn draw_square(ctx: &CanvasRenderingContext2d, pos: &Vertex2<f64>, size: &Vertex2<f64>) {
    ctx.set_fill_style_str("red");
    ctx.set_line_width(1.0);
    ctx.set_stroke_style_str("black");

    ctx.begin_path();
    ctx.move_to(pos.x, pos.y);
    ctx.line_to(pos.x + size.x, pos.y);
    ctx.line_to(pos.x + size.x, pos.y + size.y);
    ctx.line_to(pos.x, pos.y + size.y);
    ctx.line_to(pos.x, pos.y);

    ctx.stroke();
    ctx.fill();
}
