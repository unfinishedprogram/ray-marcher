use crate::vector3::{Vec3, Vector3};

use super::SignedDistance;

pub enum Primitive {
    Plane,
    Sphere(f64),
    Torus(f64, f64),
    Box(Vec3),
}

impl SignedDistance for Primitive {
    #[inline]
    fn distance_from(&self, point: Vec3) -> f64 {
        use Primitive::*;

        match self {
            Box(b) => {
                let (x, y, z) = point;
                let q = (x.abs(), y.abs(), z.abs()).sub(*b);

                let q_len = (q.0.max(0.0), q.1.max(0.0), q.2.max(0.0)).magnitude();

                q_len + q.0.max(q.1.max(q.2)).min(0.0)
            }
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
