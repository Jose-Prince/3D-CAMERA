// main.rs

mod framebuffer;
mod color;
mod fileReader;
mod bmp;

use framebuffer::Framebuffer;
use color::Color;

fn main() {
    let width = 500;  // Ajusta el tamaño del framebuffer según sea necesario
    let height = 800; // Ajusta el tamaño del framebuffer según sea necesario
    let mut framebuffer = Framebuffer::new(width, height);

}
