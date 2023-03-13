use crate::{camera::Camera, light::Light, primitives::SignedDistance, vector3::Vector3};

pub struct Scene {
    pub camera: Camera,
    pub entities: Vec<Box<dyn SignedDistance>>,
    pub lights: Vec<Light>,
}

pub struct SceneBuilder {
    camera: Camera,
    entities: Vec<Box<dyn SignedDistance>>,
    lights: Vec<Light>,
}

impl SceneBuilder {
    pub fn build(self) -> Scene {
        Scene {
            camera: self.camera,
            entities: self.entities,
            lights: self.lights,
        }
    }

    pub fn new(camera: Camera) -> Self {
        Self {
            camera,
            entities: vec![],
            lights: vec![],
        }
    }

    pub fn add<T: SignedDistance + 'static>(mut self, entity: T) -> Self {
        self.entities.push(Box::new(entity));
        self
    }

    pub fn light(mut self, position: impl Into<Vector3>, color: impl Into<Vector3>) -> Self {
        self.lights.push(Light::new(position, color));

        self
    }
}
