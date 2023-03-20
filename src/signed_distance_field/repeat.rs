use crate::vector3::Vec3;

use super::SignedDistance;

pub struct Repeated<T: SignedDistance + Sized>(pub Box<T>, pub f64);

impl<T: SignedDistance + Sized> SignedDistance for Repeated<T> {
    fn distance_from(&self, point: Vec3) -> f64 {
        let (x, y, z) = point;
        let point = (
            x.abs() % self.1 - self.1 / 2.0,
            y.abs() % self.1 - self.1 / 2.0,
            z.abs() % self.1 - self.1 / 2.0,
        );
        self.0.distance_from(point)
    }
}
