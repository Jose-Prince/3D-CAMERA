//figures.rs

use nalgebra_glm::Vec3;
use crate::Material;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

pub struct Cube {
    pub center: Vec3,
    pub length: i16,
    pub material: Material,
}

impl Cube {
    pub fn new(center: Vec3, length: i16, material: Material) -> Self {

        Cube {
            center,
            length,
            material,
        }
    }
}