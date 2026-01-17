struct VertexOutput {
    @builtin(position) position: vec4<f32>,
}

@vertex
fn vs_main(@location(0) position: vec2<f32>) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4<f32>(position, 0.0, 1.0);
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Jednolity niebieski kolor
    return vec4<f32>(0.0, 0.5, 1.0, 1.0);
}
