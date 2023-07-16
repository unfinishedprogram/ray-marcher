// @group(0) @binding(0) 
// var<uniform> dimensions: vec4<f32>;

@group(0) @binding(0)
var output_texture: texture_2d<f32>;

@group(0) @binding(1)
var texture_sampler: sampler;

struct Input {
    @builtin(position) screen_cords: vec4<f32>,
};


@fragment
fn main(in: Input) -> @location(0) vec4<f32> {
    // // Calculate UV coordinates for sampling the output texture
    // let uv = vec2<f32>(in.screen_cords.x / dimensions.x, in.screen_cords.y / dimensions.y);

    // // Sample the output texture using the computed UV coordinates
    // let color: vec4<f32> = textureSample(texture_sampler, output_texture, uv);
    let dims = textureDimensions(output_texture);
    let dims_x = f32(dims.x);
    let dims_y = f32(dims.y);
    let dims_div = vec2<f32>(dims_x, dims_y);

    return textureSample(output_texture, texture_sampler, in.screen_cords.xy / dims_div);
}