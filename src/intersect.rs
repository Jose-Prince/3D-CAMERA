use nalgebra_glm::Vec3;
use crate::color::Color;
use crate::material::Material;
use std::sync::Arc;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Intersect {
    pub point: Vec3,        // Punto de intersección
    pub normal: Vec3,       // Normal en el punto de intersección
    pub distance: f32,      // Distancia desde el origen del rayo hasta el punto de intersección
    pub is_intersecting: bool, // Indica si hay una intersección
    pub material: Option<Arc<Material>>, // Material en el punto de intersección
    pub u: f32,             // Coordenada U para texturizado
    pub v: f32,             // Coordenada V para texturizado
}

impl Intersect {
    // Constructor para crear un nuevo Intersect
    pub fn new(point: Vec3, normal: Vec3, distance: f32, material: Option<Arc<Material>>, u: f32, v: f32) -> Self {
        Intersect {
            point,
            normal,
            distance,
            is_intersecting: true,
            material,
            u,
            v,
        }
    }

    // Método para crear un Intersect vacío
    pub fn empty() -> Self {
        Intersect {
            point: Vec3::zeros(),
            normal: Vec3::zeros(),
            distance: 0.0,
            is_intersecting: false,
            material: None, // No hay material para un Intersect vacío
            u: 0.0,
            v: 0.0,
        }
    }
}

// Trait para la intersección de rayos
pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect;
}
