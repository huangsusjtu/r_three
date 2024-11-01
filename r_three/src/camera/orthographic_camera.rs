use crate::camera::CameraInterface;
use glam::Mat4;

#[derive(Clone, Debug)]
pub struct OrthographicCamera {
    eye: glam::Vec3,
    target: glam::Vec3,
    up: glam::Vec3,
    z_near: f32,
    z_far: f32,

    height: f32,

    view_width: f32,
    view_height: f32,
}

impl OrthographicCamera {
    pub fn new(
        eye: glam::Vec3,
        target: glam::Vec3,
        up: glam::Vec3,
        z_near: f32,
        z_far: f32,
        height: f32,
    ) -> Self {
        OrthographicCamera {
            eye,
            target,
            up,
            z_near,
            z_far,
            height,

            view_width: 0.0,
            view_height: 0.0,
        }
    }
}

impl CameraInterface for OrthographicCamera {
    fn r#type(&self) -> &'static str {
        "Orthographic"
    }

    fn build_view_projection_matrix(&self) -> Mat4 {
        let aspect_ratio = self.view_width / self.view_height;
        let view = glam::Mat4::look_at_rh(self.eye, self.target, self.up);
        let proj = glam::Mat4::orthographic_rh(
            -0.5 * aspect_ratio * self.height,
            0.5 * aspect_ratio * self.height,
            -0.5 * self.height,
            0.5 * self.height,
            self.z_near,
            self.z_far,
        );

        proj * view
    }

    fn build_projection_matrix(&self) -> Mat4 {
        let aspect_ratio = self.view_width / self.view_height;
        let proj = glam::Mat4::orthographic_rh(
            -0.5 * aspect_ratio * self.height,
            0.5 * aspect_ratio * self.height,
            -0.5 * self.height,
            0.5 * self.height,
            self.z_near,
            self.z_far,
        );

        proj
    }

    fn build_view_matrix(&self) -> Mat4 {
        let view = glam::Mat4::look_at_rh(self.eye, self.target, self.up);
        view
    }

    fn eye(&self) -> glam::Vec3 {
        self.eye
    }

    fn target(&self) -> glam::Vec3 {
        self.target
    }

    fn up(&self) -> glam::Vec3 {
        self.up
    }

    fn set_eye(&mut self, eye: glam::Vec3) {
        self.eye = eye;
    }

    fn set_target(&mut self, target: glam::Vec3) {
        self.target = target;
    }

    fn set_up(&mut self, up: glam::Vec3) {
        self.up = up;
    }
    fn set_viewport(&mut self, width: f32, height: f32) {
        self.view_width = width;
        self.view_height = height;
    }
}
