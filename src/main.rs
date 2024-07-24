extern crate nalgebra_glm as glm;

mod framebuffer;
mod line;
mod bmp;
mod polygon1;

use crate::framebuffer::Framebuffer;
use crate::polygon1::Polygon;
use crate::bmp::WriteBmp;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    framebuffer.set_current_color(0xFFFFFF);


    framebuffer.polygon(&[(165, 380), (185, 360), (180, 330), (207, 345), (233, 330), (230, 360), (250, 380), (220, 385), (205, 410), (193, 383)]);
    framebuffer.polygon(&[(321, 335), (288, 286), (339, 251), (374, 302)]);
 

    let _ = framebuffer.render_buffer("output.bmp");
}