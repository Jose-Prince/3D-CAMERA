// material

use crate::color::Color;
use crate::texture::Texture;

#[derive(Debug, Clone)]
pub struct Material {
    pub diffuse: Color,
    pub specular: f32,
    pub albedo: [f32; 4],
    pub refractive_index: f32,
    pub texture: Option<Texture>,
}

impl Material {
    pub fn new(
        diffuse: Color,
        specular: f32,
        albedo: [f32; 4],
        refractive_index: f32,
        texture_path: Option<&str>
    ) -> Self {
        let texture = texture_path.map(|path| Texture::new(path));

        Material {
            diffuse,
            specular,
            albedo,
            refractive_index,
            texture,
        }
    }

    pub fn black() -> Self {
        Material {
            diffuse: Color::new(0,0,0),
            specular: 0.0,
            albedo: [0.0, 0.0, 0.0, 0.0],
            refractive_index: 1.0,
            texture: None,
        }
    }
}