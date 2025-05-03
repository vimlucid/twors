use super::Context;
use crate::Vertex2;
use std::ops::Add;
use web_sys::CanvasRenderingContext2d;

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

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Layer {
    One,
    Two,
    Three,
    Four,
    Five,
}

pub struct Renderable {
    pub transform: Transform,
    pub vertices: Vec<Vertex2<f32>>,
    pub style: fn(&CanvasRenderingContext2d),
    pub layer: Layer,
}

pub trait Component {
    fn transform(&self) -> &Transform;
    fn renderables(&self) -> &[Renderable];

    fn update(&mut self, ctx: &mut Context);

    fn children(&self) -> Vec<&dyn Component>;
    fn children_mut(&mut self) -> Vec<&mut dyn Component>;
}
