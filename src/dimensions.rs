//! A useful utility to easily handle common calculations like getting the `x` value of the left
//! and right sides of a shape, getting coordinates of a corner, padded position relative to a
//! point, etc...

use crate::Vertex2;

#[derive(Debug)]
pub struct Dimensions {
    position: Vertex2<f32>,
    width: f32,
    height: f32,
}

impl Dimensions {
    pub fn new(position: Vertex2<f32>, width: f32, height: f32) -> Self {
        Self {
            position,
            width,
            height,
        }
    }

    pub fn half_width(&self) -> f32 {
        self.width / 2.0
    }

    pub fn half_height(&self) -> f32 {
        self.height / 2.0
    }

    pub fn left(&self) -> f32 {
        self.position.x - self.half_width()
    }

    pub fn right(&self) -> f32 {
        self.position.x + self.half_width()
    }

    pub fn top(&self) -> f32 {
        self.position.y - self.half_height()
    }

    pub fn bottom(&self) -> f32 {
        self.position.y + self.half_height()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const POS_X: f32 = 30.0;
    const POS_Y: f32 = 40.0;
    const POSITION: Vertex2<f32> = Vertex2::new(POS_X, POS_Y);

    const WIDTH: f32 = 20.0;
    const HEIGHT: f32 = 10.0;

    #[test]
    fn half_size() {
        let dimensions = Dimensions::new(POSITION, WIDTH, HEIGHT);

        assert_eq!(dimensions.half_width(), WIDTH / 2.0);
        assert_eq!(dimensions.half_height(), HEIGHT / 2.0);
    }

    #[test]
    fn sides() {
        let dimensions = Dimensions::new(POSITION, WIDTH, HEIGHT);

        assert_eq!(dimensions.left(), POS_X - (WIDTH / 2.0));
        assert_eq!(dimensions.right(), POS_X + (WIDTH / 2.0));
        assert_eq!(dimensions.top(), POS_Y - (HEIGHT / 2.0));
        assert_eq!(dimensions.bottom(), POS_Y + (HEIGHT / 2.0));
    }
}
