pub fn smooth_step(min: f64, max: f64, x: f64) -> f64 {
    x * x * (3.0 - 2.0 * x)
}

pub fn soft_clamp(x: f64, min: f64, max: f64) -> f64 {
    smooth_step(
        0.0,
        1.0,
        (2.0 / 3.0) * (x - min) / (max - min) + (1.0 / 6.0),
    ) * (max - min)
        + min
}
