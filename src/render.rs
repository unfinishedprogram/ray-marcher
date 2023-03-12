use image::RgbaImage;

use crate::{ray::ViewRay, scene::Scene};

pub fn render(scene: &Scene, (width, height): (u32, u32)) -> RgbaImage {
    let mut image = RgbaImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let ray = scene
                .camera
                .get_ray(x as f32 / width as f32, y as f32 / height as f32);

            let (r, g, b) = march(ray, scene).color;

            image[(x, y)] = [r, g, b, 255].into();
        }
    }

    image
}

pub fn march(mut ray: ViewRay, scene: &Scene) -> ViewRay {
    loop {
        let ray_length = (ray.origin - ray.position).magnitude();

        let signed_distance = scene
            .entities
            .iter()
            .map(|entity| entity.distance_from(ray.position))
            .reduce(f32::min)
            .unwrap_or_default();

        // Clip plane
        if ray_length > 1000.0 {
            ray.color = (255, ray.steps as u8 - 255, ray.steps as u8 - 255);
            break;
        }

        // Hit object
        if signed_distance <= 0.0001 {
            ray.color = (ray.steps as u8 - 255, 255, ray.steps as u8 - 255);
            break;
        }

        // Timeout
        if ray.steps >= 255 {
            ray.color = (ray.steps as u8 - 255, ray.steps as u8 - 255, 255);
            break;
        }

        ray.step(signed_distance);
    }

    ray
}
