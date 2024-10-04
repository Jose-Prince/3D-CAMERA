//figures.rs

use nalgebra_glm::Vec3;
use crate::Material;
use crate::intersect::Intersect;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

pub struct RectangularPrism {
    pub center: Vec3,
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub material: Material,
}

impl RectangularPrism {
    pub fn get_uv(&self, point: &Vec3, normal: &Vec3) -> (f32, f32) {
        let width_half = self.width / 2.0;
    let depth_half = self.depth / 2.0;

    // Comprobar en función de la normal
    if normal.y > 0.0 { // Cara superior
        let u = (point.x + width_half) / self.width;
        let v = (point.z + depth_half) / self.depth;
        return (u, v);
    } else if normal.y < 0.0 { // Base inferior
        let u = (point.x + width_half) / self.width;
        let v = (point.z + depth_half) / self.depth;
        return (u, v);
    } else if normal.x > 0.0 { // Cara +X
        let v = (point.y + self.height / 2.0) / self.height;
        let u = (point.z + depth_half) / self.depth;
        return (u, v);
    } else if normal.x < 0.0 { // Cara -X
        let v = (point.y + self.height / 2.0) / self.height;
        let u = (point.z + depth_half) / self.depth;
        return (u, v);
    } else if normal.z > 0.0 { // Cara +Z
        let u = (point.x + width_half) / self.width;
        let v = (point.y + self.height / 2.0) / self.height;
        return (u, v);
    } else if normal.z < 0.0 { // Cara -Z
        let u = (point.x + width_half) / self.width;
        let v = (point.y + self.height / 2.0) / self.height;
        return (u, v);
    }

    // Si no está en ninguna cara, retornar (0.0, 0.0) como predeterminado
    (0.0, 0.0)
    }
}


pub struct Cube {
    pub center: Vec3,
    pub length: i16,
    pub material: Material,
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

