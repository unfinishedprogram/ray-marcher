use image::RgbaImage;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::{ray::ViewRay, scene::Scene, vector3::Vector3};

pub fn render(scene: &Scene, (width, height): (u32, u32)) -> RgbaImage {
    let mut image = RgbaImage::new(width, height);

    let colors: Vec<(u8, u8, u8)> = (0..(width as usize * height as usize))
        .into_par_iter()
        .map(|index| {
            let (x, y) = (index % width as usize, index / width as usize);

            let ray = scene
                .camera
                .get_ray(x as f64 / width as f64, y as f64 / height as f64);

            march(ray, scene).color
        })
        .collect();

    for (index, (r, g, b)) in colors.into_iter().enumerate() {
        let (x, y) = (index % width as usize, index / width as usize);
        image[(x as u32, y as u32)] = [r, g, b, 255].into();
    }

    image
}

pub fn signed_distance(point: Vector3, scene: &Scene) -> f64 {
    scene
        .entities
        .iter()
        .map(|entity| entity.distance_from(point))
        .reduce(f64::min)
        .unwrap_or_default()
}

pub fn calculate_light(point: Vector3, normal: Vector3, scene: &Scene) -> f64 {
    let mut lighting = 0.0;

    for light in &scene.lights {
        let light_direction = (light.position - point).normalize();
        lighting += light_direction.dot(normal);
    }

    lighting
}
pub fn calculate_normal(point: Vector3, scene: &Scene) -> Vector3 {
    let small_step_x = (0.000001, 0.0, 0.0).into();
    let small_step_y = (0.0, 0.000001, 0.0).into();
    let small_step_z = (0.0, 0.0, 0.000001).into();

    Vector3 {
        x: signed_distance(point + small_step_x, scene)
            - signed_distance(point - small_step_x, scene),
        y: signed_distance(point + small_step_y, scene)
            - signed_distance(point - small_step_y, scene),
        z: signed_distance(point + small_step_z, scene)
            - signed_distance(point - small_step_z, scene),
    }
    .normalize()
}

pub fn march(mut ray: ViewRay, scene: &Scene) -> ViewRay {
    loop {
        let ray_length = ray.len_sq();

        let steps = ray.steps as u8;
        let signed_distance = signed_distance(ray.position, scene);

        // Hit object
        if signed_distance <= 0.000001 || ray_length > 1000.0 * 1000.0 || steps == 255 {
            break;
        }

        ray.step(signed_distance);
    }
    let surface_normal = calculate_normal(ray.position, scene) + (0.5, 0.5, 0.5).into();

    let intensity = calculate_light(ray.position, surface_normal, scene);

    ray.color = (
        (intensity * 255.0) as u8,
        (intensity * 255.0) as u8,
        (intensity * 255.0) as u8,
    );

    ray
}
