const STACK_SIZE = 8u;
const MAX_ENTITIES = 4u;
const MAX_LIGHTS = 8u;

const MAX_SIGNED_DISTANCE = 1000.0;
const MAX_MARCH_STEPS = 255u;

const CLIP_NEAR:f32 = 0.001;
const CLIP_FAR:f32 = 100.0;

const AO_STEPS:i32 = 4;
const AO_DISTANCE: f32 = 0.25;

const THRESHOLD:f32 = 0.001;

// Constants defining the Enum Index of primitives
const EMPTY = 0u;
const SPHERE = 1u;
const BOX = 2u;
const CYLINDER = 3u;

var<private> STACK_PTR:u32 = 0u;
var<private> STACK_ITEMS:array<vec2<u32>, STACK_SIZE>; 
var<private> ITEM_DISTANCE:array<f32, MAX_ENTITIES>;

struct Transform {
    translation: vec3<f32>,
    rotation: vec3<f32>,
    scale: vec3<f32>,
}

struct Camera {
    position: vec3<f32>,
    fov: f32,
    orientation: vec4<f32>,
    clip_near: f32,
    clip_far: f32,
}

// Base stack item mostly for padding
// All scene items must have a "render" property, 
// if it's value is 0 it is not rendered directly
struct SceneItem {
    item_type: u32,
    transform: Transform,
    padding: vec3<f32>,
}

struct Scene {
    entities: array<SceneItem, MAX_ENTITIES>,
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

// "Inherits" SceneItem
struct Sphere {
    item_type: u32,
    transform: Transform,
    radius: f32,
}

// "Inherits" SceneItem
struct Box {
    item_type: u32,
    transform: Transform,
    dimensions:vec3<f32>,
}

struct Cylinder {
    item_type: u32,
    transform: Transform,
    radius: f32,
    height: f32,
}

fn as_sphere(item:SceneItem) -> Sphere {
    var sphere:Sphere;
    sphere.radius = item.padding.x;
    return sphere;
}

fn as_box(item:SceneItem) -> Box {
    var box:Box;
    box.dimensions = item.padding;
    return box;
}

fn as_cylinder(item:SceneItem) -> Cylinder {
    var cylinder:Cylinder;
    cylinder.radius = item.padding.x;
    cylinder.height = item.padding.y;
    return cylinder;
}

fn pop() -> vec2<u32> {
    STACK_PTR -= 1u;
    return STACK_ITEMS[STACK_PTR];
}

fn push(index:u32) {
    STACK_ITEMS[STACK_PTR] = vec2<u32>(index, 0u);
    STACK_PTR += 1u;
}

fn push_raw(index:u32) {
    STACK_ITEMS[STACK_PTR] = vec2<u32>(index, 1u);
    STACK_PTR += 1u;
}

fn apply_rotation(v:vec3<f32>, rv:vec4<f32>) -> vec3<f32>{
    let r = rv * vec4<f32>(-1.0, -1.0, -1.0, 1.0);
    let s = r.w;
    let u = r.xyz;
    let a = u * (dot(u, v) * 2.0);
    let b = v * ((s * s) - dot(u, u));
    let c = cross(u, v) * (2.0 * s);
    return a + b + c;
}

fn apply_transform(point:vec3<f32>, transform:Transform) -> vec3<f32> {
    return point + transform.translation;
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
    var color = vec3<f32>(0.0);

    for (var i = 0u; i < MAX_LIGHTS; i++) {
        let light = lights.lights[i];

        if light.enabled == 0u { return color; }

        let delta = light.position - point;
        let dir = normalize(delta);

        let angle = max(dot(dir, normal), 0.0);
        let distance = length(delta);

        var power = (angle / distance);

        // Edge case optimization
        if power > 0.0 {
            power *= trace_shadow(point, light);
        }
        
        color += light.color * power;
    }
    
    return color;
}

fn trace_shadow(point:vec3<f32>, light: Light) -> f32 {
    var res: f32 = 1.0;
    let d = light.position - point;

    let max_t = length(d);
    let normal = normalize(d);
    var t = 0.01;

    var i = 0u;
    while(i < MAX_MARCH_STEPS) {
        let h = map(point + (normal * t));

        res = min(res, h / (light.radius * t));
        t += max(h, 0.01);

        if res < -1.0 || t > max_t {
            break;
        }
        i++;
    }

    res = max(res, -1.0);

    return 0.25 * (1.0 + res) * (1.0 + res) * (2.0 - res);
}

fn evaluate_sdf(item_index:u32, o_point: vec3<f32>) -> f32 {
    var signed_distance:f32 = MAX_SIGNED_DISTANCE;

    let item = scene.entities[item_index];

    if item.item_type == 0u {
        return signed_distance;
    }

    let point = o_point - item.transform.translation;

    switch item.item_type {
        case 1u: { // SPHERE
            let sphere = as_sphere(item);
            signed_distance = min(signed_distance, length(point) - sphere.radius);
        }
        case 2u: { // BOX
            let box = as_box(item);
            let q = abs(point) - box.dimensions;
            let distance = length(max(q, vec3<f32>(0.0))) + min(max(q.x, max(q.y, q.z)),0.0);
            signed_distance = min(signed_distance, distance);
        }
        case 3u: { // CYLINDER
            let cylinder = as_cylinder(item);
            let d = abs(vec2<f32>(length(point.xz),point.y)) - vec2<f32>(cylinder.radius,cylinder.height);
            let sd = min(max(d.x, d.y), 0.0) + length(max(d, vec2<f32>(0.0)));
            signed_distance = min(signed_distance, sd);
        }
        default: { return signed_distance; }
    }

    return signed_distance;
}

fn map(point:vec3<f32>) -> f32 {
    var min_dist = MAX_SIGNED_DISTANCE;
    for (var i = 0u; i < MAX_ENTITIES; i++) {
        min_dist = min(min_dist, evaluate_sdf(i, point));
    }

    return min_dist;
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

@group(0) @binding(0) 
var<uniform> dimensions: vec4<f32>;

@group(0) @binding(1) 
var<uniform> scene: Scene;

@group(0) @binding(2) 
var<uniform> lights: Lights;

@group(0) @binding(3) 
var<uniform> camera: Camera;

struct Input {
    @builtin(position) screen_cords: vec4<f32>,
};

@fragment
fn main(in: Input) -> @location(0) vec4<f32> {
    // Get the aspect ratio of the render target
    let aspect_ratio = dimensions.x / dimensions.y;
    // Normalize the pixel coordonates to -0.5 -0.5;
    let normalized = in.screen_cords / vec4<f32>(dimensions.xy, 1.0, 1.0) - vec4<f32>(0.5);

    let aspected = normalized * vec4<f32>(aspect_ratio, 1.0, 1.0, 1.0);

    let ray_dir = normalize(vec3(aspected.x, -aspected.y, 1.0));
    let ray_direction = apply_rotation(ray_dir, camera.orientation);
    let ray_origin = camera.position;
    var ray_length:f32 = camera.clip_near;
    var steps:u32 = 0u;

    loop {
        let point = ray_origin + (ray_direction * ray_length);
        let min_signed_distance = map(point);
        if ray_length > CLIP_FAR { break; }
        if min_signed_distance < THRESHOLD { break; }
        ray_length += min_signed_distance;
        steps++;
        if steps > MAX_MARCH_STEPS {break;}
    }
    
    let surface_point = ray_origin + ray_direction * ray_length;
    let surface_normal = surface_normal(surface_point);

    // let occlusion = ambient_occlusion(surface_point, surface_normal);

    var color = vec3<f32>(0.0);

    if length(surface_point) < 100.0 {
        color = direct_lighting(surface_point, surface_normal);
    } 

    color = sqrt(color);
    
    
    // return vec4<f32>(surface_normal * 0.5 + vec3<f32>(0.5), 1.0);
    return vec4<f32>(color, 1.0);
}
