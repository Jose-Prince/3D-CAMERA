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

    let dirt = Material {
        diffuse: Color::new(150, 100, 50),
        specular: 15.0,  // Bajo brillo
        albedo: [0.2, 0.3, 0.1, 0.0],
        refractive_index: 1.0,
        textures: [
            Some(Texture::load_from_file("textures/dirt.png")),
            Some(Texture::load_from_file("textures/dirt.png")),
            Some(Texture::load_from_file("textures/dirt.png")),
            Some(Texture::load_from_file("textures/dirt.png")), 
            Some(Texture::load_from_file("textures/dirt.png")), 
            Some(Texture::load_from_file("textures/dirt.png")), 
        ],
        has_texture: true,
    };

    let width = 800;  // Ajusta el tamaño del framebuffer según sea necesario
    let height = 800; // Ajusta el tamaño del framebuffer según sea necesario
    let mut framebuffer = Framebuffer::new(width, height);


    //Top lair
    let cube_0_0_0: Cube = Cube {
        center: Vec3::new(0.0, 0.0, 0.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_0_0_1: Cube = Cube {
        center: Vec3::new(0.0, 0.0, 1.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_0_0_2: Cube = Cube {
        center: Vec3::new(0.0, 0.0, 2.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_0_0_3: Cube = Cube {
        center: Vec3::new(0.0, 0.0, 3.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_0_0_4: Cube = Cube {
        center: Vec3::new(0.0, 0.0, 4.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_0_0_5: Cube = Cube {
        center: Vec3::new(0.0, 0.0, 5.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_1_0_0: Cube = Cube {
        center: Vec3::new(1.0, 0.0, 0.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_1_0_1: Cube = Cube {
        center: Vec3::new(1.0, 0.0, 1.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_1_0_2: Cube = Cube {
        center: Vec3::new(1.0, 0.0, 2.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_1_0_3: Cube = Cube {
        center: Vec3::new(1.0, 0.0, 3.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_1_0_4: Cube = Cube {
        center: Vec3::new(1.0, 0.0, 4.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_1_0_5: Cube = Cube {
        center: Vec3::new(1.0, 0.0, 5.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_2_0_0: Cube = Cube {
        center: Vec3::new(2.0, 0.0, 0.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_2_0_1: Cube = Cube {
        center: Vec3::new(2.0, 0.0, 1.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_2_0_2: Cube = Cube {
        center: Vec3::new(2.0, 0.0, 2.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_2_0_3: Cube = Cube {
        center: Vec3::new(2.0, 0.0, 3.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_2_0_4: Cube = Cube {
        center: Vec3::new(2.0, 0.0, 4.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_2_0_5: Cube = Cube {
        center: Vec3::new(2.0, 0.0, 5.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_3_0_0: Cube = Cube {
        center: Vec3::new(3.0, 0.0, 0.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_3_0_1: Cube = Cube {
        center: Vec3::new(3.0, 0.0, 1.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_3_0_2: Cube = Cube {
        center: Vec3::new(3.0, 0.0, 2.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_4_0_0: Cube = Cube {
        center: Vec3::new(4.0, 0.0, 0.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_4_0_1: Cube = Cube {
        center: Vec3::new(4.0, 0.0, 1.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_4_0_2: Cube = Cube {
        center: Vec3::new(4.0, 0.0, 2.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_5_0_0: Cube = Cube {
        center: Vec3::new(5.0, 0.0, 0.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_5_0_1: Cube = Cube {
        center: Vec3::new(5.0, 0.0, 1.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_5_0_2: Cube = Cube {
        center: Vec3::new(5.0, 0.0, 2.0),
        length: 1,
        material: dirt_grass.clone(),
    };
    
    //----------------------------------------------
    //Mid lair
    let cube_0_m1_0: Cube = Cube {
        center: Vec3::new(2.0, -1.0, 2.0),
        length: 1,
        material: dirt.clone(),
    };

    let cube_0_0_1: Cube = Cube {
        center: Vec3::new(0.0, 0.0, 1.0),
        length: 1,
        material: dirt.clone(),
    };

    let cube_0_0_2: Cube = Cube {
        center: Vec3::new(0.0, 0.0, 2.0),
        length: 1,
        material: dirt.clone(),
    };

    let cube_0_0_3: Cube = Cube {
        center: Vec3::new(0.0, 0.0, 3.0),
        length: 1,
        material: dirt.clone(),
    };

    let cube_0_0_4: Cube = Cube {
        center: Vec3::new(0.0, 0.0, 4.0),
        length: 1,
        material: dirt.clone(),
    };

    let cube_0_0_5: Cube = Cube {
        center: Vec3::new(0.0, 0.0, 5.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_1_0_0: Cube = Cube {
        center: Vec3::new(1.0, 0.0, 0.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_1_0_5: Cube = Cube {
        center: Vec3::new(1.0, 0.0, 5.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_2_0_0: Cube = Cube {
        center: Vec3::new(2.0, 0.0, 0.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_2_0_3: Cube = Cube {
        center: Vec3::new(2.0, 0.0, 3.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_2_0_4: Cube = Cube {
        center: Vec3::new(2.0, 0.0, 4.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_2_0_5: Cube = Cube {
        center: Vec3::new(2.0, 0.0, 5.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_3_0_0: Cube = Cube {
        center: Vec3::new(3.0, 0.0, 0.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_3_0_2: Cube = Cube {
        center: Vec3::new(3.0, 0.0, 2.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_4_0_0: Cube = Cube {
        center: Vec3::new(4.0, 0.0, 0.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_4_0_2: Cube = Cube {
        center: Vec3::new(4.0, 0.0, 2.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_5_0_0: Cube = Cube {
        center: Vec3::new(5.0, 0.0, 0.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_5_0_1: Cube = Cube {
        center: Vec3::new(5.0, 0.0, 1.0),
        length: 1,
        material: dirt_grass.clone(),
    };

    let cube_5_0_2: Cube = Cube {
        center: Vec3::new(5.0, 0.0, 2.0),
        length: 1,
        material: dirt_grass.clone(),
    };


    let objects: Vec<&dyn Renderable> = vec![
        &cube_0_0_0,
        &cube_0_0_1,
        &cube_0_0_2,
        &cube_0_0_3,
        &cube_0_0_4,
        &cube_0_0_5,
        &cube_1_0_0,
        &cube_1_0_1,
        &cube_1_0_2,
        &cube_1_0_3,
        &cube_1_0_4,
        &cube_1_0_5,
        &cube_2_0_0,
        &cube_2_0_1,
        &cube_2_0_2,
        &cube_2_0_3,
        &cube_2_0_4,
        &cube_2_0_5,
        &cube_3_0_0,
        &cube_3_0_1,
        &cube_3_0_2,
        &cube_4_0_0,
        &cube_4_0_1,
        &cube_4_0_2,
        &cube_5_0_0,
        &cube_5_0_1,
        &cube_5_0_2,
        &cube_0_m1_0,
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