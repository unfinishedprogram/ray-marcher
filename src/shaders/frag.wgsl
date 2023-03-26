const STACK_SIZE = 4u;
const MAX_ENTITIES = 4u;
const MAX_SIGNED_DISTANCE = 1000000.0;
const MAX_MARCH_STEPS = 255u;

const CLIP_NEAR:f32 = 0.001;
const CLIP_FAR:f32 = 10000.0;


const SPHERE = 0u;
const TRANSLATE = 1u;

// Base stack item mostly for padding

struct SceneItem {
    v:vec3<u32>
    // item_type: u32,
    // _padding: array<u32, 7u>,
}

struct RenderAble {
    index:u32,
    pad0:u32,
    pad1:u32,
    pad2:u32,
}

struct Scene {
    entities: array<SceneItem, MAX_ENTITIES>,
    render_queue: array<RenderAble, MAX_ENTITIES>,
    // entities_length: u32,
    // render_queue_length: u32,
}

// "Inherits" SceneItem
struct Sphere {
    item_type: u32,
    radius: f32,
    _padding: array<u32, 6u>,
}

// "Inherits" SceneItem
struct Translate {
    item_type: u32,
    v: vec3<f32>,
    pointer: u32, 
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
    // sphere.radius = bitcast<f32>(item._padding[0]);
    sphere.radius = 1.0;
    sphere.item_type = SPHERE;
    return sphere;
}

fn as_translate(item:SceneItem) -> Translate {
    var translate:Translate;
    translate.item_type = TRANSLATE;

    // let x = bitcast<f32>(item._padding[0]);
    // let y = bitcast<f32>(item._padding[1]);
    // let z = bitcast<f32>(item._padding[2]);

    // let pointer = item._padding[3];
    let pointer = 1u;

    // translate.v = vec3<f32>(x, y, z);
    translate.v = vec3(1.0);
    
    translate.pointer = pointer;
    return translate;
}

fn pop(p_stack: ptr<function, SDFEvalStack>) -> SceneItem {
    var stack = *p_stack;
    stack.pointer -= 1u;
    *p_stack = stack;
    return stack.items[stack.pointer];

}

fn push(p_stack: ptr<function, SDFEvalStack>, item:SceneItem) {
    var stack = *p_stack;
    stack.items[stack.pointer] = item;
    stack.pointer += 1u;
    *p_stack = stack;
}

fn evaluate_sdf(index:u32, point:vec3<f32>) -> f32 {
    var stack: SDFEvalStack;

    var signed_distance:f32 = MAX_SIGNED_DISTANCE;
    var transformed_point:vec3<f32> = point;
    push(&stack, scene.entities[index]);

    // While items remain on the stack, evaluate them
    while stack.pointer > 0u {
        let item = pop(&stack);
        // let item_type = item.item_type;
        let item_type = item.v[0];

        switch item_type {
            case 1u {
                var translate = as_translate(item);
                transformed_point -= translate.v;
                push(&stack, scene.entities[translate.pointer]);
            }

            case 0u {
                let sphere = as_sphere(item);
                signed_distance = min(signed_distance, length(transformed_point) - sphere.radius);
            }

            default {}
        }
    }

    return signed_distance;
}

fn map(point:vec3<f32>) -> f32 {
    var min_dist:f32 = MAX_SIGNED_DISTANCE;

    var stack: SDFEvalStack;

    for (var i = 0u; i < MAX_ENTITIES; i++) {
        // let index = scene.render_queue[i];
        let index = scene.render_queue[i].index;
        min_dist = min(min_dist, evaluate_sdf(index, point));
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
    let step_x = vec3<f32>(0.00001, 0.0, 0.0);
    let step_y = vec3<f32>(0.0, 0.00001, 0.0);
    let step_z = vec3<f32>(0.0, 0.0, 0.00001);

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

    // for (var i:u32 = 0u; i < MAX_MARCH_STEPS; i++) {
    //     let point = ray_origin + (ray_direction * ray_length);

    //     let min_signed_distance = map(point);

    //     if ray_length >= CLIP_FAR { break; }
    //     if min_signed_distance <= 0.001 { break; }

    //     ray_length += min_signed_distance;
    // }

    return vec4<f32>(clamp(ray_length, 0.0, 25.0) / 25.0) ;

    // return vec4<f32>(ray_direction, 1.0);
    // let d = dimensions;
}
