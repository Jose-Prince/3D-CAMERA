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

    let dirt_texture = Arc::new(Material {
        diffuse: Color::new(150, 100, 50),
            specular: 15.0,  // Bajo brillo
            albedo: [0.2, 0.3, 0.1, 0.0],
            refractive_index: 1.0,
            texture: Some(Texture::load_from_file("textures/dirt.png")),
            has_texture: true,
    });

    let width = 800;  // Ajusta el tamaño del framebuffer según sea necesario
    let height = 800; // Ajusta el tamaño del framebuffer según sea necesario
    let mut framebuffer = Framebuffer::new(width, height);

    // let dirt_texture = Texture::new("textures/dirt.jpeg").unwrap();

    let dirt = Material::new(color::Color::new(255,255,255), 0.0, [0.6, 0.3, 0.0, 0.0], 1.0, dirt_texture.get_texture(), true);
    let fur = Material::new(color::Color::new(0,255,0), 0.0, [0.6, 0.3, 0.0, 0.0], 1.0, None, false);
    let skin = Material::new(color::Color::new(250,175,188), 100.0, [0.6, 0.3, 0.0, 0.0], 1.0, None, false);
    let mouth_m = Material::new(color::Color::new(240,240,240), 0.3, [0.6, 0.3, 0.0, 0.0], 1.0, None, false);
    let black = Material::new(color::Color::new(0,0,0), 0.0, [0.6, 0.3, 0.0, 0.0], 1.0, None, false);
    
    let right_ear: Sphere = Sphere {
        center: Vec3::new(1.7,2.0,-7.0),
        radius: 1.2,
        material: fur.clone(),
    };

    let left_ear: Sphere = Sphere {
        center: Vec3::new(-1.7,2.0,-7.0),
        radius: 1.2,
        material: fur.clone(),
    };

    let head: Sphere = Sphere {
        center: Vec3::new(0.0,0.0,-5.0),
        radius: 2.0,
        material: fur.clone(),
    };

    let inside_right_ear: Sphere = Sphere {
        center: Vec3::new(1.5,1.8,-6.0),
        radius: 0.6,
        material: skin.clone(),
    };

    let inside_left_ear: Sphere = Sphere {
        center: Vec3::new(-1.5,1.8,-6.0),
        radius: 0.6,
        material: skin.clone(),
    };

    let mouth: Sphere = Sphere {
        center: Vec3::new(0.0,-0.55,-3.5),
        radius: 0.9,
        material: mouth_m.clone(),
    };

    let nose: Sphere = Sphere {
        center: Vec3::new(0.0,-0.2,-2.0),
        radius: 0.2,
        material: black.clone(),
    };

    let eye_r: Sphere = Sphere {
        center: Vec3::new(0.2,0.3,-2.0),
        radius: 0.15,
        material: black.clone(),
    };

    let eye_l: Sphere = Sphere {
        center: Vec3::new(0.0,0.0,0.0),
        radius: 0.15,
        material: black.clone(),
    };

    let cube_1: Cube = Cube {
        center: Vec3::new(-0.2,0.3,-15.0),
        length: 5,
        material: dirt.clone(),
    };

    let objects: Vec<&dyn Renderable> = vec![
        // &head,&right_ear, 
        // &left_ear, 
        // &inside_right_ear, 
        // &inside_left_ear,
        // &mouth,
        // &nose,
        // &eye_r,
        // &eye_l,
        &cube_1,
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