
struct CameraUniforms {
    view_proj: mat4x4f,
}

@group(0) @binding(0) var<uniform> camera: CameraUniforms;

// 顶点着色器
struct VertexInput {
    @location(0) position: vec3f,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4f,
    @location(0) color: vec3f,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
//    out.color = model.color;
    out.color = vec3f(0.9, 0.0,0.0);
//    out.clip_position = vec4f(model.position, 1.0);
    out.clip_position = camera.view_proj * vec4f(model.position, 1.0);
    return out;
}

// 片元着色器

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    return vec4f(in.color, 1.0);
}