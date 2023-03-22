struct ViewRay {
    position:vec3<f32>,
    distance:f32, // Distance along ray, 
}

@group(0) @binding(0) 
var<storage, read_write> ray_buffer: array<ViewRay>;

@group(0) @binding(1) 
var<uniform> dimensions: vec2<u32>;

fn map(point:vec3<f32>) -> f32 {
    return length(point) - 2.0;
}

const MAX:f32 = 25.0;

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var index = global_id.x + global_id.y * dimensions.x;

    let position = vec2<f32>(f32(global_id.x), f32(global_id.y));
    let dims_f32 = vec2<f32>(f32(dimensions.x), f32(dimensions.y));

    let aspect_ratio = dims_f32.x / dims_f32.y;

    var normalized = ((position / dims_f32) - vec2<f32>(0.5, 0.5)) * vec2<f32>(aspect_ratio, 1.0);

    let ray_direction = normalize(vec3(normalized, 1.0));
    let ray_position = vec3<f32>(0.0, 0.0, -10.0);

    var i = 0;
    var t:f32 = 0.0;

    loop {
        let d = map(ray_position + (ray_direction * t));

        if d <= 0.01 {
            t = 0.0;
            break;
        }

        if i >= 256 || t >= MAX {
            t = MAX;
            break;
        }

        t += d;
        i += 1;
    }

    ray_buffer[index].position = ray_direction;
    ray_buffer[index].distance = t;
}