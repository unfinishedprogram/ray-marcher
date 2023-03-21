use crate::{
    angle::Angle,
    quaternion::Quaternion,
    vector3::{Vec3, Vector3},
};

pub struct Camera {
    pub fov: Angle,
    pub position: Vec3,
    pub orientation: Quaternion,
    pub clip_plane: (f64, f64),
    pub aspect_ratio: f64,
}

impl Camera {
    pub fn new(
        fov: Angle,
        aspect_ratio: f64,
        position: Vec3,
        orientation: Quaternion,
        clip_plane: (f64, f64),
    ) -> Self {
        Camera {
            fov,
            aspect_ratio,
            position,
            orientation,
            clip_plane,
        }
    }

    pub fn get_ray_direction(&self, x: f64, y: f64) -> Vec3 {
        let y = -y + 0.5;
        let x = (x - 0.5) * self.aspect_ratio;

        (x, y, self.clip_plane.0)
            .normalize()
            .apply_rotation(self.orientation)
    }
}
