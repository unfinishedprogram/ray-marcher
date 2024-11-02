const MAX_LIGHTS = 8u;

const MAX_SIGNED_DISTANCE = 10000.0;
const MAX_MARCH_STEPS = 255u;

const CLIP_NEAR:f32 = 0.001;
const CLIP_FAR:f32 = 100.0;

const AO_STEPS:i32 = 4;
const AO_DISTANCE: f32 = 0.25;

const THRESHOLD:f32 = 0.00001;

const MAX_RECUR_DEPTH = 8;

@group(0) @binding(0) 
var<uniform> dimensions: vec4<f32>;

@group(0) @binding(1) 
var<uniform> scene: Scene;

@group(0) @binding(2)
var<uniform> lights: Lights;

@group(0) @binding(3) 
var<uniform> camera: Camera;

@group(0) @binding(4)
var<uniform> cuboids: array<Cuboid, 2>;

@group(0) @binding(5)
var<uniform> spheres: array<Sphere, 1>;


struct Camera {
    position: vec3<f32>,
    fov: f32,
    orientation: vec4<f32>,
    clip_near: f32,
    clip_far: f32,
}

struct Scene {
    sphere_count: u32,
    cuboid_count: u32,
}

struct Light {
    position: vec3<f32>,
    radius: f32,
    color: vec3<f32>,
    enabled: u32,
}

struct Lights {
    lights: array<Light, MAX_LIGHTS>
}

struct Sphere {
    transform: mat4x4<f32>,
    radius: f32,
}

struct Cuboid {
    transform: mat4x4<f32>,
    dimensions: vec3<f32>,
}

struct ViewRay {
    position: vec3<f32>,
    distance: f32, // Distance along ray, 
}

fn applyRotation(v:vec3<f32>, rv:vec4<f32>) -> vec3<f32>{
    let r = rv * vec4<f32>(-1.0, -1.0, -1.0, 1.0);
    let s = r.w;
    let u = r.xyz;
    let a = u * (dot(u, v) * 2.0);
    let b = v * ((s * s) - dot(u, u));
    let c = cross(u, v) * (2.0 * s);
    return a + b + c;
}

fn ambient_occlusion(point:vec3<f32>, normal:vec3<f32>) -> f32 {
    var occlusion = 0.0;
    var i = 1;
    while i <= AO_STEPS {
        i++;
        let distance = f32(AO_DISTANCE) / f32(AO_STEPS) * f32(i);
        let d = map(point + (normal * distance));
        occlusion += max(-(d - distance), 0.0);
    }
    return occlusion/f32(AO_DISTANCE * f32(AO_STEPS) * 4.0);
}

fn direct_lighting(point:vec3<f32>, normal:vec3<f32>) -> vec3<f32> {
    var light = vec3<f32>(0.0);

    for (var i = 0u; i < MAX_LIGHTS; i++) {
        let l = lights.lights[i];

        if l.enabled == 0u { return light; }

        let delta = l.position - point;
        let dir = normalize(delta);

        let distance = length(delta);
        let angle = dot(dir, normal);

        var power = (angle / distance);

        // Edge case optimization
        if power > 0.0 {
            power *= trace_shadow(point, l);
        }

        light += l.color * max(0.0, power);
    }
    
    return light;
}

fn trace_shadow(point:vec3<f32>, light: Light) -> f32 {
    var res: f32 = 1.0;

    let max_t = length(light.position - point);
    let normal = normalize(light.position - point);
    var t = 0.01;

    for(var i = 0u; i < MAX_MARCH_STEPS; i++){

        let h = map(point + (normal * t));

        res = min(res, h / (light.radius * t));
        t += max(h, 0.01);

        if res < -1.0 || t > max_t {
            break;
        }
    }

    res = max(res, -1.0);

    return 0.25 * (1.0 + res) * (1.0 + res) * (2.0 - res);
}

fn evaluate_sdf_sphere(sphere: Sphere, point: vec3<f32>) -> f32 {
    let transformed_point = sphere.transform * vec4f(point, 1.0);
    return length(transformed_point.xyz) - sphere.radius;
}

fn evaluate_sdf_cuboid(cuboid: Cuboid, point: vec3<f32>) -> f32 {
    let transformed_point =  cuboid.transform * vec4f(point, 1.0);
    let q = abs(transformed_point.xyz) - cuboid.dimensions;
    return length(max(q, vec3<f32>(0.0))) + min(max(q.x, max(q.y, q.z)), 0.0);
}

fn evaluate_sdf(point: vec3<f32>) -> f32 {
    var min_dist = MAX_SIGNED_DISTANCE;

    for (var i = 0u; i < scene.sphere_count; i++) {
        min_dist = min(min_dist, evaluate_sdf_sphere(spheres[i], point));
    }

    for (var i = 0u; i < scene.cuboid_count; i++) {
        min_dist = min(min_dist, evaluate_sdf_cuboid(cuboids[i], point));
    }

    return min_dist;
}

fn map(point:vec3<f32>) -> f32 {
    return evaluate_sdf(point);
}

fn surface_normal(point:vec3<f32>) -> vec3<f32> {
    let step_x = vec3<f32>(0.0001, 0.0, 0.0);
    let step_y = vec3<f32>(0.0, 0.0001, 0.0);
    let step_z = vec3<f32>(0.0, 0.0, 0.0001);

    return normalize(vec3<f32>(
        map(point + step_x) - map(point - step_x),
        map(point + step_y) - map(point - step_y),
        map(point + step_z) - map(point - step_z),
    ));
}


struct Input {
    @builtin(position) screen_cords: vec4<f32>,
};

fn surface_point(ray_origin: vec3f, ray_direction: vec3f) -> vec3f {
    var ray_length:f32 = CLIP_NEAR;
    var steps:u32 = 0u;

    loop {
        let point = ray_origin + (ray_direction * ray_length);
        let min_signed_distance = map(point);
        if ray_length > CLIP_FAR { break;}
        if min_signed_distance < THRESHOLD { break;  }
        ray_length += min_signed_distance;
        steps++;
        if steps > MAX_MARCH_STEPS {break;}
    }

    return ray_origin + ray_direction * ray_length;
}


@fragment
fn main(in: Input) -> @location(0) vec4<f32> {
    // Get the aspect ratio of the render target
    let aspect_ratio = dimensions.x / dimensions.y;
    // Normalize the pixel coordonates to -0.5 - 0.5;
    let normalized = in.screen_cords / vec4<f32>(dimensions.x, dimensions.y, 1.0, 1.0) - vec4<f32>(0.5);

    let aspected = normalized * vec4<f32>(aspect_ratio, 1.0, 1.0, 1.0);

    let ray_dir = normalize(vec3(aspected.x, -aspected.y, 1.0));
    let ray_direction = applyRotation(ray_dir, camera.orientation);
    let ray_origin = camera.position;

    let surface_point = surface_point(ray_origin, ray_direction);

    let surface_normal = surface_normal(surface_point);

    let reflected_ray = reflect(ray_direction, surface_normal);

    let reflected_point = surface_point(surface_point, reflected_ray);

    let occlusion = ambient_occlusion(surface_point, surface_normal);

    let reflected_surface_normal = surface_normal(reflected_point);

    let reflected_occlusion = ambient_occlusion(reflected_point, reflected_surface_normal);

    var color = vec3<f32>(0.0);
    var reflected_color = vec3<f32>(0.0);

    if length(surface_point) < 100.0 {
        color = direct_lighting(surface_point, surface_normal) * max(0.0, 1.0 - occlusion);
    }

    if length(reflected_point - surface_point) < 100.0 {
        reflected_color = direct_lighting(reflected_point, reflected_surface_normal) * max(0.0, 1.0 - reflected_occlusion);
    }

    // Apply fresnel effect
    let ratio = pow(abs(dot(ray_direction, surface_normal)), 0.5);
    
    color = mix(reflected_color, color, ratio);

    color = sqrt(color);

    return vec4<f32>(color, 1.0);
}
