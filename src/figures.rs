//figures.rs

use nalgebra_glm::Vec3;
use crate::Material;
use crate::intersect::Intersect;
use std::sync::Arc;


pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

pub struct Cube {
    pub center: Vec3,
    pub length: i16,
    pub material: Arc<Material>,
}

impl Cube {

    pub fn min(&self) -> Vec3 {
        self.center - Vec3::new(self.length as f32 / 2.0, self.length as f32 / 2.0, self.length as f32 / 2.0)
    }

    pub fn max(&self) -> Vec3 {
        self.center + Vec3::new(self.length as f32 / 2.0, self.length as f32 / 2.0, self.length as f32 / 2.0)
    }

    pub fn get_uv(&self, point: &Vec3, normal: &Vec3) -> (f32, f32) {
        let mut u = 0.0;
        let mut v = 0.0;

        let min = self.min();
        let max = self.max();

        // Front face
        if normal.z.abs() > 0.99 {
            u = (point.x - min.x) / (max.x - min.x);
            v = (point.y - min.y) / (max.y - min.y);
        }
        // Back face
        else if normal.z.abs() > 0.01 && normal.z < 0.0 {
            u = (max.x - point.x) / (max.x - min.x);
            v = (point.y - min.y) / (max.y - min.y);
        }
        // Left face
        else if normal.x.abs() > 0.99 && normal.x < 0.0 {
            u = (max.z - point.z) / (max.z - min.z);
            v = (point.y - min.y) / (max.y - min.y);
        }
        // Right face
        else if normal.x.abs() > 0.99 && normal.x > 0.0 {
            u = (point.z - min.z) / (max.z - min.z);
            v = (point.y - min.y) / (max.y - min.y);
        }
        // Top face
        else if normal.y.abs() > 0.99 && normal.y > 0.0 {
            u = (point.x - min.x) / (max.x - min.x);
            v = (max.z - point.z) / (max.z - min.z);
        }
        // Bottom face
        else if normal.y.abs() > 0.99 && normal.y < 0.0 {
            u = (point.x - min.x) / (max.x - min.x);
            v = (point.z - min.z) / (max.z - min.z);
        }

        (u, v)
    }
}

