// rayIntersect.rs

use nalgebra_glm::{Vec3, dot};
use crate::figures::Sphere;
use crate::figures::Cube;
use crate::figures::RectangularPrism;
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

        Intersect::new(hit_point, normal, distance, Some(self.material.clone()), 0.0, 0.0)
    }

    fn get_normal(&self, point: &Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0) // Normal predeterminada (sin colisión detectada)
    }
}

impl Renderable for RectangularPrism {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        // Primero, determina los límites del prisma
        let half_width = self.width / 2.0;
        let half_height = self.height / 2.0;
        let half_depth = self.depth / 2.0;

        // Definir las caras del prisma
        let min_x = self.center.x - half_width;
        let max_x = self.center.x + half_width;
        let min_y = self.center.y - half_height;
        let max_y = self.center.y + half_height;
        let min_z = self.center.z - half_depth;
        let max_z = self.center.z + half_depth;

        // Calcular los t's para cada plano
        let t_x_min = (min_x - ray_origin.x) / ray_direction.x;
        let t_x_max = (max_x - ray_origin.x) / ray_direction.x;
        let t_y_min = (min_y - ray_origin.y) / ray_direction.y;
        let t_y_max = (max_y - ray_origin.y) / ray_direction.y;
        let t_z_min = (min_z - ray_origin.z) / ray_direction.z;
        let t_z_max = (max_z - ray_origin.z) / ray_direction.z;

        // Asegurar que t_x_min siempre sea menor que t_x_max
        let (t_min_x, t_max_x) = if t_x_min < t_x_max {
            (t_x_min, t_x_max)
        } else {
            (t_x_max, t_x_min)
        };

        // Hacer lo mismo para las coordenadas Y y Z
        let (t_min_y, t_max_y) = if t_y_min < t_y_max {
            (t_y_min, t_y_max)
        } else {
            (t_y_max, t_y_min)
        };

        let (t_min_z, t_max_z) = if t_z_min < t_z_max {
            (t_z_min, t_z_max)
        } else {
            (t_z_max, t_z_min)
        };

        // Encontrar el valor máximo de t_min y el mínimo de t_max
        let t_enter = t_min_x.max(t_min_y).max(t_min_z);
        let t_exit = t_max_x.min(t_max_y).min(t_max_z);

        // Verificar si hay una intersección
        if t_enter < t_exit && t_exit > 0.0 {

            let point = ray_origin + ray_direction * t_enter;
            let normal = self.get_normal(&point);
            let (u, v) = self.get_uv(&point, &normal);

            return Intersect {
                is_intersecting: true,
                distance: t_enter,
                point: point,
                normal: normal,
                material: Some(self.material.clone()),
                u: u,
                v: v,
            };
        }

        Intersect {
            is_intersecting: false,
            distance: 0.0,
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            material: None,
            u: 0.0,
            v: 0.0,
        }
    }

    fn get_normal(&self, point: &Vec3) -> Vec3 {
        let half_width = self.width / 2.0;
        let half_height = self.height / 2.0;
        let half_depth = self.depth / 2.0;

        // Determinar en qué cara del prisma se encuentra el punto
        if (point.x - (self.center.x + half_width)).abs() < 1e-4 { // Cara +X
            return Vec3::new(1.0, 0.0, 0.0);
        } else if (point.x - (self.center.x - half_width)).abs() < 1e-4 { // Cara -X
            return Vec3::new(-1.0, 0.0, 0.0);
        } else if (point.y - (self.center.y + half_height)).abs() < 1e-4 { // Cara +Y
            return Vec3::new(0.0, 1.0, 0.0);
        } else if (point.y - (self.center.y - half_height)).abs() < 1e-4 { // Cara -Y
            return Vec3::new(0.0, -1.0, 0.0);
        } else if (point.z - (self.center.z + half_depth)).abs() < 1e-4 { // Cara +Z
            return Vec3::new(0.0, 0.0, 1.0);
        } else if (point.z - (self.center.z - half_depth)).abs() < 1e-4 { // Cara -Z
            return Vec3::new(0.0, 0.0, -1.0);
        }

        Vec3::new(0.0, 0.0, 0.0) // Normal por defecto si no está en ninguna cara
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
