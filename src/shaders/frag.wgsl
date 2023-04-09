const STACK_SIZE = 4u;
const MAX_ENTITIES = 4u;

const MAX_SIGNED_DISTANCE = 10000.0;
const MAX_MARCH_STEPS = 255u;

const CLIP_NEAR:f32 = 0.001;
const CLIP_FAR:f32 = 10000.0;

const RECUR_DEPTH = 8;

// Constants defining the Enum Index of primitives
const SPHERE = 0u;
const TRANSLATE = 1u;

var<private> STACK_PTR:u32 = 0u;
var<private> STACK_ITEMS:array<SceneItem, STACK_SIZE>; 

// Base stack item mostly for padding

struct SceneItem {
    item_type: u32,
    render:u32,
    pad2:u32,
    pad3:u32,
    pad4:u32,
    pad5:u32,
    pad6:u32,
    pad7:u32,
}

struct Scene {
    entities: array<SceneItem, MAX_ENTITIES>,
}

// "Inherits" SceneItem
struct Sphere {
    item_type: u32,
    render:u32, 
    radius: f32,
}

// "Inherits" SceneItem
struct Translate {
    item_type: u32,
    render:u32, 
    pointer: u32, 
    v: vec3<f32>,
}

struct SDFEvalStack {
    pointer:u32,
    items:array<SceneItem, STACK_SIZE>,
}

struct ViewRay {
    position:vec3<f32>,
    distance:f32, // Distance along ray, 
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

fn pop() -> SceneItem {
    STACK_PTR -= 1u;
    return STACK_ITEMS[STACK_PTR];
}

fn push(item:SceneItem) {
    STACK_ITEMS[STACK_PTR] = item;
    STACK_PTR += 1u;
}

fn evaluate_sdf(index:u32, point:vec3<f32>) -> f32 {
    var signed_distance:f32 = MAX_SIGNED_DISTANCE;
    var transformed_point:vec3<f32> = point;
    push(scene.entities[index]);

    var iters = 0;
    // While items remain on the stack, evaluate them
    while iters < RECUR_DEPTH && STACK_PTR > 0u {
        iters += 1;
        let item = pop();
        let item_type = item.item_type;

        if item_type == TRANSLATE {
            var translate = as_translate(item);
            transformed_point -= translate.v;
            push(scene.entities[translate.pointer]);
        }

        if item_type == SPHERE {
            let sphere = as_sphere(item);
            signed_distance = min(signed_distance, length(transformed_point) - sphere.radius);
        }
    }

    return signed_distance;
}

fn map(point:vec3<f32>) -> f32 {
    var min_dist = MAX_SIGNED_DISTANCE;
    var stack: SDFEvalStack;

    for (var i = 0u; i < MAX_ENTITIES; i++) {
        if scene.entities[i].render != 0u {
            min_dist = min(min_dist, evaluate_sdf(i, point));
        }
    }

    return min_dist;
}

fn pixel_color(rgb: vec3<f32>) -> u32 {
    var res:u32 = 0xFF000000u;
    let color = normalize(rgb) * 255.0;

    res |= (u32(color.x) << 16u);
    res |= (u32(color.y) << 8u);
    res |= (u32(color.z));

    return res;
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


struct Dimensions {
    x:f32, 
    y:f32, 
    z:f32, 
    w:f32, 
}

@group(0) @binding(0) 
var<uniform> dimensions: Dimensions;
@group(0) @binding(1) 
var<uniform> scene: Scene;


struct Input {
    @builtin(position) screen_cords: vec4<f32>,
};

@fragment
fn main(in: Input) -> @location(0) vec4<f32> {
    let normalized = in.screen_cords / vec4<f32>(dimensions.x, dimensions.y, 1.0, 1.0);
    let aspect_ratio = dimensions.x / dimensions.y;

    let aspected = (normalized - vec4<f32>(0.5)) * vec4<f32>(aspect_ratio, 1.0, 1.0, 1.0);


    let ray_direction = normalize(vec3(aspected.x, aspected.y, 1.0));
    let ray_origin = vec3<f32>(0.0, 0.0, -10.0);

    var ray_length:f32 = CLIP_NEAR;

    for (var i:u32 = 0u; i < MAX_MARCH_STEPS; i++) {
        let point = ray_origin + (ray_direction * ray_length);
        let min_signed_distance = map(point);
        if ray_length >= CLIP_FAR { break; }
        if min_signed_distance <= 0.001 { break; }
        ray_length += min_signed_distance;
    }

    let d = 1.0/ray_length;

    let c = surface_normal(ray_origin + (ray_direction * ray_length));

    return vec4<f32>(c, 1.0);
}
