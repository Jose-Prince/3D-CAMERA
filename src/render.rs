// render.rs

use crate::framebuffer::Framebuffer;
use crate::color::Color;
use crate::intersect::Intersect;
use crate::ray_intersect::Sphere;
use crate::intersect::RayIntersect;
use nalgebra_glm::{Vec3, normalize};
use std::f32;

pub fn render(framebuffer: &mut Framebuffer, objects: &[Sphere]) {
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
            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));

            // Lanzar el rayo y obtener el color del píxel
            let (pixel_color, z) = cast_ray(&Vec3::new(0.0, 0.0, 0.0), &ray_direction, objects);

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

pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Sphere]) -> (Color, f32) {
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
        (intersect.material.diffuse, intersect.distance)
    } else {
        (Color::new(4, 12, 36), f32::INFINITY)
    }
}
