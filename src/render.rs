use crate::framebuffer::Framebuffer;
use crate::color::Color;
use crate::intersect::Intersect;
use crate::ray_intersect::Sphere;
use crate::intersect::RayIntersect;
use crate::light::Light;
use nalgebra_glm::{Vec3, normalize};
use std::f32;
use std::ops::Add;
use std::ops::Mul;
use crate::camera::Camera;

pub fn render(framebuffer: &mut Framebuffer, objects: &[Sphere], camera: &Camera, light: &Light) {
    let width = framebuffer.get_width() as f32;
    let height = framebuffer.get_height() as f32;
    let aspect_ratio = width / height;

    // Crear un z-buffer para almacenar la distancia de los píxeles
    let mut z_buffer = vec![f32::INFINITY; (width * height) as usize];

    for y in 0..framebuffer.get_height() {
        for x in 0..framebuffer.get_width() {
            // Mapear la coordenada del píxel al espacio de la pantalla [-1,1]
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            // Ajustar la relación de aspecto
            let screen_x = screen_x * aspect_ratio;

            // Calcular la dirección del rayo para este píxel
            let ray_camera_space = Vec3::new(screen_x, screen_y, -1.0).normalize();
            let ray_direction = camera.basis_change(&ray_camera_space);
            let ray_origin = camera.eye; // Usar la posición de la cámara como origen del rayo

            // Lanzar el rayo y obtener el color del píxel
            let (pixel_color, z) = cast_ray(&ray_origin, &ray_direction, objects, light);

            // Convertir las coordenadas de píxeles en un índice de z-buffer
            let pixel_index = (y as usize * width as usize) + (x as usize);

            // Solo dibujar el píxel si el nuevo objeto está más cerca
            if z < z_buffer[pixel_index] {
                framebuffer.set_current_color(pixel_color);
                framebuffer.point(x.try_into().unwrap(), y.try_into().unwrap());
                z_buffer[pixel_index] = z;
            }
        }
    }
}

pub fn cast_ray(
    ray_origin: &Vec3, 
    ray_direction: &Vec3, 
    objects: &[Sphere], 
    light: &Light,
) -> (Color, f32) {
    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

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

        // Intensidad difusa
        let diffuse_intensity = normal.dot(&light_dir).max(0.0);
        let diffuse = intersect.material.diffuse * intersect.material.albedo[0] * diffuse_intensity * light_intensity;

        // Intensidad especular
        let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(intersect.material.specular);
        let specular = light.color * intersect.material.albedo[1] * specular_intensity * light_intensity;

        // Ajustar el color con las intensidades calculadas
        color = color.add(diffuse);
        color = color.add(specular);

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
    objects: &[Sphere],
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
