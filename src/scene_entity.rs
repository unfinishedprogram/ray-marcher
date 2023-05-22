use crate::{quaternion::Quaternion, scene_buffer::SceneBufferBuilder, vector3::Vec3};

pub enum SceneEntity {
    Sphere {
        radius: f32,
    },

    Translate {
        pointer: Box<SceneEntity>,
        v: Vec3,
    },

    Box {
        dimensions: Vec3,
    },

    Rotate {
        pointer: Box<SceneEntity>,
        q: Quaternion,
    },

    Cylinder {
        radius: f32,
        height: f32,
    },

    Subtract {
        a: Box<SceneEntity>,
        b: Box<SceneEntity>,
    },
}

impl SceneEntity {
    pub fn translate(self, v: Vec3) -> Self {
        SceneEntity::Translate {
            pointer: Box::new(self),
            v,
        }
    }

    pub fn rotate(self, q: Quaternion) -> Self {
        SceneEntity::Rotate {
            pointer: Box::new(self),
            q,
        }
    }

    pub fn subtract(self, other: SceneEntity) -> Self {
        SceneEntity::Subtract {
            a: Box::new(self),
            b: Box::new(other),
        }
    }
}

#[derive(Default)]
pub struct SceneBuilder {
    items: Vec<SceneEntity>,
}

fn append(builder: &mut SceneBufferBuilder, item: SceneEntity) -> &mut SceneBufferBuilder {
    match item {
        SceneEntity::Sphere { radius } => builder.sphere(radius),
        SceneEntity::Box { dimensions } => builder.r#box(dimensions),
        SceneEntity::Cylinder { radius, height } => builder.cylinder(height, radius),
        SceneEntity::Translate { pointer, v } => append(builder, *pointer).translate(v),
        SceneEntity::Rotate { pointer, q } => todo!(),
        SceneEntity::Subtract { a, b } => {
            let a_ptr = builder.entities_length;
            append(builder, *a);
            let b_ptr = builder.entities_length;
            append(builder, *b);
            builder.subtract(a_ptr as u32, b_ptr as u32)
        }
    };
    builder
}

impl SceneBuilder {
    pub fn add(&mut self, item: SceneEntity) {
        self.items.push(item);
    }

    pub fn build(self) -> SceneBufferBuilder {
        let mut builder = SceneBufferBuilder::new();
        for item in self.items {
            append(&mut builder, item);
        }

        builder
    }
}
