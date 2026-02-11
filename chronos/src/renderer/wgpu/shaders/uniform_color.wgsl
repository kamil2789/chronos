struct VertexOutput {
    @builtin(position) position: vec4<f32>,
}

struct ColorUniform {
    color: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> u_color: ColorUniform;

@vertex
fn vs_main(@location(0) position: vec3<f32>) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4<f32>(position, 1.0);
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return u_color.color;
}
