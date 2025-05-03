//! A set of utilities to create vertices for various shapes.
//! 
//! > **Note:** all of the shapes created via this module have a center of (0.0, 0.0) - this means
//! > that the vertices of the left side of the shape have a negative `x` vertex value.

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rectangle() {
        const WIDTH: f32 = 200.0;
        const HEIGHT: f32 = 50.0;
        let shape = super::rectangle(WIDTH, HEIGHT);

        assert_eq!(shape.len(), 4);
        assert_eq!(shape[0], Vertex2::new(-WIDTH / 2.0, -HEIGHT / 2.0));
        assert_eq!(shape[1], Vertex2::new(WIDTH / 2.0, -HEIGHT / 2.0));
        assert_eq!(shape[2], Vertex2::new(WIDTH / 2.0, HEIGHT / 2.0));
        assert_eq!(shape[3], Vertex2::new(-WIDTH / 2.0, HEIGHT / 2.0));
    }

    #[test]
    fn square() {
        const SIZE: f32 = 200.0;
        let shape = super::square(SIZE);

        assert_eq!(shape.len(), 4);
        assert_eq!(shape[0], Vertex2::new(-SIZE / 2.0, -SIZE / 2.0));
        assert_eq!(shape[1], Vertex2::new(SIZE / 2.0, -SIZE / 2.0));
        assert_eq!(shape[2], Vertex2::new(SIZE / 2.0, SIZE / 2.0));
        assert_eq!(shape[3], Vertex2::new(-SIZE / 2.0, SIZE / 2.0));
    }
}
