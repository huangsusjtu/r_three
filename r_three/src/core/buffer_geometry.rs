pub struct BufferGeometry {
    pub position: Vec<glam::Vec3>,
}

impl BufferGeometry {
    pub fn new() -> BufferGeometry {
        BufferGeometry { position: Vec::new() }
    }

    pub fn set_from_points(&mut self, points: Vec<glam::Vec3>) {
        self.position = points;
    }
}
