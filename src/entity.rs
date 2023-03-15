use crate::{material::Material, signed_distance_field::SignedDistance};

pub struct Entity {
    sdf: Box<impl SignedDistance>,
    material: Material,
}
