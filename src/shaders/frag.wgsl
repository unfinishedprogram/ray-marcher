const STACK_SIZE = 8u;
const MAX_ENTITIES = 8u;

const MAX_SIGNED_DISTANCE = 10000.0;
const MAX_MARCH_STEPS = 255u;

const CLIP_NEAR:f32 = 0.001;
const CLIP_FAR:f32 = 10000.0;

const THRESHOLD:f32 = 0.001;

const MAX_RECUR_DEPTH = 8;

// Constants defining the Enum Index of primitives
const EMPTY = 0u;
const SPHERE = 1u;
const TRANSLATE = 2u;
const BOX = 3u;
const ROTATE = 4u;

var<private> STACK_PTR:u32 = 0u;
var<private> STACK_ITEMS:array<SceneItem, STACK_SIZE>; 

// Base stack item mostly for padding
// All scene items must have a "render" property, 
// if it's value is 0 it is not rendered directly
struct SceneItem {
    item_type: u32,
    render: u32,
    pad2: u32,
    pad3: u32,
    pad4: u32,
    pad5: u32,
    pad6: u32,
    pad7: u32,
}

struct Scene {
    entities: array<SceneItem, MAX_ENTITIES>,
}

// "Inherits" SceneItem
struct Sphere {
    item_type: u32,
    render: u32, 
    radius: f32,
}

// "Inherits" SceneItem
struct Translate {
    item_type: u32,
    render: u32, 
    pointer: u32, 
    v: vec3<f32>,
}

// "Inherits" SceneItem
struct Box {
    item_type: u32,
    render: u32, 
    dimensions:vec3<f32>,
}

struct Rotate {
    item_type: u32,
    render: u32, 
    pointer: u32, 
    rotation:vec4<f32>,
}

struct ViewRay {
    position: vec3<f32>,
    distance: f32, // Distance along ray, 
}

fn as_sphere(item:SceneItem) -> Sphere {
    var sphere:Sphere;
    sphere.radius = bitcast<f32>(item.pad2);
    sphere.item_type = SPHERE;
    return sphere;
}

fn as_translate(item:SceneItem) -> Translate {
    var translate:Translate;
    translate.item_type = TRANSLATE;
    
    translate.pointer = item.pad2;
    let x = bitcast<f32>(item.pad3);
    let y = bitcast<f32>(item.pad4);
    let z = bitcast<f32>(item.pad5);

    translate.v = vec3<f32>(x, y, z);
    
    return translate;
}

fn as_box(item:SceneItem) -> Box {
    var box:Box;
    box.item_type = BOX;

    let x = bitcast<f32>(item.pad2);
    let y = bitcast<f32>(item.pad3);
    let z = bitcast<f32>(item.pad4);

    box.dimensions = vec3<f32>(x, y, z);
    return box;
}

fn as_rotate(item:SceneItem) -> Rotate {
    var rotate:Rotate;
    rotate.item_type = ROTATE;

    rotate.pointer = item.pad2;
    let x = bitcast<f32>(item.pad3);
    let y = bitcast<f32>(item.pad4);
    let z = bitcast<f32>(item.pad5);
    let w = bitcast<f32>(item.pad6);

    rotate.rotation = vec4<f32>(x, y, z, w);
    return rotate;
}

fn pop() -> SceneItem {
    STACK_PTR -= 1u;
    return STACK_ITEMS[STACK_PTR];
}

fn push(item:SceneItem) {
    STACK_ITEMS[STACK_PTR] = item;
    STACK_PTR += 1u;
}


fn applyRotation(v:vec3<f32>, rv:vec4<f32>) -> vec3<f32>{
    let r = rv * vec4<f32>(-1.0, -1.0, -1.0, 1.0);
    let s = r.w;
    let u = vec3<f32>(r.x, r.y, r.z);
    let a = u * (dot(u, v) * 2.0);
    let b = v * ((s * s) - dot(u, u));
    let c = cross(u, v) * (2.0 * s);
    return a + b + c;
}


fn evaluate_sdf(index: u32, point: vec3<f32>) -> f32 {
    var signed_distance:f32 = MAX_SIGNED_DISTANCE;
    var transformed_point:vec3<f32> = point;
    push(scene.entities[index]);

    var iters = 0;
    // While items remain on the stack, evaluate them
    while iters < MAX_RECUR_DEPTH && STACK_PTR > 0u {
        iters += 1;
        let item = pop();
        let item_type = item.item_type;

        if item_type == TRANSLATE {
            var translate = as_translate(item);
            transformed_point -= translate.v;
            push(scene.entities[translate.pointer]);
        }

        if item_type == ROTATE {
            var rotate = as_rotate(item);
            transformed_point = applyRotation(transformed_point, rotate.rotation);
            push(scene.entities[rotate.pointer]);
        }

        if item_type == SPHERE {
            let sphere = as_sphere(item);
            signed_distance = min(signed_distance, length(transformed_point) - sphere.radius);
        }

        if item_type == BOX {
            let box = as_box(item);
            let q = abs(transformed_point) - box.dimensions;
            let distance = length(max(q, vec3<f32>(0.0))) + min(max(q.x, max(q.y, q.z)),0.0);
            signed_distance = min(signed_distance, distance);
        }
    }

    return signed_distance;
}

fn map(point:vec3<f32>) -> f32 {
    var min_dist = MAX_SIGNED_DISTANCE;

    for (var i = 0u; i < MAX_ENTITIES; i++) {
        if scene.entities[i].render != 0u {
            min_dist = min(min_dist, evaluate_sdf(i, point));
        }
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


struct Input {
    @builtin(position) screen_cords: vec4<f32>,
};

@fragment
fn main(in: Input) -> @location(0) vec4<f32> {
    // Get the aspect ratio of the render target
    let aspect_ratio = dimensions.x / dimensions.y;
    // Normalize the pixel coordonates to -0.5 - 0.5;
    let normalized = in.screen_cords / vec4<f32>(dimensions.x, dimensions.y, 1.0, 1.0) - vec4<f32>(0.5);

    let aspected = (normalized ) * vec4<f32>(aspect_ratio, 1.0, 1.0, 1.0);

    let ray_direction = normalize(vec3(aspected.x, -aspected.y, 1.0));
    let ray_origin = vec3<f32>(0.0, 0.0, -10.0);

    var ray_length:f32 = CLIP_NEAR;

    var steps:u32 = 0u;
    while(steps < MAX_MARCH_STEPS) {
        steps++;
        let point = ray_origin + (ray_direction * ray_length);
        let min_signed_distance = map(point);
        if ray_length >= CLIP_FAR { break; }
        if min_signed_distance <= THRESHOLD { break; }
        ray_length += min_signed_distance;
    }
    
    let surface_point = ray_origin + ray_direction * ray_length;
    let surface_normal = surface_normal(surface_point);

    let color = f32(steps) / 255.0;
    return vec4<f32>(surface_normal * 0.5 + vec3<f32>(0.5), 1.0);
}
