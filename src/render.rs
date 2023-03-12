use image::RgbaImage;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::{ray::ViewRay, scene::Scene};

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

pub fn march(mut ray: ViewRay, scene: &Scene) -> ViewRay {
    loop {
        let ray_length = ray.len_sq();

        let signed_distance = scene
            .entities
            .iter()
            .map(|entity| entity.distance_from(ray.position))
            .reduce(f32::min)
            .unwrap_or_default();

        let steps = ray.steps as u8;

        // Clip plane
        if ray_length > 1000.0 * 1000.0 {
            ray.color = (255, steps - 255, steps - 255);
            break;
        }

        // Hit object
        if signed_distance <= 0.0001 {
            ray.color = (steps - 255, 255, steps - 255);
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
