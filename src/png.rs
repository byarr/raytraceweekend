use std::io::Write;
use crate::{Colour, Vec3};

pub fn write_png<W: Write>(w: &mut W, data: &[Colour], width: u32, heigth: u32) -> std::io::Result<()> {

    let mut encoder = png::Encoder::new(w, width, heigth); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    let pixel_data = color_to_bytes(data);
    writer.write_image_data(&pixel_data)?;
    writer.finish()?;
    Ok(())
}

fn color_to_bytes(data: &[Colour]) -> Vec<u8> {
    let mut result = Vec::with_capacity(data.len() * 3);
    data.iter().for_each(|c| {
        result.push((c.r() * 255.0) as u8);
        result.push((c.g() * 255.0) as u8);
        result.push((c.b() * 255.0) as u8);});
    result
}

