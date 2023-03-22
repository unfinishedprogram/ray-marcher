use image::RgbaImage;
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::{
    light::Light,
    scene::Scene,
    vector3::{Vec3, Vector3},
};

fn get_ray_normal_buffer(scene: &Scene, (width, height): (usize, usize)) -> Vec<Vec3> {
    (0..(width * height))
        .into_par_iter()
        .map(|index| {
            let (x, y) = (index % width, index / width);
            let (x, y, width, height) = (x as f32, y as f32, width as f32, height as f32);
            scene.camera.get_ray_direction(x / width, y / height)
        })
        .collect()
}

fn get_surface_buffer(scene: &Scene, camera_rays: &Vec<Vec3>) -> Vec<Vec3> {
    camera_rays
        .into_par_iter()
        .map(|ray_dir| {
            let mut ray_pos = scene.camera.position;
            let mut step = 0;
            let mut len = 0.0;

            loop {
                let min_distance = scene.query_entities(ray_pos).distance;
                let epsilon = 0.00001;

                if min_distance <= epsilon || len > 1000.0 * 1000.0 || step == 512 {
                    break;
                }

                ray_pos.add_assign(ray_dir.multiply_scalar(min_distance));
                len += min_distance;
                step += 1;
            }
            ray_pos
        })
        .collect()
}

fn get_normal_buffer(scene: &Scene, surface: &Vec<Vec3>) -> Vec<Vec3> {
    surface
        .into_par_iter()
        .map(|&point| calculate_normal(point, scene))
        .collect()
}

fn get_light_buffer(scene: &Scene, surface: &Vec<Vec3>, normal: &Vec<Vec3>) -> Vec<Vec3> {
    surface
        .into_par_iter()
        .zip(normal.into_par_iter())
        .map(|(&point, &normal)| {
            let mut lighting = (0.0, 0.0, 0.0);
            for light in &scene.lights {
                let light_delta = light.position.sub(point);
                let light_distance = light_delta.magnitude_sq();
                let light_direction = light_delta.normalize();

                let angle = light_direction.dot(normal).max(0.0);
                let mut power = angle / light_distance;
                // Only do expensive shadow tracing if not trivially obscured
                if angle != 0.0 {
                    power *= trace_shadow_ray(scene, point, light);
                }

                lighting.add_assign(light.color.multiply_scalar(power));
            }
            lighting
        })
        .collect()
}

fn get_albedo_buffer(scene: &Scene, surface: &Vec<Vec3>) -> Vec<Vec3> {
    surface
        .into_par_iter()
        .map(|&point| scene.query_entities(point).entity.material.albedo)
        .collect()
}

pub fn render(scene: &Scene, (width, height): (usize, usize)) -> RgbaImage {
    let mut image = RgbaImage::new(width as u32, height as u32);

    let camera_rays = get_ray_normal_buffer(scene, (width, height));
    let surface = get_surface_buffer(scene, &camera_rays);
    let normals = get_normal_buffer(scene, &surface);
    let colors = get_light_buffer(scene, &surface, &normals);
    // let albedo = get_albedo_buffer(scene, &surface);

    let colors: Vec<_> = colors.into_par_iter().map(|c| c.rgb_u8()).collect();

    for (index, (r, g, b)) in colors.into_iter().enumerate() {
        let (x, y) = (index % width, index / width);
        image[(x as u32, y as u32)] = [r, g, b, 255].into();
    }

    image
}

pub fn trace_shadow_ray(scene: &Scene, point: Vec3, light: &Light) -> f32 {
    let max_t = light.position.sub(point).magnitude();
    let normal = light.position.sub(point).normalize();
    let mut res: f32 = 1.0;
    let mut t = 0.00001;

    for _ in 0..255 {
        let h = scene
            .query_entities(point.add(normal.multiply_scalar(t)))
            .distance;

        res = res.min(h / (light.radius * t));
        t += h.max(0.005);

        if res < -1.0 || t > max_t {
            break;
        }
    }

    let res = res.max(-1.0);

    0.25 * (1.0 + res) * (1.0 + res) * (2.0 - res)
}

fn calculate_normal(point: Vec3, scene: &Scene) -> Vec3 {
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
