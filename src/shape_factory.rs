use crate::Vertex2;

pub fn square(size: f32) -> Vec<Vertex2<f32>> {
    rectangle(size, size)
}

pub fn rectangle(width: f32, height: f32) -> Vec<Vertex2<f32>> {
    let half_width = width / 2.0;
    let half_height = height / 2.0;
    vec![
        Vertex2 {
            x: -half_width,
            y: -half_height,
        },
        Vertex2 {
            x: half_width,
            y: -half_height,
        },
        Vertex2 {
            x: half_width,
            y: half_height,
        },
        Vertex2 {
            x: -half_width,
            y: half_height,
        },
    ]
}
