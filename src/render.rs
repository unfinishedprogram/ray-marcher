use image::RgbaImage;
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::{
    ray::ViewRay,
    scene::Scene,
    vector3::{Vec3, Vector3},
};

pub fn render(scene: &Scene, (width, height): (usize, usize)) -> RgbaImage {
    let mut image = RgbaImage::new(width as u32, height as u32);

    let colors: Vec<(u8, u8, u8)> = (0..(width * height))
        .into_par_iter()
        .map(|index| {
            let (x, y) = (index % width, index / width);
            scene
                .camera
                .get_ray(x as f64 / width as f64, y as f64 / height as f64)
        })
        .map(|ray| march(ray, scene).color.rgb_u8())
        .collect();

    for (index, (r, g, b)) in colors.into_iter().enumerate() {
        let (x, y) = (index % width, index / width);
        image[(x as u32, y as u32)] = [r, g, b, 255].into();
    }

    image
}

pub fn signed_distance(point: Vec3, scene: &Scene) -> f64 {
    scene
        .entities
        .iter()
        .map(|entity| entity.distance_from(point))
        .reduce(f64::min)
        .unwrap_or_default()
}

pub fn calculate_light(point: Vec3, normal: Vec3, scene: &Scene) -> Vec3 {
    let mut lighting = (0.0, 0.0, 0.0);

    for light in &scene.lights {
        let light_delta = light.position.sub(point);
        let light_distance = light_delta.magnitude_sq();
        let light_direction = light_delta.normalize();

        let angle = light_direction.dot(normal);
        let power = angle / light_distance;

        lighting.add_assign(light.color.multiply_scalar(power));
    }

    lighting
}
pub fn calculate_normal(point: Vec3, scene: &Scene) -> Vec3 {
    let small_step_x = (0.000001, 0.0, 0.0);
    let small_step_y = (0.0, 0.000001, 0.0);
    let small_step_z = (0.0, 0.0, 0.000001);

    (
        signed_distance(point.add(small_step_x), scene)
            - signed_distance(point.sub(small_step_x), scene),
        signed_distance(point.add(small_step_y), scene)
            - signed_distance(point.sub(small_step_y), scene),
        signed_distance(point.add(small_step_z), scene)
            - signed_distance(point.sub(small_step_z), scene),
    )
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

    let surface_normal = calculate_normal(ray.position, scene);
    ray.color = calculate_light(ray.position, surface_normal, scene);

    ray
}
