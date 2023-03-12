use crate::{ray::ViewRay, vector3::Vector3};

pub struct Camera {
    pub fov: f32,
    pub position: Vector3,
    pub orientation: Vector3,
    horizontal_aspect: f32,
    vertical_aspect: f32,
    horizontal_fov: f32,
    vertical_fov: f32,
}

impl Camera {
    pub fn new(
        fov: f32,
        position: impl Into<Vector3>,
        orientation: impl Into<Vector3>,
        (width, height): (f32, f32),
    ) -> Self {
        let horizontal_aspect = width / width.min(height);
        let vertical_aspect = height / width.min(height);

        let vertical_fov = fov * vertical_aspect;
        let horizontal_fov = fov * horizontal_aspect;

        Camera {
            fov,
            position: position.into(),
            orientation: orientation.into(),
            horizontal_aspect,
            vertical_aspect,
            vertical_fov,
            horizontal_fov,
        }
    }

    // Gets a ray given UV coordinates
    pub fn get_ray(&self, x: f32, y: f32) -> ViewRay {
        let angle_y = (y - 0.5) * self.vertical_fov;
        let angle_x = (x - 0.5) * self.horizontal_fov;

        ViewRay::new(
            self.position,
            self.orientation
                .rotate((1.0, 0.0, 0.0).into(), angle_y)
                .rotate((0.0, 1.0, 0.0).into(), angle_x),
        )
    }
}
