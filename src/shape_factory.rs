use crate::Vertex2;

pub fn square(size: f32) -> Vec<Vertex2<f32>> {
    rectangle(size, size)
}

pub fn rectangle(width: f32, height: f32) -> Vec<Vertex2<f32>> {
    vec![
        Vertex2 {
            x: -width,
            y: -height,
        },
        Vertex2 {
            x: width,
            y: -height,
        },
        Vertex2 {
            x: width,
            y: height,
        },
        Vertex2 {
            x: -width,
            y: height,
        },
    ]
}
