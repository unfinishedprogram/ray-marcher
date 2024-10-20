use winit::keyboard::Key;

use crate::vector3::{Vec3, Vector3};

fn bool_tuple_to_vec((a, b, c): (bool, bool, bool)) -> Vec3 {
    let x = if a { 1.0 } else { 0.0 };
    let y = if b { 1.0 } else { 0.0 };
    let z = if c { 1.0 } else { 0.0 };

    (x, y, z)
}

pub trait KeyboardProvider {
    fn is_down(&self, key: Key) -> bool;
    // Returns a vector representing the players directed movement in 3D
    fn movement(&self) -> Vec3 {
        let positive = bool_tuple_to_vec((
            self.is_down(Key::Character("d".into())),
            self.is_down(Key::Character("q".into())),
            self.is_down(Key::Character("s".into())),
        ));

        let negative = bool_tuple_to_vec((
            self.is_down(Key::Character("a".into())),
            self.is_down(Key::Character("e".into())),
            self.is_down(Key::Character("w".into())),
        ))
        .multiply_scalar(-1.0);

        Vector3::add(positive, negative).normalize()
    }
}
