// texture.rs

use image::{DynamicImage, GenericImageView};
use crate::Color;
use std::sync::Arc;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Texture {
    pub data: Arc<Vec<Color>>,
    pub width: usize,
    pub height: usize,
}

impl Texture {
    pub fn get_color(&self, x: usize, y: usize) -> Color {
        self.data[y * self.width + x]
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Arc<Self> {
        let img = image::open(path).expect("Failed to load texture");
        let img = img.to_rgb8();
        let (width, height) = img.dimensions();
        
        // Primero, crea un Vec<Color>
        let data: Vec<Color> = img
            .pixels()
            .map(|p| Color::new(p[0] as i32, p[1] as i32, p[2] as i32))  // Conversión de u8 a i32
            .collect(); // Esto devuelve un Vec<Color>
        
        // Luego, convierte el Vec<Color> a un Arc<Vec<Color>>
        Arc::new(Texture {
            data: Arc::new(data),
            width: width as usize,
            height: height as usize,
        })
    }
    
}
