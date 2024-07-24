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
    framebuffer.polygon(&[(377, 249), (411, 197), (436, 249)]);
    framebuffer.polygon(&[
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37), (660, 52), (750, 145),
        (761, 179), (672, 192), (659, 214), (615, 214), (632, 230), (580, 230), (597, 215), (552, 214),
        (517, 144), (466, 180)
    ]);
    framebuffer.polygon(&[(682, 175), (708, 120), (735, 148), (739, 170)]); 

    let _ = framebuffer.render_buffer("output.bmp");
}