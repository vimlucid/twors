use super::EngineContext;
use crate::Vertex2;
use web_sys::CanvasRenderingContext2d;

const DEFAULT_SCALE: Vertex2<f32> = const { Vertex2::new(1.0, 1.0) };

pub struct Transform {
    pub position: Vertex2<f32>,
    pub scale: Vertex2<f32>,
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

pub struct Renderable {
    pub vertices: Vec<Vertex2<f32>>,
    pub style: fn(&CanvasRenderingContext2d),
}

pub trait Logic {
    fn on_update(&mut self, ctx: &EngineContext, transform: &mut Transform);
}

pub struct Component {
    pub transform: Transform,
    pub renderables: Vec<Renderable>,
    pub logic: Box<dyn Logic>,
}
