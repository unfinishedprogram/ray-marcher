// Angle is always stored in radians

use std::{f32::consts::PI, ops::Mul};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Angle(f32);

impl Angle {
    const PI_180: f32 = PI / 180.0;
    const FULL_ROTATION: f32 = 2.0 * PI;

    #[inline]
    pub fn from_radians(rad: f32) -> Self {
        Self(rad % Self::FULL_ROTATION)
    }

    #[inline]
    pub fn from_degrees(deg: f32) -> Self {
        Self((deg * Self::PI_180) % Self::FULL_ROTATION)
    }

    #[inline]
    pub fn rad(self) -> f32 {
        self.0
    }

    #[inline]
    pub fn deg(self) -> f32 {
        self.0 / Self::PI_180
    }

    #[inline]
    pub fn rotate(self, other: Angle) -> Angle {
        Angle((self.0 + other.0) % Self::FULL_ROTATION)
    }
}

impl Mul<f32> for Angle {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Angle((self.0 * rhs) % Self::FULL_ROTATION)
    }
}
