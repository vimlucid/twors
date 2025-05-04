use crate::Vertex2;
use std::ops::Add;

const DEFAULT_SCALE: Vertex2<f32> = const { Vertex2::new(1.0, 1.0) };

/// Directly modify the `position` and `scale` fields - the engine will automatically pick up the
/// changes and move your `Renderable`s
#[derive(Clone, Debug)]
pub struct Transform {
    pub position: Vertex2<f32>,
    pub scale: Vertex2<f32>,
}

impl Add<&Transform> for Transform {
    type Output = Transform;

    fn add(self, rhs: &Self) -> Self::Output {
        Self::Output {
            position: self.position + rhs.position,
            scale: self.scale * rhs.scale,
        }
    }
}

impl Transform {
    pub fn from_position(position: Vertex2<f32>) -> Self {
        Self {
            position,
            scale: DEFAULT_SCALE,
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vertex2::default(),
            scale: DEFAULT_SCALE,
        }
    }
}
