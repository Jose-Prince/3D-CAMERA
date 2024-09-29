// rayIntersect.rs

use nalgebra_glm::{Vec3, dot};
use crate::figures::Sphere;
use crate::figures::Cube;
use crate::material::Material;
use crate::intersect::{Intersect, RayIntersect}; // Importa Intersect y RayIntersect

pub trait Renderable {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect;
    fn get_normal(&self, point: &Vec3) -> Vec3;
}

impl Renderable for Sphere {
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

        Intersect::new(hit_point, normal, distance, self.material.clone(), (0.0, 0.0))
    }

    fn get_normal(&self, point: &Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0) // Normal predeterminada (sin colisión detectada)
    }
}

impl Renderable for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let inv_dir = Vec3::new(1.0 / ray_direction.x, 1.0 / ray_direction.y, 1.0 / ray_direction.z);
        let bounds = [
            self.center - Vec3::new(self.length as f32 / 2.0, self.length as f32 / 2.0, self.length as f32 / 2.0),
            self.center + Vec3::new(self.length as f32 / 2.0, self.length as f32 / 2.0, self.length as f32 / 2.0),
        ];
    
        let mut tmin = (bounds[0].x - ray_origin.x) * inv_dir.x;
        let mut tmax = (bounds[1].x - ray_origin.x) * inv_dir.x;
        if tmin > tmax {
            std::mem::swap(&mut tmin, &mut tmax);
        }
    
        let mut tymin = (bounds[0].y - ray_origin.y) * inv_dir.y;
        let mut tymax = (bounds[1].y - ray_origin.y) * inv_dir.y;
        if tymin > tymax {
            std::mem::swap(&mut tymin, &mut tymax);
        }
    
        if (tmin > tymax) || (tymin > tmax) {
            return Intersect::empty(); // No hay intersección
        }
    
        tmin = tmin.max(tymin);
        tmax = tmax.min(tymax);
    
        let mut tzmin = (bounds[0].z - ray_origin.z) * inv_dir.z;
        let mut tzmax = (bounds[1].z - ray_origin.z) * inv_dir.z;
        if tzmin > tzmax {
            std::mem::swap(&mut tzmin, &mut tzmax);
        }
    
        if (tmin > tzmax) || (tzmin > tmax) {
            return Intersect::empty(); // No hay intersección
        }
    
        tmin = tmin.max(tzmin);
        tmax = tmax.min(tzmax);
    
        // Si llegamos aquí, hay una intersección
        let distance = tmin; // La distancia más cercana
        let point = ray_origin + ray_direction * distance;
        let normal = self.get_normal(&point); // Debes calcular la normal en el punto de intersección
    
        let (u, v) = match normal {
            n if n.x.abs() > n.y.abs() && n.x.abs() > n.z.abs() => {
                // Cara izquierda o derecha
                let u = if n.x > 0.0 {
                    (point.z + self.length as f32 / 2.0) / self.length as f32 // Cara derecha
                } else {
                    (self.length as f32 / 2.0 - point.z) / self.length as f32 // Cara izquierda
                };
                let v = (point.y + self.length as f32 / 2.0) / self.length as f32; // Altura
                (u, v)
            },
            n if n.y.abs() > n.x.abs() && n.y.abs() > n.z.abs() => {
                // Cara superior o inferior
                let u = (point.x + self.length as f32 / 2.0) / self.length as f32; // Ancho
                let v = if n.y > 0.0 {
                    (self.length as f32 / 2.0 - point.z) / self.length as f32 // Cara superior
                } else {
                    (point.z + self.length as f32 / 2.0) / self.length as f32 // Cara inferior
                };
                (u, v)
            },
            _ => {
                // Cara frontal o trasera
                let u = (point.x + self.length as f32 / 2.0) / self.length as f32; // Ancho
                let v = if normal.z > 0.0 { // Aquí se ha cambiado n por normal
                    (point.y + self.length as f32 / 2.0) / self.length as f32 // Cara trasera
                } else {
                    (self.length as f32 / 2.0 - point.y) / self.length as f32 // Cara frontal
                };
                (u, v)
            }
        };
    
        Intersect {
            is_intersecting: true,
            distance,
            point,
            normal,
            material: self.material.clone(),
            uv: (u, v),
        }
    }
    

    fn get_normal(&self, point: &Vec3) -> Vec3 {
        let half_length = self.length as f32 / 2.0;
        let min_bound = self.center - Vec3::new(half_length, half_length, half_length);
        let max_bound = self.center + Vec3::new(half_length, half_length, half_length);

        let epsilon = 1e-5; // Pequeño valor para manejar imprecisiones de coma flotante

        // Comparar el punto con los límites del cubo para determinar la cara
        if (point.x - max_bound.x).abs() < epsilon {
            return Vec3::new(1.0, 0.0, 0.0);  // Cara +X
        } else if (point.x - min_bound.x).abs() < epsilon {
            return Vec3::new(-1.0, 0.0, 0.0); // Cara -X
        } else if (point.y - max_bound.y).abs() < epsilon {
            return Vec3::new(0.0, 1.0, 0.0);  // Cara +Y
        } else if (point.y - min_bound.y).abs() < epsilon {
            return Vec3::new(0.0, -1.0, 0.0); // Cara -Y
        } else if (point.z - max_bound.z).abs() < epsilon {
            return Vec3::new(0.0, 0.0, 1.0);  // Cara +Z
        } else if (point.z - min_bound.z).abs() < epsilon {
            return Vec3::new(0.0, 0.0, -1.0); // Cara -Z
        }

        // Si llegamos aquí, significa que no pudimos identificar la cara, lo cual
        // podría ser un error en la intersección, pero para completar:
        Vec3::new(0.0, 0.0, 0.0) // Normal predeterminada (sin colisión detectada)
    }
}

