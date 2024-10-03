use crate::color::Color;
use crate::texture::Texture;
use std::sync::Arc;

#[derive(Debug, Clone)] 
pub struct Material {
    pub diffuse: Color,
    pub specular: f32,
    pub albedo: [f32; 4],
    pub refractive_index: f32,
    pub textures: [Option<Arc<Texture>>; 6], 
    pub has_texture: bool,
}

impl Material {
    pub fn new(
        diffuse: Color,
        specular: f32,
        albedo: [f32; 4],
        refractive_index: f32,
        textures: [Option<Arc<Texture>>; 6],
        has_texture: bool,
    ) -> Self {
        Material {
            diffuse,
            specular,
            albedo,
            refractive_index,
            textures,
            has_texture,
        }
    }

    pub fn get_texture_for_face(&self, face_index: usize) -> Option<Arc<Texture>> {
        if face_index < 6 {
            self.textures[face_index].clone()
        } else {
            None
        }
    }

    pub fn black() -> Self {
        Material {
            diffuse: Color::new(0, 0, 0),
            specular: 0.0,
            albedo: [0.0, 0.0, 0.0, 0.0],
            refractive_index: 1.0,
            textures: [None,None,None,None,None,None],
            has_texture: false,
        }
    }

    pub fn get_diffuse_color(&self, face_index: usize, u: f32, v: f32) -> Color {
        if self.has_texture {
            if let Some(ref texture) = self.get_texture_for_face(face_index) {
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
