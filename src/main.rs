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
        center: Vec3::new(0.0, -1.0, 0.0),
        length: 1,
        material: dirt.clone(),
    };

    let cube_0_m1_1: Cube = Cube {
        center: Vec3::new(0.0, -1.0, 1.0),
        length: 1,
        material: dirt.clone(),
    };

    let cube_0_m1_2: Cube = Cube {
        center: Vec3::new(0.0, -1.0, 2.0),
        length: 1,
        material: dirt.clone(),
    };

    let cube_0_m1_3: Cube = Cube {
        center: Vec3::new(0.0, -1.0, 3.0),
        length: 1,
        material: dirt.clone(),
    };

    let cube_0_m1_4: Cube = Cube {
        center: Vec3::new(0.0, -1.0, 4.0),
        length: 1,
        material: dirt.clone(),
    };

    let cube_0_m1_5: Cube = Cube {
        center: Vec3::new(0.0, -1.0, 5.0),
        length: 1,
        material: dirt.clone(),
    };

    let cube_1_m1_0: Cube = Cube {
        center: Vec3::new(1.0, -1.0, 0.0),
        length: 1,
        material: dirt.clone(),
    };

    let cube_1_m1_2: Cube = Cube {
        center: Vec3::new(1.0, -1.0, 2.0),
        length: 1,
        material: dirt.clone(),
    };


    let cube_1_m1_5: Cube = Cube {
        center: Vec3::new(1.0, -1.0, 5.0),
        length: 1,
        material: dirt.clone(),
    };

    let cube_2_m1_0: Cube = Cube {
        center: Vec3::new(2.0, -1.0, 0.0),
        length: 1,
        material: dirt.clone(),
    };

    let cube_2_m1_1: Cube = Cube {
        center: Vec3::new(2.0, -1.0, 1.0),
        length: 1,
        material: dirt.clone(),
    };

    let cube_2_m1_3: Cube = Cube {
        center: Vec3::new(2.0, -1.0, 3.0),
        length: 1,
        material: dirt.clone(),
    };

    let cube_2_m1_4: Cube = Cube {
        center: Vec3::new(2.0, -1.0, 4.0),
        length: 1,
        material: dirt.clone(),
    };

    let cube_2_m1_5: Cube = Cube {
        center: Vec3::new(2.0, -1.0, 5.0),
        length: 1,
        material: dirt.clone(),
    };

    let cube_3_m1_0: Cube = Cube {
        center: Vec3::new(3.0, -1.0, 0.0),
        length: 1,
        material: dirt.clone(),
    };

    let cube_3_m1_1: Cube = Cube {
        center: Vec3::new(3.0, -1.0, 1.0),
        length: 1,
        material: dirt.clone(),
    };

    let cube_3_m1_2: Cube = Cube {
        center: Vec3::new(3.0, -1.0, 2.0),
        length: 1,
        material: dirt.clone(),
    };

    let cube_4_m1_0: Cube = Cube {
        center: Vec3::new(4.0, -1.0, 0.0),
        length: 1,
        material: dirt.clone(),
    };

    let cube_4_m1_2: Cube = Cube {
        center: Vec3::new(4.0, -1.0, 2.0),
        length: 1,
        material: dirt.clone(),
    };

    let cube_5_m1_0: Cube = Cube {
        center: Vec3::new(5.0, -1.0, 0.0),
        length: 1,
        material: dirt.clone(),
    };

    let cube_5_m1_1: Cube = Cube {
        center: Vec3::new(5.0, -1.0, 1.0),
        length: 1,
        material: dirt.clone(),
    };

    let cube_5_m1_2: Cube = Cube {
        center: Vec3::new(5.0, -1.0, 2.0),
        length: 1,
        material: dirt.clone(),
    };

    let cobblestone_cube: Cube = Cube {
        center: Vec3::new(1.0, 0.0, 1.0),
        length: 1,
        material: cobblestone.clone(),
    };

    //------------------------
    //Tree

    let log_1 : Cube = Cube {
        center: Vec3::new(1.0, 1.0, 4.0),
        length: 1,
        material: oak_log.clone()
    };

    let log_2 : Cube = Cube {
        center: Vec3::new(1.0, 2.0, 4.0),
        length: 1,
        material: oak_log.clone()
    };

    let log_3 : Cube = Cube {
        center: Vec3::new(1.0, 3.0, 4.0),
        length: 1,
        material: oak_log.clone()
    };

    let leaf_1: Cube = Cube {
        center: Vec3::new(-1.0, 3.0, 2.0),
        length: 1,
        material: leaf.clone(),
    };

    let leaf_1_2: Cube = Cube {
        center: Vec3::new(-1.0, 4.0, 2.0),
        length: 1,
        material: leaf.clone(),
    };

    let leaf_2: Cube = Cube {
        center: Vec3::new(0.0, 3.0, 2.0),
        length: 1,
        material: leaf.clone(),
    };

    let leaf_2_2: Cube = Cube {
        center: Vec3::new(0.0, 4.0, 2.0),
        length: 1,
        material: leaf.clone(),
    };
    let leaf_3: Cube = Cube {
        center: Vec3::new(1.0, 3.0, 2.0),
        length: 1,
        material: leaf.clone(),
    };

    let leaf_3_2: Cube = Cube {
        center: Vec3::new(1.0, 4.0, 2.0),
        length: 1,
        material: leaf.clone(),
    };
    let leaf_4: Cube = Cube {
        center: Vec3::new(2.0, 3.0, 2.0),
        length: 1,
        material: leaf.clone(),
    };

    let leaf_4_2: Cube = Cube {
        center: Vec3::new(2.0, 4.0, 2.0),
        length: 1,
        material: leaf.clone(),
    };
    let leaf_5: Cube = Cube {
        center: Vec3::new(3.0, 3.0, 2.0),
        length: 1,
        material: leaf.clone(),
    };

    let leaf_5_2: Cube = Cube {
        center: Vec3::new(3.0, 4.0, 2.0),
        length: 1,
        material: leaf.clone(),
    };
    let leaf_6: Cube = Cube {
        center: Vec3::new(-1.0, 3.0, 6.0),
        length: 1,
        material: leaf.clone(),
    };

    let leaf_6_2: Cube = Cube {
        center: Vec3::new(-1.0, 4.0, 6.0),
        length: 1,
        material: leaf.clone(),
    };
    let leaf_7: Cube = Cube {
        center: Vec3::new(0.0, 3.0, 6.0),
        length: 1,
        material: leaf.clone(),
    };

    let leaf_7_2: Cube = Cube {
        center: Vec3::new(0.0, 4.0, 6.0),
        length: 1,
        material: leaf.clone(),
    };
    let leaf_8: Cube = Cube {
        center: Vec3::new(1.0, 3.0, 6.0),
        length: 1,
        material: leaf.clone(),
    };

    let leaf_8_2: Cube = Cube {
        center: Vec3::new(1.0, 4.0, 6.0),
        length: 1,
        material: leaf.clone(),
    };
    let leaf_9: Cube = Cube {
        center: Vec3::new(2.0, 3.0, 6.0),
        length: 1,
        material: leaf.clone(),
    };

    let leaf_9_2: Cube = Cube {
        center: Vec3::new(2.0, 4.0, 6.0),
        length: 1,
        material: leaf.clone(),
    };
    let leaf_10: Cube = Cube {
        center: Vec3::new(3.0, 3.0, 6.0),
        length: 1,
        material: leaf.clone(),
    };

    let leaf_10_2: Cube = Cube {
        center: Vec3::new(3.0, 4.0, 6.0),
        length: 1,
        material: leaf.clone(),
    };
    let leaf_11: Cube = Cube {
        center: Vec3::new(-1.0, 3.0, 3.0),
        length: 1,
        material: leaf.clone(),
    };

    let leaf_11_2: Cube = Cube {
        center: Vec3::new(-1.0, 4.0, 3.0),
        length: 1,
        material: leaf.clone(),
    };
    let leaf_12: Cube = Cube {
        center: Vec3::new(-1.0, 3.0, 4.0),
        length: 1,
        material: leaf.clone(),
    };

    let leaf_12_2: Cube = Cube {
        center: Vec3::new(-1.0, 4.0, 4.0),
        length: 1,
        material: leaf.clone(),
    };
    let leaf_13: Cube = Cube {
        center: Vec3::new(-1.0, 3.0, 5.0),
        length: 1,
        material: leaf.clone(),
    };

    let leaf_13_2: Cube = Cube {
        center: Vec3::new(-1.0, 4.0, 5.0),
        length: 1,
        material: leaf.clone(),
    };
    let leaf_14: Cube = Cube {
        center: Vec3::new(3.0, 3.0, 3.0),
        length: 1,
        material: leaf.clone(),
    };

    let leaf_14_2: Cube = Cube {
        center: Vec3::new(3.0, 4.0, 3.0),
        length: 1,
        material: leaf.clone(),
    };
    let leaf_15: Cube = Cube {
        center: Vec3::new(3.0, 3.0, 4.0),
        length: 1,
        material: leaf.clone(),
    };

    let leaf_15_2: Cube = Cube {
        center: Vec3::new(3.0, 4.0, 4.0),
        length: 1,
        material: leaf.clone(),
    };
    let leaf_16: Cube = Cube {
        center: Vec3::new(3.0, 3.0, 5.0),
        length: 1,
        material: leaf.clone(),
    };

    let leaf_16_2: Cube = Cube {
        center: Vec3::new(3.0, 4.0, 5.0),
        length: 1,
        material: leaf.clone(),
    };

    let leaf_17: Cube = Cube {
        center: Vec3::new(0.0, 5.0, 3.0),
        length: 1,
        material: leaf.clone(),
    };

    let leaf_18: Cube = Cube {
        center: Vec3::new(1.0, 5.0, 3.0),
        length: 1,
        material: leaf.clone(),
    };

    let leaf_19: Cube = Cube {
        center: Vec3::new(2.0, 5.0, 3.0),
        length: 1,
        material: leaf.clone(),
    };

    let leaf_20: Cube = Cube {
        center: Vec3::new(0.0, 5.0, 5.0),
        length: 1,
        material: leaf.clone(),
    };

    let leaf_21: Cube = Cube {
        center: Vec3::new(1.0, 5.0, 5.0),
        length: 1,
        material: leaf.clone(),
    };

    let leaf_22: Cube = Cube {
        center: Vec3::new(2.0, 5.0, 5.0),
        length: 1,
        material: leaf.clone(),
    };

    let leaf_23: Cube = Cube {
        center: Vec3::new(0.0, 5.0, 4.0),
        length: 1,
        material: leaf.clone(),
    };

    let leaf_24: Cube = Cube {
        center: Vec3::new(1.0, 5.0, 4.0),
        length: 1,
        material: leaf.clone(),
    };

    let leaf_25: Cube = Cube {
        center: Vec3::new(2.0, 5.0, 4.0),
        length: 1,
        material: leaf.clone(),
    };

    let lava_cube: Cube = Cube {
        center: Vec3::new(1.0, 0.0, 2.0),
        length: 1,
        material: lava.clone(),
    };

    let water_cube: Cube = Cube {
        center: Vec3::new(3.0, 0.0, 1.0),
        length: 1,
        material: water.clone(),
    };

    let chest : Cube = Cube {
        center: Vec3::new(4.0, 1.0, 1.0),
        length: 1,
        material: chest_tetxure.clone(),
    };

    let objects: Vec<&dyn Renderable> = vec![
        &cube_0_0_0,
        &cube_0_0_1,
        &cube_0_0_2,
        &cube_0_0_3,
        &cube_0_0_4,
        &cube_0_0_5,
        &cube_1_0_0,
        &cube_1_0_3,
        &cube_1_0_4,
        &cube_1_0_5,
        &cube_2_0_0,
        &cube_2_0_2,
        &cube_2_0_3,
        &cube_2_0_4,
        &cube_2_0_5,
        &cube_3_0_0,
        &cube_3_0_2,
        &cube_4_0_0,
        &cube_4_0_1,
        &cube_4_0_2,
        &cube_5_0_0,
        &cube_5_0_1,
        &cube_5_0_2,
        &cube_0_m1_0,
        &cube_0_m1_1,
        &cube_0_m1_2,
        &cube_0_m1_3,
        &cube_0_m1_4,
        &cube_0_m1_5,
        &cube_1_m1_0,
        &cube_1_m1_5,
        &cube_2_m1_0,
        &cube_2_m1_3,
        &cube_2_m1_3,
        &cube_2_m1_4,
        &cube_2_m1_5,
        &cube_3_m1_0,
        &cube_3_m1_2,
        &cube_4_m1_0,
        &cube_4_m1_2,
        &cube_5_m1_0,
        &cube_5_m1_1,
        &cube_5_m1_2,
        &cube_1_m1_2,
        &cube_2_m1_1,
        &cube_3_m1_1,
        &cobblestone_cube,
        &log_1,
        &log_2,
        &log_3,
        &leaf_1,
        &leaf_1_2,
        &leaf_2,
        &leaf_2_2,
        &leaf_3,
        &leaf_3_2,
        &leaf_4,
        &leaf_4_2,
        &leaf_5,
        &leaf_5_2,
        &leaf_6,
        &leaf_6_2,
        &leaf_7,
        &leaf_7_2,
        &leaf_8,
        &leaf_8_2,
        &leaf_9,
        &leaf_9_2,
        &leaf_10,
        &leaf_10_2,
        &leaf_11,
        &leaf_11_2,
        &leaf_12,
        &leaf_12_2,
        &leaf_13,
        &leaf_13_2,
        &leaf_10,
        &leaf_10_2,
        &leaf_11,
        &leaf_11_2,
        &leaf_12,
        &leaf_12_2,
        &leaf_13,
        &leaf_13_2,
        &leaf_14,
        &leaf_14_2,
        &leaf_15,
        &leaf_15_2,
        &leaf_16,
        &leaf_16_2,
        &leaf_17,
        &leaf_18,
        &leaf_19,
        &leaf_20,
        &leaf_21,
        &leaf_22,
        &leaf_23,
        &leaf_24,
        &leaf_25,
        &water_cube,
        &lava_cube,
        &chest,
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