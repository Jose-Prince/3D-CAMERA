mod framebuffer;
mod color;
mod intersect;
mod material;
mod ray_intersect;
mod render;
mod bmp;
mod camera;
mod light;
mod figures;
mod texture;

use framebuffer::Framebuffer;
use color::Color;
use material::Material;
use figures::Sphere;
use figures::Cube;
use render::render;
use light::Light;
use nalgebra_glm::{Vec3, vec3};
use minifb::{Key, Window, WindowOptions};
use camera::Camera;

use crate::ray_intersect::Renderable;
use crate::texture::Texture;

fn main() {
    let width = 800;  // Ajusta el tamaño del framebuffer según sea necesario
    let height = 800; // Ajusta el tamaño del framebuffer según sea necesario
    let mut framebuffer = Framebuffer::new(width, height);
    let dirt_texture = Texture::new("textures/dirt.png");
    
    let dirt = Material::new(color::Color::new(94,58,30), 0.1, [0.8, 0.1, 0.0, 0.0], 1.0, Some("textures/dirt.jpeg"));

    let fur = Material::new(color::Color::new(255,255,255), 0.0, [0.6, 0.3, 0.0, 0.0], 1.0, Some("textures/dirt.jpeg"));
    let mouth_m = Material::new(color::Color::new(240,240,240), 0.3, [0.6, 0.3, 0.0, 0.0], 1.0, Some("textures/dirt.jpeg"));
    let black = Material::new(color::Color::new(0,0,0), 0.0, [0.6, 0.3, 0.0, 0.0], 1.0, Some("textures/dirt.jpeg"));
    

    let cube_1: Cube = Cube {
        center: Vec3::new(0.0,0.0,0.0),
        length: 1,
        material: dirt,
    };

    let cube_2: Cube = Cube {
        center: Vec3::new(1.0,0.0,0.0),
        length: 1,
        material: fur,
    };

    let objects: Vec<&dyn Renderable> = vec![
        &cube_1,
        &cube_2,
    ];

    let mut camera = Camera {
        eye: vec3(0.0, 0.0, 10.0),    // Posición inicial de la cámara
        center: vec3(0.0, 0.0, 0.0),  // Punto que la cámara está mirando
        up: vec3(0.0, 1.0, 0.0),      // Vector "up"
    };

    let light = Light::new(
        Vec3::new(5.0, 5.0, 5.0),
        Color::new(255,255,255),
        1.0,
    );

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
        framebuffer.clear();
        
        framebuffer.draw_background(&camera);
        
        // Rotación de la cámara
        if window.is_key_down(Key::A) {
            camera.orbit(0.05, 0.0);  // Rotar en el eje Y (yaw) hacia la izquierda
        }
        if window.is_key_down(Key::D) {
            camera.orbit(-0.05, 0.0);  // Rotar en el eje Y (yaw) hacia la derecha
        }
        if window.is_key_down(Key::W) {
            camera.orbit(0.0, 0.05);  // Rotar en el eje X (pitch) hacia arriba
        }
        if window.is_key_down(Key::S) {
            camera.orbit(0.0, -0.05);  // Rotar en el eje X (pitch) hacia abajo
        }

        //Changing camera zoom
        if window.is_key_down(Key::Up) {
            camera.zoom(-0.5); //Zoom in
        }

        if window.is_key_down(Key::Down) {
            camera.zoom(0.5); //Zoom out
        }

        // Renderiza la escena con la posición actual de la cámara
        render(&mut framebuffer, &objects, &camera, &light);

        window.update_with_buffer(framebuffer.get_buffer(), width, height).unwrap();
    }
}
