use crate::framebuffer::Framebuffer;
use crate::color::Color;
use crate::intersect::Intersect;
use crate::figures::Sphere;
use crate::intersect::RayIntersect;
use crate::light::Light;
use nalgebra_glm::{Vec3, normalize};
use std::f32;
use std::ops::Add;
use std::ops::Mul;
use crate::camera::Camera;
use crate::ray_intersect::Renderable;
use crate::texture::Texture;


pub fn render(
    framebuffer: &mut Framebuffer, 
    objects: &[&dyn Renderable], 
    camera: &Camera, 
    light: &Light,
) {
    let width = framebuffer.get_width() as f32;
    let height = framebuffer.get_height() as f32;
    let aspect_ratio = width / height;

    let mut z_buffer = vec![f32::INFINITY; (width * height) as usize];

    for y in 0..framebuffer.get_height() {
        for x in 0..framebuffer.get_width() {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;
            let screen_x = screen_x * aspect_ratio;
            let ray_camera_space = Vec3::new(screen_x, screen_y, -1.0).normalize();
            let ray_direction = camera.basis_change(&ray_camera_space);
            let ray_origin = camera.eye;

            let (pixel_color, z) = cast_ray(&ray_origin, &ray_direction, objects, light, 5);

            let pixel_index = (y as usize * width as usize) + (x as usize);

            if z < z_buffer[pixel_index] {
                
                framebuffer.set_current_color(pixel_color);
                framebuffer.point(x.try_into().unwrap(), y.try_into().unwrap());
                z_buffer[pixel_index] = z;
            }
        }
    }
}

fn cast_ray(
    ray_origin: &Vec3, 
    ray_direction: &Vec3, 
    objects: &[&dyn Renderable], 
    light: &Light,
    depth: u32
) -> (Color, f32) {
    if depth == 0 {
        return (Color::new(0, 0, 0), f32::INFINITY); // Limitar la profundidad de las reflexiones y refracciones
    }    

    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    // Buscar el objeto más cercano con el que el rayo intersecta
    for object in objects {
        let tmp = object.ray_intersect(ray_origin, ray_direction);
        if tmp.is_intersecting && tmp.distance < zbuffer {
            zbuffer = tmp.distance;
            intersect = tmp;
        }
    }

    if intersect.is_intersecting {
        let normal = intersect.normal;
        let mut color = Color::new(0, 0, 0);

        let light_dir = (light.position - intersect.point).normalize();
        let view_dir = (ray_origin - intersect.point).normalize();
        let reflect_dir = reflect(&-light_dir, &intersect.normal);

        // Calcular la intensidad de la sombra
        let shadow_intensity = cast_shadow(&intersect, light, objects);
        let light_intensity = light.intensity * (1.0 - shadow_intensity);

        // Luz ambiental: color constante aplicado a todas las superficies
        let ambient_light_intensity = 0.3; // Valor que puedes ajustar según tu preferencia
        let ambient_light_color = Color::new(80, 80, 80); // Color ambiental (gris oscuro)

        // Determinar la textura en función de las coordenadas UV o la normal
        let texture_color = if let Some(ref material) = intersect.material {
            let face_index = if normal.x.abs() > 0.9 {
                if normal.x > 0.0 { 0 } else { 1 } // Caras derecha e izquierda
            } else if normal.y.abs() > 0.9 {
                if normal.y > 0.0 { 2 } else { 3 } // Caras superior e inferior
            } else if normal.z.abs() > 0.9 {
                if normal.z > 0.0 { 4 } else { 5 } // Caras frontal y trasera
            } else {
                0 // Default
            };

            material.get_diffuse_color(face_index, intersect.u, 1.0 - intersect.v)
        } else {
            Color::new(0, 0, 0) // Color negro si no hay material
        };

        // Intensidad difusa
        let diffuse_intensity = normal.dot(&light_dir).max(0.2);

        let ambient = ambient_light_color.mul(ambient_light_intensity);
        let diffuse = texture_color.mul(diffuse_intensity * light_intensity);

        if let Some(ref material) = intersect.material {
            // Intensidad especular
            let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(material.specular);
            let specular = light.color * material.albedo[1] * specular_intensity * light_intensity;

            // Reflexión
            if material.albedo[2] > 0.0 {
                let reflection_color = cast_ray(&intersect.point, &reflect(&ray_direction, &normal), objects, light, depth - 1).0;
                color = color.add(reflection_color.mul(material.albedo[2]));
            }

            // Refracción
            if material.albedo[3] > 0.0 {
                let refraction_color = cast_ray_with_refraction(&intersect, &ray_direction, objects, light, depth - 1);
                color = color.add(refraction_color.mul(material.albedo[3]));
            }

            // Sumar las componentes de luz ambiental, difusa y especular
            let shadow_color = Color::new(20, 20, 20);
            color = ambient.add(diffuse).add(shadow_color.mul(shadow_intensity)); // Agregar luz ambiental
        }

        (color, intersect.distance)
    } else {
        (Color::new(4, 12, 36), f32::INFINITY) // Color de fondo
    }
}


fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

fn cast_shadow(
    intersect: &Intersect,
    light: &Light,
    objects: &[&dyn Renderable],
) -> f32 {
    let light_dir = (light.position - intersect.point).normalize();
    let shadow_ray_origin = intersect.point + light_dir * 1e-3; // Mover ligeramente el origen para evitar "shadow acne"
    let mut shadow_intensity: f32 = 0.0;
    let mut zbuffer = f32::INFINITY;

    for object in objects {
        let shadow_intersect = object.ray_intersect(&shadow_ray_origin, &light_dir);
        if shadow_intersect.is_intersecting && shadow_intersect.distance < zbuffer {
            zbuffer = shadow_intersect.distance;
            shadow_intensity += 0.5; // Suma la intensidad de sombra para cada intersección
        }
    }

    shadow_intensity.clamp(0.0, 1.0) // Limitar la intensidad entre 0 y 1
}

fn cast_ray_with_refraction(
    intersect: &Intersect, 
    ray_direction: &Vec3, 
    objects: &[&dyn Renderable], 
    light: &Light, 
    depth: u32
) -> Color {

    if depth == 0 {
        return Color::new(0, 0, 0); // Limitar la profundidad
    }

    let normal = intersect.normal;
    let n1 = 1.0; // Índice de refracción del aire
    let n2 = intersect.material.clone().unwrap().refractive_index; // Índice del material

    // Calcular el ángulo de refracción usando la Ley de Snell
    let cos_i = -ray_direction.dot(&normal);
    let sin_t2 = (n1 / n2) * (n1 / n2) * (1.0 - cos_i * cos_i);

    if sin_t2 > 1.0 {
        // Reflexión total interna
        return cast_ray(&intersect.point, &reflect(ray_direction, &normal), objects, light, depth - 1).0;
    } else {
        // Refracción
        let cos_t = (1.0 - sin_t2).sqrt();
        let refracted_direction = (n1 / n2) * ray_direction + (n1 / n2 * cos_i - cos_t) * normal;
        return cast_ray(&intersect.point, &refracted_direction, objects, light, depth - 1).0;
    }    
}