use std::{cell::RefCell, rc::Rc};

use twors::{Result, Vertex2, main_loop};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
pub fn entry(canvas_id: &str) -> Result<()> {
    let player_pos = Rc::new(RefCell::new(Vertex2 { x: 0.0, y: 0.0 }));
    let player_pos = player_pos.clone();

    const SPEED: f32 = 30.0;
    main_loop::run(
        canvas_id,
        Rc::new(move |ctx| {
            let mut player_pos = player_pos.borrow_mut();
            player_pos.x += (SPEED * 2.0 * ctx.delta_time) as f64;
            player_pos.y += (SPEED * ctx.delta_time) as f64;

            draw_square(ctx.render_ctx, &player_pos, &Vertex2 { x: 40.0, y: 40.0 });

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
