// texture.rs

use image::{DynamicImage, GenericImageView};
use crate::Color;

#[derive(Debug, Clone)]
pub struct Texture {
    pub image: DynamicImage,
}

impl Texture {
    pub fn new(path: &str) -> Self {
        let image = image::open(path).expect("Failed to load texture");
        Texture { image }
    }

    pub fn get_color(&self, u: f32, v: f32) -> Color {
        let (width, height) = self.image.dimensions();
        let x = (u * width as f32).clamp(0.0, (width - 1) as f32) as u32;
        let y = (v * height as f32).clamp(0.0, (height - 1) as f32) as u32;
    
        let pixel = self.image.get_pixel(x, y).0;
        Color::new(pixel[0] as i32, pixel[1] as i32, pixel[2] as i32)
    }    

}