struct ViewRay {
    position:vec3<f32>,
    distance:f32, // Distance along ray, 
}

@group(0) @binding(0) 
var<storage, read_write> ray_buffer: array<ViewRay>;

@group(0) @binding(1) 
var<uniform> dimensions: vec2<u32>;

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var index = global_id.x + global_id.y * dimensions.x;

    let position = vec2<f32>(f32(global_id.x), f32(global_id.y));
    let dims_f32 = vec2<f32>(f32(dimensions.x), f32(dimensions.y));

    let aspect_ratio = dims_f32.x / dims_f32.y;

    var normalized = ((position / dims_f32) - vec2<f32>(0.5, 0.5)) * vec2<f32>(aspect_ratio, 1.0);

    let ray_position = normalize(vec3(normalize(normalized), 1.0));

    ray_buffer[index].position = ray_position;
    ray_buffer[index].distance = f32(index);
}