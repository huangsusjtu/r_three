use crate::color::Color;
use crate::materials::Material;

pub struct MeshBasicMaterial {
    pub color: Color,
}

impl MeshBasicMaterial {}

impl Material for MeshBasicMaterial {}
