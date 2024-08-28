// Render.rs

use crate::fileReader::load_maze;
use crate::framebuffer::Framebuffer;
use crate::color::Color;
use std::io::Result;

pub fn render(framebuffer: &mut Framebuffer, objects: &[Object]) {
    let width = framebuffer.get_width() as f32;
    let height = framebuffer.get_height() as f32;
    let aspect_ratio = width/height;

    for y in 0..framebuffer.get_height() {
        for x in 0..framebuffer.get_width() {
            //Map the pixel coordinate to screen space [-1,1]
            let scren_x = (2.0 * x as f32)/ width -1.0;
            let screen_y = -(2.0 * y as f32)/ height + 1.0;

            //Adjust for aspect ratio
            let screen_x = screen_x * aspect_ratio;

            //Calculate the direction of the ray for this pixel
            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));

            //Cast the ray and get the pixel color
            let pixel_color =cast_ray(&Vec3::new(0.0,0.0,0.0), &ray_direction, objects);

            //Draw the pixel on screen with the returned color
            framebuffer.set_current_color(pixel_color);
            framebuffer.point(x,y);
        }
    }
}

pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Sphere]) -> Color {
    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY; 

    for object in objects {
        let tmp = object.ray_intersect(ray_origin, ray_direction);
        if tmp.is_intersecting && tmp.distance < zbuffer {
            zbuffer = intersect.distance;
            intersect = tmp;
        }
    }

    if !intersect.is_intersecting {
        return Color::new(4,12,36);
    }

    let diffuse = intersect.material.diffuse;
    
    diffuse
}
