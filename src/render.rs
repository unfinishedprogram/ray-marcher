use image::RgbaImage;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::{primitives::SignedDistance, ray::ViewRay, scene::Scene, vector3::Vector3};

pub fn render(scene: &Scene, (width, height): (u32, u32)) -> RgbaImage {
    let mut image = RgbaImage::new(width, height);

    let colors: Vec<(u8, u8, u8)> = (0..(width as usize * height as usize))
        .into_par_iter()
        .map(|index| {
            let (x, y) = (index % width as usize, index / width as usize);

            let ray = scene
                .camera
                .get_ray(x as f32 / width as f32, y as f32 / height as f32);

            march(ray, scene).color
        })
        .collect();

    for (index, (r, g, b)) in colors.into_iter().enumerate() {
        let (x, y) = (index % width as usize, index / width as usize);
        image[(x as u32, y as u32)] = [r, g, b, 255].into();
    }

    image
}

pub fn signed_distance(point: Vector3, scene: &Scene) -> f32 {
    scene
        .entities
        .iter()
        .map(|entity| entity.distance_from(point))
        .reduce(f32::min)
        .unwrap_or_default()
}

pub fn calculate_normal(point: Vector3, scene: &Scene) -> Vector3 {
    let small_step_x = (0.1, 0.0, 0.0).into();
    let small_step_y = (0.0, 0.1, 0.0).into();
    let small_step_z = (0.0, 0.0, 0.1).into();

    Vector3 {
        x: signed_distance(point + small_step_x, scene)
            - signed_distance(point - small_step_x, scene),
        y: signed_distance(point + small_step_y, scene)
            - signed_distance(point - small_step_y, scene),
        z: signed_distance(point + small_step_z, scene)
            - signed_distance(point - small_step_z, scene),
    }
}

pub fn march(mut ray: ViewRay, scene: &Scene) -> ViewRay {
    loop {
        let ray_length = ray.len_sq();

        let steps = ray.steps as u8;
        let signed_distance = signed_distance(ray.position, scene);

        // Clip plane
        if ray_length > 1000.0 * 1000.0 {
            ray.color = (255, steps - 255, steps - 255);
            break;
        }

        // Hit object
        if signed_distance <= 0.0001 {
            let Vector3 { x: r, y: g, z: b } =
                calculate_normal(ray.position, scene) + (0.5, 0.5, 0.5).into();

            ray.color = ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8);

            break;
        }

        // Timeout
        if steps == 255 {
            ray.color = (steps - 255, steps - 255, 255);
            break;
        }

        ray.step(signed_distance);
    }

    ray
}
