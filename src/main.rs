// main.rs

mod framebuffer;
mod color;
mod intersect;
mod material;
mod ray_intersect;
mod render;
mod bmp;


use framebuffer::Framebuffer;
use color::Color;
use material::Material;
use ray_intersect::Sphere;
use render::{render};
use nalgebra_glm::Vec3;
use minifb::{Key, Window, WindowOptions};

fn main() {
    let width = 800;  // Ajusta el tamaño del framebuffer según sea necesario
    let height = 800; // Ajusta el tamaño del framebuffer según sea necesario
    let mut framebuffer = Framebuffer::new(width, height);

    let fur = Material::new(color::Color::new(255,255,255));
    let skin = Material::new(color::Color::new(250,175,188));
    
    let right_ear: Sphere = Sphere {
        center: Vec3::new(1.7,2.0,-5.0),
        radius: 1.2,
        material: fur,
    };

    let left_ear: Sphere = Sphere {
        center: Vec3::new(-1.7,2.0,-5.0),
        radius: 1.2,
        material: fur,
    };

    let head: Sphere = Sphere {
        center: Vec3::new(0.0,0.0,-10.0),
        radius: 5.0,
        material: fur,
    };

    let inside_right_ear: Sphere = Sphere {
        center: Vec3::new(0.0,0.0,-4.0),
        radius: 1.0,
        material: skin,
    };

    let inside_left_ear: Sphere = Sphere {
        center: Vec3::new(1.7,2.0,-5.0),
        radius: 1.2,
        material: fur,
    };

    let objects = vec![head,right_ear, left_ear, inside_right_ear];

    render(&mut framebuffer, &objects);

    let mut window = Window::new(
        "3D Camera",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(framebuffer.get_buffer(), width, height).unwrap();
    }

}
