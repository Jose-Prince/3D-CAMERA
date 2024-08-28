// rayINtersect.rs

use nalgebra_glm::{Vec3, dot};
use crate::material::Material;
use crate::intersect::{Intersect, RayIntersect}; // Importa Intersect y RayIntersect

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl RayIntersect for Sphere {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let oc = ray_origin - self.center;
        let a = dot(ray_direction, ray_direction);
        let b = 2.0 * dot(&oc, ray_direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return Intersect::empty();
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        let distance = if t1 > 0.0 { t1 } else { t2 };
        if distance < 0.0 {
            return Intersect::empty();
        }

        let hit_point = ray_origin + ray_direction * distance;
        let normal = (hit_point - self.center).normalize();

        Intersect::new(hit_point, normal, distance, self.material)
    }
}
