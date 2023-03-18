use image::RgbaImage;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::{
    light::Light,
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

pub fn trace_shadow_ray(point: Vec3, scene: &Scene, light: &Light) -> f64 {
    let light_distance_sq = light.position.sub(point).magnitude_sq();
    let normal = calculate_normal(point, scene);

    let cam_dist = point.sub(scene.camera.position).magnitude_sq();
    let epsilon = cam_dist.max(0.1) / 100000.0;

    let mut ray = ViewRay::new(
        point.add(normal.multiply_scalar(epsilon)),
        light.position.sub(point).normalize(),
        (0.0, 0.0),
    );

    loop {
        let ray_length = ray.len_sq();

        let steps = ray.steps as u8;
        let distance = scene.query_entities(ray.position).distance;

        // Hit object
        if distance <= epsilon || ray_length > 1000.0 * 1000.0 || steps == 50 {
            return 0.0;
        } else if ray_length >= light_distance_sq {
            return 1.0;
        }

        ray.step(distance);
    }
}

pub fn calculate_light(point: Vec3, normal: Vec3, scene: &Scene) -> Vec3 {
    let mut lighting = (0.0, 0.0, 0.0);
    for light in &scene.lights {
        let light_delta = light.position.sub(point);
        let light_distance = light_delta.magnitude_sq();
        let light_direction = light_delta.normalize();

        let angle = light_direction.dot(normal).max(0.0);
        let mut power = angle / light_distance;

        // Only do expensive shadow tracing if not trivially obscured
        if angle != 0.0 {
            power *= trace_shadow_ray(point, scene, light);
        }

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
        let epsilon = ray_length.max(0.1) / 100000.0;
        // Hit object
        if distance <= epsilon || ray_length > 1000.0 * 1000.0 || steps == 255 {
            break;
        }

        ray.step(distance);
    }

    let surface_normal = calculate_normal(ray.position, scene);
    let surface_material = &scene.query_entities(ray.position).entity.get_material();

    ray.color = match surface_material {
        Material::Basic(color) => {
            calculate_light(ray.position, surface_normal, scene).channel_multiply(*color)
        }
    };

    ray
}
