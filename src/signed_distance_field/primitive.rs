use crate::vector3::{Vec3, Vector3};

use super::SignedDistance;

pub enum Primitive {
    Plane,
    Sphere(f64),
    Torus(f64, f64),
}

impl SignedDistance for Primitive {
    #[inline]
    fn distance_from(&self, point: Vec3) -> f64 {
        use Primitive::*;

        match self {
            Plane => point.1,
            Sphere(radius) => point.magnitude() - radius,
            Torus(inner, outer) => {
                let (px, py, pz) = point;
                let q = ((px * px + pz * pz).sqrt() - inner, py);
                (q.1 * q.1 + q.0 * q.0).sqrt() - outer
            }
        }
    }
}
