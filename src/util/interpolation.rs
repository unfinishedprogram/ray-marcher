pub fn smooth_step(min: f32, max: f32, x: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        let x = (x - min) / (max - min);
        x * x * (3.0 - 2.0 * x)
    }
}

pub fn soft_clamp(x: f32, min: f32, max: f32) -> f32 {
    smooth_step(
        0.0,
        1.0,
        (2.0 / 3.0) * (x - min) / (max - min) + (1.0 / 6.0),
    ) * (max - min)
        + min
}
