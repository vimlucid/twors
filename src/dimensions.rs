use crate::Vertex2;

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
