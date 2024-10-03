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

use std::sync::Arc;
use crate::ray_intersect::Renderable;
use crate::texture::Texture;

fn main() {

    let dirt_grass = Material {
        diffuse: Color::new(150, 100, 50),
        specular: 15.0,  // Bajo brillo
        albedo: [0.2, 0.3, 0.1, 0.0],
        refractive_index: 1.0,
        textures: [
            Some(Texture::load_from_file("textures/dirt_side.webp")), //Right
            Some(Texture::load_from_file("textures/dirt_side.webp")), //Left
            Some(Texture::load_from_file("textures/dirt_top.jpg")), //Top
            Some(Texture::load_from_file("textures/dirt.png")), //Bottom
            Some(Texture::load_from_file("textures/dirt_side.webp")), //Front
            Some(Texture::load_from_file("textures/dirt_side.webp")), //Back
        ],
        has_texture: true,
    };

    let width = 800;  // Ajusta el tamaño del framebuffer según sea necesario
    let height = 800; // Ajusta el tamaño del framebuffer según sea necesario
    let mut framebuffer = Framebuffer::new(width, height);

    let cube_1: Cube = Cube {
        center: Vec3::new(0.0, 0.0, 0.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_2: Cube = Cube {
        center: Vec3::new(0.0, 0.0, 1.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_3: Cube = Cube {
        center: Vec3::new(0.0, 0.0, 2.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_4: Cube = Cube {
        center: Vec3::new(0.0, 0.0, 3.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_5: Cube = Cube {
        center: Vec3::new(1.0, 0.0, 0.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_6: Cube = Cube {
        center: Vec3::new(1.0, 0.0, 1.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_7: Cube = Cube {
        center: Vec3::new(1.0, 0.0, 2.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_8: Cube = Cube {
        center: Vec3::new(1.0, 0.0, 3.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_9: Cube = Cube {
        center: Vec3::new(2.0, 0.0, 0.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_10: Cube = Cube {
        center: Vec3::new(2.0, 0.0, 1.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_11: Cube = Cube {
        center: Vec3::new(2.0, 0.0, 2.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_12: Cube = Cube {
        center: Vec3::new(2.0, 0.0, 3.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_13: Cube = Cube {
        center: Vec3::new(-1.0, 0.0, 0.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_14: Cube = Cube {
        center: Vec3::new(-2.0, 0.0, 0.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_15: Cube = Cube {
        center: Vec3::new(-3.0, 0.0, 0.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let objects: Vec<&dyn Renderable> = vec![
        &cube_1,
        &cube_2,
        &cube_3,
        &cube_4,
        &cube_5,
        &cube_6,
        &cube_7,
        &cube_8,
        &cube_9,
        &cube_10,
        &cube_11,
        &cube_12,
        &cube_13,
        &cube_14,
        &cube_15,
    ];

    let mut camera = Camera {
        eye: vec3(0.0, 0.0, 10.0),    // Posición inicial de la cámara
        center: vec3(0.0, 0.0, 0.0),  // Punto que la cámara está mirando
        up: vec3(0.0, 1.0, 0.0),      // Vector "up"
    };

    let light = Light::new(
        Vec3::new(10.0, 10.0, 10.0),
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