// rayIntersect.rs

use nalgebra_glm::{Vec3, dot};
use crate::figures::Sphere;
use crate::figures::Cube;
use crate::material::Material;
use crate::intersect::{Intersect, RayIntersect}; // Importa Intersect y RayIntersect

pub trait Renderable: Send + Sync{
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

        Intersect::new(hit_point, normal, distance, Some(self.material.clone().into()), 0.0, 0.0)
    }

    fn get_normal(&self, point: &Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0) // Normal predeterminada (sin colisión detectada)
    }
}

impl Renderable for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        // Evitar división por cero
        if ray_direction.x.abs() < 1e-6 || ray_direction.y.abs() < 1e-6 || ray_direction.z.abs() < 1e-6 {
            return Intersect::empty(); // No hay intersección si el rayo no tiene dirección
        }

        let inv_dir = Vec3::new(1.0 / ray_direction.x, 1.0 / ray_direction.y, 1.0 / ray_direction.z);
        let half_length = self.length as f32 / 2.0;
        let bounds = [
            self.center - Vec3::new(half_length, half_length, half_length),
            self.center + Vec3::new(half_length, half_length, half_length),
        ];

        // Calcular tmin y tmax para el eje x
        let (mut tmin, mut tmax) = calculate_t(bounds[0].x, bounds[1].x, ray_origin.x, inv_dir.x);

        // Calcular tymin y tymax para el eje y
        let (mut tymin, mut tymax) = calculate_t(bounds[0].y, bounds[1].y, ray_origin.y, inv_dir.y);

        // Comprobar si hay intersección
        if (tmin > tymax) || (tymin > tmax) {
            return Intersect::empty(); // No hay intersección
        }

        // Actualizar tmin y tmax
        tmin = tmin.max(tymin);
        tmax = tmax.min(tymax);

        // Calcular tzmin y tzmax para el eje z
        let (mut tzmin, mut tzmax) = calculate_t(bounds[0].z, bounds[1].z, ray_origin.z, inv_dir.z);

        // Comprobar si hay intersección
        if (tmin > tzmax) || (tzmin > tmax) {
            return Intersect::empty(); // No hay intersección
        }

        // Actualizar tmin y tmax
        tmin = tmin.max(tzmin);
        tmax = tmax.min(tzmax);

        // Si llegamos aquí, hay una intersección
        if tmin < 0.0 {
            return Intersect::empty(); // Ignorar intersecciones detrás del origen
        }

        let distance = tmin; // La distancia más cercana
        let point = ray_origin + ray_direction * distance;
        let normal = self.get_normal(&point); // Calcular la normal en el punto de intersección

        let (u, v) = self.get_uv(&point, &normal);

        Intersect {
            is_intersecting: true,
            distance,
            point,
            normal,
            material: Some(self.material.clone()),
            u,
            v,
        }
    }

    fn get_normal(&self, point: &Vec3) -> Vec3 {
        let half_length = self.length as f32 / 2.0;
        let min_bound = self.center - Vec3::new(half_length, half_length, half_length);
        let max_bound = self.center + Vec3::new(half_length, half_length, half_length);

        let epsilon = 1e-4; // Pequeño valor para manejar imprecisiones de coma flotante

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

// Función auxiliar para calcular tmin y tmax
fn calculate_t(min_bound: f32, max_bound: f32, ray_origin: f32, inv_dir: f32) -> (f32, f32) {
    let tmin = (min_bound - ray_origin) * inv_dir;
    let tmax = (max_bound - ray_origin) * inv_dir;

    if inv_dir < 0.0 {
        (tmax, tmin) // Intercambiar tmin y tmax si inv_dir es negativo
    } else {
        (tmin, tmax)
    }
}
