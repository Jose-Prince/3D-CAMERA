use crate::color::Color;
use crate::texture::Texture;
use std::sync::Arc;

#[derive(Debug, Clone)] 
pub struct Material {
    pub diffuse: Color,
    pub specular: f32,
    pub albedo: [f32; 4],
    pub refractive_index: f32,
    pub texture: Option<Arc<Texture>>, // Option permite que sea None o Some(Texture)
    pub has_texture: bool,
}

impl Material {
    pub fn new(
        diffuse: Color,
        specular: f32,
        albedo: [f32; 4],
        refractive_index: f32,
        texture: Option<Arc<Texture>>,
        has_texture: bool,
    ) -> Self {
        Material {
            diffuse,
            specular,
            albedo,
            refractive_index,
            texture,
            has_texture,
        }
    }

    pub fn get_texture(&self) -> Option<Arc<Texture>> {
        self.texture.clone()
    }

    pub fn black() -> Self {
        Material {
            diffuse: Color::new(0, 0, 0),
            specular: 0.0,
            albedo: [0.0, 0.0, 0.0, 0.0],
            refractive_index: 1.0,
            texture: None,
            has_texture: false,
        }
    }

    pub fn get_diffuse_color(&self, u: f32, v: f32) -> Color {
        if self.has_texture {
            if let Some(ref texture) = self.texture {
                let x = u * (texture.width as f32 - 1.0);
                let y = v * (texture.height as f32 - 1.0);
                texture.get_color(x as usize, y as usize)
            } else {
                self.diffuse
            }
        } else {
            self.diffuse
        }
    }
}
