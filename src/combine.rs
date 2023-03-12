use crate::primitives::SignedDistance;

pub enum Combine<A: SignedDistance, B: SignedDistance> {
    Subtract(Box<A>, Box<B>),
    Intersect(Box<A>, Box<B>),
    Union(Box<A>, Box<B>),
}

impl<A: SignedDistance, B: SignedDistance> SignedDistance for Combine<A, B> {
    fn distance_from(&self, position: crate::vector3::Vector3) -> f64 {
        use Combine::*;
        let (a, b) = match self {
            Subtract(a, b) | Intersect(a, b) | Union(a, b) => {
                (a.distance_from(position), b.distance_from(position))
            }
        };

        match self {
            Subtract(_, _) => a.max(-b),
            Intersect(_, _) => a.max(b),
            Union(_, _) => a.min(b),
        }
    }
}

pub fn subtract<A: SignedDistance, B: SignedDistance>(a: A, b: B) -> Combine<A, B> {
    Combine::Subtract(Box::new(a), Box::new(b))
}

pub fn intersect<A: SignedDistance, B: SignedDistance>(a: A, b: B) -> Combine<A, B> {
    Combine::Intersect(Box::new(a), Box::new(b))
}
pub fn union<A: SignedDistance, B: SignedDistance>(a: A, b: B) -> Combine<A, B> {
    Combine::Union(Box::new(a), Box::new(b))
}
