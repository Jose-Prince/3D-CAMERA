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
        diffuse: Color::from_hex(0x8B4513),
        specular: 0.05,  // Bajo brillo
        albedo: [0.9, 0.1, 0.0, 0.0],
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
        diffuse: Color::from_hex(0x8B4513),
        specular: 0.05,  // Bajo brillo
        albedo: [0.9, 0.1, 0.0, 0.0],
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

    let cobblestone = Material {
        diffuse: Color::from_hex(0x696969),
        specular: 0.3,
        albedo: [0.7, 0.3, 0.0, 0.0],
        refractive_index: 1.0,
        textures: [
            Some(Texture::load_from_file("textures/cobblestone.png")),
            Some(Texture::load_from_file("textures/cobblestone.png")),
            Some(Texture::load_from_file("textures/cobblestone.png")),
            Some(Texture::load_from_file("textures/cobblestone.png")),
            Some(Texture::load_from_file("textures/cobblestone.png")),
            Some(Texture::load_from_file("textures/cobblestone.png")),
        ],
        has_texture: true,
    };

    let leaf = Material {
        diffuse: Color::from_hex(0x228B22),
        specular: 0.2,
        albedo: [0.8, 0.2, 0.1, 0.0],
        refractive_index: 1.0,
        textures: [
            Some(Texture::load_from_file("textures/leaf.webp")),
            Some(Texture::load_from_file("textures/leaf.webp")),
            Some(Texture::load_from_file("textures/leaf.webp")),
            Some(Texture::load_from_file("textures/leaf.webp")),
            Some(Texture::load_from_file("textures/leaf.webp")),
            Some(Texture::load_from_file("textures/leaf.webp")),
        ],
        has_texture: true,
    };

    let lava = Material {
        diffuse: Color::from_hex(0xFF4500),
        specular: 0.6,
        albedo: [0.8, 0.6, 0.0, 0.0],
        refractive_index: 1.3,
        textures: [
            Some(Texture::load_from_file("textures/lava.gif")),
            Some(Texture::load_from_file("textures/lava.gif")),
            Some(Texture::load_from_file("textures/lava.gif")),
            Some(Texture::load_from_file("textures/lava.gif")),
            Some(Texture::load_from_file("textures/lava.gif")),
            Some(Texture::load_from_file("textures/lava.gif")),
        ],
        has_texture: true,
    };

    let oak_log = Material {
        diffuse: Color::from_hex(0x8B4513),
        specular: 0.1,
        albedo: [0.9, 0.1, 0.0, 0.0],
        refractive_index: 1.0,
        textures: [
            Some(Texture::load_from_file("textures/log_oak.png")),
            Some(Texture::load_from_file("textures/log_oak.png")),
            Some(Texture::load_from_file("textures/log_oak.png")),
            Some(Texture::load_from_file("textures/log_oak.png")),
            Some(Texture::load_from_file("textures/log_oak.png")),
            Some(Texture::load_from_file("textures/log_oak.png")),
        ],
        has_texture: true,
    };

    let water = Material {
        diffuse: Color::from_hex(0x1E90FF),
        specular: 0.9,
        albedo: [0.2, 0.7, 0.1, 0.0],
        refractive_index: 1.33,
        textures: [
            Some(Texture::load_from_file("textures/water.webp")),
            Some(Texture::load_from_file("textures/water.webp")),
            Some(Texture::load_from_file("textures/water.webp")),
            Some(Texture::load_from_file("textures/water.webp")),
            Some(Texture::load_from_file("textures/water.webp")),
            Some(Texture::load_from_file("textures/water.webp")),
        ],
        has_texture: true,
    };

    let chest_tetxure = Material {
        diffuse: Color::from_hex(0x8B4513),
        specular: 0.3,
        albedo: [0.7, 0.2, 0.1, 0.0],
        refractive_index: 1.0,
        textures: [
            Some(Texture::load_from_file("textures/chest_side.png")), //Right
            Some(Texture::load_from_file("textures/chest_front.png")), //Left
            Some(Texture::load_from_file("textures/chest_top.png")), //Top
            Some(Texture::load_from_file("textures/chest_top.png")), //Bottom
            Some(Texture::load_from_file("textures/chest_side.png")), //Front
            Some(Texture::load_from_file("textures/chest_side.png")), //Back
        ],
        has_texture: true
    };

    let width = 800;  // Ajusta el tamaño del framebuffer según sea necesario
    let height = 800; // Ajusta el tamaño del framebuffer según sea necesario
    let mut framebuffer = Framebuffer::new(width, height);


    let mut objects: Vec<Box<dyn Renderable>> = Vec::new(); 

    for x in 0..=5 {
        for z in 0..=2 {
            let invalid_positions = vec![(1, 1), (2, 1), (3, 1), (1, 2)];
    
            if !invalid_positions.contains(&(x, z)) {
                let cube = Cube {
                    center: Vec3::new(x as f32, 0.0, z as f32),
                    length: 1,
                    material: dirt_grass.clone().into(),
                };
                objects.push(Box::new(cube));
            }
        }
    }
    

    for x in 0..=2 {
        for z in 3..=5 {
            let cube = Cube {
                center: Vec3::new(x as f32, 0.0, z as f32),
                length: 1,
                material: dirt_grass.clone().into(),
            };
            objects.push(Box::new(cube));
        }
    }

    for x in 0..=5 {
        for z in 0..=2 {
            let cube = Cube {
                center: Vec3::new(x as f32, -1.0, z as f32),
                length: 1,
                material: dirt.clone().into(),
            };
            objects.push(Box::new(cube));
        }
    }
    

    for x in 0..=2 {
        for z in 3..=5 {
            let cube = Cube {
                center: Vec3::new(x as f32, -1.0, z as f32),
                length: 1,
                material: dirt.clone().into(),
            };
            objects.push(Box::new(cube));
        }
    }

    for i in 1..=3 {
        objects.push(Box::new(Cube {
            center: Vec3::new(1.0, i as f32, 4.0), 
            length: 1,
            material: oak_log.clone().into(), 
        }));
    }

    for x in -1..=3 {
        for z in 2..=6 {
            objects.push(Box::new(Cube {
                center: Vec3::new(x as f32, 3.0, z as f32),
                length: 1,
                material: leaf.clone().into(),
            }));
    
            objects.push(Box::new(Cube {
                center: Vec3::new(x as f32, 4.0, z as f32),
                length: 1,
                material: leaf.clone().into(),
            }));
        }
    }
    
    for (x, z) in &[
        (0.0, 3.0), (1.0, 3.0), (2.0, 3.0),
        (0.0, 5.0), (1.0, 5.0), (2.0, 5.0),
        (0.0, 4.0), (1.0, 4.0), (2.0, 4.0),
    ] {
        objects.push(Box::new(Cube {
            center: Vec3::new(*x, 5.0, *z),
            length: 1,
            material: leaf.clone().into(),
        }));
    }

    objects.push(Box::new(Cube {
        center: Vec3::new(1.0, 0.0, 2.0),
        length: 1,
        material: lava.clone().into(),
    }));

    objects.push(Box::new(Cube {
        center: Vec3::new(3.0, 0.0, 1.0),
        length: 1,
        material: water.clone().into(),
    }));

    objects.push(Box::new(Cube {
        center: Vec3::new(2.0, 0.0, 1.0),
        length: 1,
        material: water.clone().into(),
    }));

    objects.push(Box::new(Cube {
        center: Vec3::new(4.0, 1.0, 1.0),
        length: 1,
        material: chest_tetxure.clone().into(),
    }));
    
    objects.push(Box::new(Cube {
        center: Vec3::new(1.0, 0.0, 1.0),
        length: 1,
        material: cobblestone.clone().into(),
    }));

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