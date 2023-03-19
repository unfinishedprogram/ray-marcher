use crate::{camera::Camera, entity::Entity, light::Light, vector3::Vec3};

pub struct Scene {
    pub camera: Camera,
    pub entities: Vec<Entity>,
    pub lights: Vec<Light>,
}

pub struct SceneBuilder {
    camera: Camera,
    entities: Vec<Entity>,
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

    pub fn add(mut self, entity: Entity) -> Self {
        self.entities.push(entity);
        self
    }

    pub fn light(mut self, position: impl Into<Vec3>, color: impl Into<Vec3>) -> Self {
        self.lights.push(Light::new(position, color));
        self
    }
}

pub struct SceneQueryResult<'a> {
    pub entity: &'a Entity,
    pub distance: f64,
}

impl Scene {
    pub fn query_entities(&self, point: Vec3) -> SceneQueryResult {
        self.entities
            .iter()
            .map(|entity| SceneQueryResult {
                entity,
                distance: entity.distance(point),
            })
            .min_by(|a, b| {
                a.distance
                    .partial_cmp(&b.distance)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap()
    }
}
