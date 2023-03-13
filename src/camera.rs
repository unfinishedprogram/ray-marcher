use crate::{angle::Angle, ray::ViewRay, vector3::Vector3};

pub struct Camera {
    pub fov: Angle,
    pub position: Vector3,
    pub orientation: Vector3,
    pub clip_plane: (f64, f64),
    // Vertical aspect is baseline: 16:9 = ~1.77
    pub aspect_ratio: f64,
}

impl Camera {
    pub fn new(
        fov: Angle,
        aspect_ratio: f64,
        position: impl Into<Vector3>,
        orientation: impl Into<Vector3>,
        clip_plane: (f64, f64),
    ) -> Self {
        Camera {
            fov,
            aspect_ratio,
            position: position.into(),
            orientation: orientation.into(),
            clip_plane,
        }
    }

    // Gets a ray given UV coordinates
    pub fn get_ray(&self, x: f64, y: f64) -> ViewRay {
        let angle_y = self.fov * (y - 0.5);
        let angle_x = self.fov * (x - 0.5) * self.aspect_ratio;

        ViewRay::new(
            self.position,
            self.orientation
                .rotate((1.0, 0.0, 0.0).into(), angle_y)
                .rotate((0.0, 1.0, 0.0).into(), angle_x),
            self.clip_plane,
        )
    }
}
