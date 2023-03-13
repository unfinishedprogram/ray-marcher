// Angle is always stored in radians

use std::{f64::consts::PI, ops::Mul};

#[derive(Clone, Copy, Debug)]
pub struct Angle(f64);

impl Angle {
    const PI_180: f64 = PI / 180.0;
    const FULL_ROTATION: f64 = 2.0 * PI;

    #[inline]
    pub fn from_radians(rad: f64) -> Self {
        Self(rad % Self::FULL_ROTATION)
    }

    #[inline]
    pub fn from_degrees(deg: f64) -> Self {
        Self((deg * Self::PI_180) % Self::FULL_ROTATION)
    }

    #[inline]
    pub fn rad(self) -> f64 {
        self.0
    }

    #[inline]
    pub fn deg(self) -> f64 {
        self.0 / Self::PI_180
    }

    #[inline]
    pub fn rotate(self, other: Angle) -> Angle {
        Angle((self.0 + other.0) % Self::FULL_ROTATION)
    }
}

impl Mul<f64> for Angle {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self {
        Angle((self.0 * rhs) % Self::FULL_ROTATION)
    }
}
