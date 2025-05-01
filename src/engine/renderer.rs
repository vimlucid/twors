use crate::{Transform, Vertex2, wasm_assert};
use web_sys::CanvasRenderingContext2d;

pub fn render(ctx: &CanvasRenderingContext2d, vertices: &[Vertex2<f32>], transform: &Transform) {
    wasm_assert!(vertices.len() > 2);

    ctx.begin_path();

    let start_vertex = vertices.first().unwrap();
    let start_vertex = apply_transform(*start_vertex, transform);

    ctx.move_to(start_vertex.x.into(), start_vertex.y.into());
    for curr_vertex in vertices {
        let curr_vertex = apply_transform(*curr_vertex, transform);
        ctx.line_to(curr_vertex.x.into(), curr_vertex.y.into());
    }
    ctx.line_to(start_vertex.x.into(), start_vertex.y.into());
}

fn apply_transform(vertex: Vertex2<f32>, transform: &Transform) -> Vertex2<f32> {
    (vertex * transform.scale) + transform.position
}
