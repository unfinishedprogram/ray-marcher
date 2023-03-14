use crate::{
    angle::Angle,
    quaternion::Quaternion,
    ray::ViewRay,
    vector3::{Vec3, Vector3},
};

pub struct Camera {
    pub fov: Angle,
    pub position: Vec3,
    pub orientation: Quaternion,
    pub clip_plane: (f64, f64),
    // Vertical aspect is baseline: 16:9 = ~1.77
    pub aspect_ratio: f64,
}

impl Camera {
    pub fn new(
        fov: Angle,
        aspect_ratio: f64,
        position: impl Into<Vec3>,
        orientation: Quaternion,
        clip_plane: (f64, f64),
    ) -> Self {
        Camera {
            fov,
            aspect_ratio,
            position: position.into(),
            orientation,
            clip_plane,
        }
    }

    // Gets a ray given UV coordinates
    pub fn get_ray(&self, x: f64, y: f64) -> ViewRay {
        // let angle_y = self.fov * (-y + 0.5);
        // let angle_x = self.fov * (x - 0.5) * self.aspect_ratio;

        let y = -y + 0.5;
        let x = (x - 0.5) * self.aspect_ratio;

        let direction = (x, y, self.clip_plane.0)
            .normalize()
            .apply_rotation(self.orientation);

        // let direction: Vec3 =
        //     Vec3::from((angle_x.rad().sin(), angle_y.rad().sin(), self.clip_plane.0))
        //         .apply_rotation(self.orientation)
        //         .normalize();

        ViewRay::new(self.position, direction, self.clip_plane)
    }
}
