use image::RgbaImage;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::{
    material::Material,
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

pub fn calculate_light(point: Vec3, normal: Vec3, scene: &Scene) -> Vec3 {
    let mut lighting = (0.0, 0.0, 0.0);

    for light in &scene.lights {
        let light_delta = light.position.sub(point);
        let light_distance = light_delta.magnitude_sq();
        let light_direction = light_delta.normalize();

        let angle = light_direction.dot(normal).max(0.0);
        let power = angle / light_distance;

        lighting.add_assign(light.color.multiply_scalar(power));
    }

    lighting
}

pub fn calculate_normal(point: Vec3, scene: &Scene) -> Vec3 {
    let small_step_x = (0.000001, 0.0, 0.0);
    let small_step_y = (0.0, 0.000001, 0.0);
    let small_step_z = (0.0, 0.0, 0.000001);
    let entity = scene.query_entities(point).entity;

    (
        entity.distance(point.add(small_step_x)) - entity.distance(point.sub(small_step_x)),
        entity.distance(point.add(small_step_y)) - entity.distance(point.sub(small_step_y)),
        entity.distance(point.add(small_step_z)) - entity.distance(point.sub(small_step_z)),
    )
        .normalize()
}

pub fn march(mut ray: ViewRay, scene: &Scene) -> ViewRay {
    loop {
        let ray_length = ray.len_sq();

        let steps = ray.steps as u8;
        let distance = scene.query_entities(ray.position).distance;

        // Hit object
        if distance <= 0.000001 || ray_length > 1000.0 * 1000.0 || steps == 255 {
            break;
        }

        ray.step(distance);
    }

    let surface_normal = calculate_normal(ray.position, scene);
    let surface_material = &scene.query_entities(ray.position).entity.material;

    ray.color = match surface_material {
        Material::Basic(color) => {
            calculate_light(ray.position, surface_normal, scene).channel_multiply(*color)
        }
    };

    ray
}
