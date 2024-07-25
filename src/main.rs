extern crate nalgebra_glm as glm;

mod framebuffer;
mod line;
mod bmp;
mod polygon1;

use glm::Vec3;

use crate::framebuffer::Framebuffer;
use crate::polygon1::Polygon;
use crate::bmp::WriteBmp;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    let points1 = vec![
        Vec3::new(165.0, 380.0, 0.0),
        Vec3::new(185.0, 360.0, 0.0),
        Vec3::new(180.0, 330.0, 0.0),
        Vec3::new(207.0, 345.0, 0.0),
        Vec3::new(233.0, 330.0, 0.0),
        Vec3::new(230.0, 360.0, 0.0),
        Vec3::new(250.0, 380.0, 0.0),
        Vec3::new(220.0, 385.0, 0.0),
        Vec3::new(205.0, 410.0, 0.0),
        Vec3::new(193.0, 383.0, 0.0),
    ];
    
    //orilla P1
    framebuffer.set_current_color(0xFFFFFF);
    framebuffer.polygon(&[(165, 380), (185, 360), (180, 330), (207, 345), (233, 330), (230, 360), (250, 380), (220, 385), (205, 410), (193, 383)]);

    //relleno p1
    framebuffer.set_fill_color(0x00FFFF);
    framebuffer.filled_polygon(&points1);

    let points2 = vec![
        Vec3::new(321.0, 335.0, 0.0),
        Vec3::new(288.0, 286.0, 0.0),
        Vec3::new(339.0, 251.0, 0.0),
        Vec3::new(374.0, 302.0, 0.0)
    ];

    framebuffer.set_current_color(0xFFFFFF);
    framebuffer.polygon(&[(321, 335), (288, 286), (339, 251), (374, 302)]);
    framebuffer.set_fill_color(0x0000FF);
    framebuffer.filled_polygon(&points2);

    let _ = framebuffer.render_buffer("output.bmp");
}