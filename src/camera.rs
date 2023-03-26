use crate::{
    angle::Angle,
    quaternion::Quaternion,
    vector3::{Vec3, Vector3},
};

#[repr(C)]
pub struct Camera {
    pub fov: Angle,
    pub position: Vec3,
    pub orientation: Quaternion,
    pub clip_plane: (f32, f32),
    pub aspect_ratio: f32,
}

impl Camera {
    pub fn new(
        fov: Angle,
        aspect_ratio: f32,
        position: Vec3,
        orientation: Quaternion,
        clip_plane: (f32, f32),
    ) -> Self {
        Camera {
            fov,
            aspect_ratio,
            position,
            orientation,
            clip_plane,
        }
    }

    pub fn get_ray_direction(&self, x: f32, y: f32) -> Vec3 {
        let y = -y + 0.5;
        let x = (x - 0.5) * self.aspect_ratio;

        (x, y, self.clip_plane.0)
            .normalize()
            .apply_rotation(self.orientation)
    }
}

// Base stack item mostly for padding
struct SceneItem {
    item_type: u32,
    _padding: [u32; 7],
}

#[repr(align(16))]
struct Scene {
    entities: [SceneItem; 4],
    render_queue: [u32; 4],
    entities_length: u32,
    render_queue_length: u32,
    _padding: [u32; 2],
}
