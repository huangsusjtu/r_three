



const VERTEX_BASE_SHADER: &'static str = r#"
struct VertexInput {
    @location(0) position: vec3f,
    @location(1) color: vec3f,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4f,
    @location(0) color: vec3f,
};
"#;



const CAMERA_UNIFORM_SHADER: &'static str = r#"
struct Uniforms {
    projection: mat4x4<f32>,
    camera_pos: vec4<f32>,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
"#;

const VS_MAIN_SHADER: &'static str = r#"
@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.color = vec3f(0.9,0.0,0.0);
    out.clip_position = uniforms.projection * vec4f(model.position, 1.0);
    return out;
}
"#;

const FS_MAIN_SHADER: &'static str = r#"
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    return vec4f(in.color, 1.0);
}
"#;
