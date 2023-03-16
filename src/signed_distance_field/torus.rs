use super::SignedDistance;

pub struct Torus(pub f64, pub f64);

impl SignedDistance for Torus {
    fn distance_from(&self, position: crate::vector3::Vec3) -> f64 {
        let (px, py, pz) = position;

        let inner = self.0;
        let outer = self.1;

        let q = ((px * px + pz * pz).sqrt() - inner, py);
        (q.1 * q.1 + q.0 * q.0).sqrt() - outer
    }
}
