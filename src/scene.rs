use crate::{camera::Camera, primitives::SignedDistance};

pub struct Scene {
    pub camera: Camera,
    pub entities: Vec<Box<dyn SignedDistance>>,
}

pub struct SceneBuilder {
    pub camera: Camera,
    pub entities: Vec<Box<dyn SignedDistance>>,
}

impl SceneBuilder {
    pub fn build(self) -> Scene {
        Scene {
            camera: self.camera,
            entities: self.entities,
        }
    }

    pub fn new(camera: Camera) -> Self {
        Self {
            camera,
            entities: vec![],
        }
    }

    pub fn add<T: SignedDistance + 'static>(mut self, entity: T) -> Self {
        self.entities.push(Box::new(entity));
        self
    }
}
