use crate::vec3::Vec3;
use std::io::Write;

pub type Color = Vec3;

// Accept any type that implements the Write trait
pub fn write_color(out: &mut impl Write, pixel_color: Color) {
    let r = (255.999 * pixel_color.x()) as u8;
    let g = (255.999 * pixel_color.y()) as u8;
    let b = (255.999 * pixel_color.z()) as u8;

    writeln!(out, "{} {} {}", r, g, b).expect("writing color");
}
