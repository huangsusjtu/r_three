mod orthographic_camera;
pub use orthographic_camera::OrthographicCamera;

mod perspective_camera;
pub use perspective_camera::PerspectiveCamera;

pub trait Camera {
    fn r#type(&self) -> &'static str;
    fn build_view_projection_matrix(&self) -> glam::Mat4;
    fn build_projection_matrix(&self) -> glam::Mat4;
    fn build_view_matrix(&self) -> glam::Mat4;

    fn eye(&self) -> glam::Vec3;
    fn target(&self) -> glam::Vec3;
    fn up(&self) -> glam::Vec3;
    fn set_eye(&mut self, eye: glam::Vec3);
    fn set_target(&mut self, target: glam::Vec3);
    fn set_up(&mut self, up: glam::Vec3);
    fn set_viewport(&mut self, width: f32, height: f32);
}
