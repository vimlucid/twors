use crate::Vertex2;
use std::ops::{Add, Sub};

const DEFAULT_SCALE: Vertex2<f32> = const { Vertex2::new(1.0, 1.0) };

/// Directly modify the `position` and `scale` fields - the engine will automatically pick up the
/// changes and move your `Renderable`s
#[derive(Clone, Debug)]
pub struct Transform {
    pub position: Vertex2<f32>,
    pub scale: Vertex2<f32>,
    pub parent: Option<Box<Transform>>,
}

impl Transform {
    pub fn from_position(position: Vertex2<f32>) -> Self {
        Self {
            position,
            scale: DEFAULT_SCALE,
            parent: None,
        }
    }

    pub fn absolute(&self) -> Transform {
        let parent = self.get_parent();
        parent + self
    }

    pub fn set_absolute(&mut self, transform: &Transform) {
        let parent = self.get_parent();
        *self = transform.clone() - &parent;
    }

    fn get_parent(&self) -> Transform {
        match &self.parent {
            Some(parent) => *parent.clone(),
            None => Transform::default(),
        }
    }
}

impl Add<&Transform> for Transform {
    type Output = Transform;

    fn add(self, rhs: &Transform) -> Self::Output {
        Self::Output {
            position: self.position + rhs.position,
            scale: self.scale * rhs.scale,
            parent: self.parent,
        }
    }
}

impl Sub<&Transform> for Transform {
    type Output = Transform;

    fn sub(self, rhs: &Transform) -> Self::Output {
        Self::Output {
            position: self.position - rhs.position,
            scale: self.scale / rhs.scale,
            parent: self.parent,
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vertex2::default(),
            scale: DEFAULT_SCALE,
            parent: None,
        }
    }
}
