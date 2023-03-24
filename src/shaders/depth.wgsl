const STACK_SIZE: u32 = 64u;
const MAX_ENTITIES: u32 = 64u;
const MAX_SIGNED_DISTANCE: f32 = 1000000.0;


struct Scene {
    entities: array<SceneItem, MAX_ENTITIES>,
    render_queue: array<u32, MAX_ENTITIES>,
    entities_length: u32,
    render_queue_length: u32,
}


// Base stack item mostly for padding
struct SceneItem {
    item_type: u32,
    _padding: array<u32, 7u>,
}

// "Inherits" SceneItem
const SPHERE:u32 = 0u;
struct Sphere {
    item_type: u32,
    radius: f32,
    _padding: array<u32, 6u>,
}

// "Inherits" SceneItem
const TRANSLATE:u32 = 1u;
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
    sphere.radius = bitcast<f32>(item._padding[0]);
    sphere.item_type = SPHERE;
    return sphere;
}

fn as_translate(item:SceneItem) -> Translate {
    var translate:Translate;
    translate.item_type = TRANSLATE;

    let x = bitcast<f32>(item._padding[1]);
    let y = bitcast<f32>(item._padding[2]);
    let z = bitcast<f32>(item._padding[3]);

    let pointer = item._padding[4];

    translate.v = vec3<f32>(x, y, z);
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
        let item_type = item.item_type;

        switch item_type {
            case TRANSLATE {
                var translate = as_translate(item);
                transformed_point -= translate.v;
                push(&stack, scene.entities[translate.pointer]);
            }

            case SPHERE {
                let sphere = as_sphere(item);
                signed_distance = min(signed_distance, length(point) - sphere.radius);
            }

            default {}
        }
    }

    return signed_distance;
}

fn map(point:vec3<f32>) -> f32 {
    var min_dist:f32 = MAX_SIGNED_DISTANCE;

    var stack: SDFEvalStack;

    for (var i = 0u; i < scene.render_queue_length; i++) {
        let index = scene.render_queue[i];
        min_dist = min(min_dist, evaluate_sdf(index, point));
    }

    return min_dist;
}

const MAX:f32 = 25.0;


@group(0) @binding(0) var<storage, read_write> ray_buffer: array<ViewRay>;
@group(0) @binding(1) var<uniform> dimensions: vec2<u32>;
@group(0) @binding(2) var<storage, read> scene: Scene;
@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var index = global_id.x + global_id.y * dimensions.x;

    let position = vec2<f32>(f32(global_id.x), f32(global_id.y));
    let dims_f32 = vec2<f32>(f32(dimensions.x), f32(dimensions.y));

    let aspect_ratio = dims_f32.x / dims_f32.y;

    var normalized = ((position / dims_f32) - vec2<f32>(0.5, 0.5)) * vec2<f32>(aspect_ratio, 1.0);

    let ray_direction = normalize(vec3(normalized, 1.0));
    let ray_position = vec3<f32>(0.0, 0.0, -10.0);

    var t:f32 = 0.0;

    for (var i = 0; i < 256; i++) {
        let point = ray_position + (ray_direction * t);
        let d = map(point);
        if d <= 0.01 { break; }
        if t >= MAX { break; }

        t += d;
    }
    

    t = clamp(t, 0.0, MAX);
    t/=MAX;
    // ray_buffer[index].position = ray_direction;z q
    ray_buffer[index].position = vec3<f32>(t, t, t);
    ray_buffer[index].distance = t;
}