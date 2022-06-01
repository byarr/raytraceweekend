use crate::Colour;
use std::io::Write;

pub fn write_png<W: Write>(
    w: &mut W,
    data: &[Colour],
    width: u32,
    heigth: u32,
    samples_per_pixel: u32,
) -> std::io::Result<()> {
    let mut encoder = png::Encoder::new(w, width, heigth); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    let pixel_data = color_to_bytes(data, samples_per_pixel);
    writer.write_image_data(&pixel_data)?;
    writer.finish()?;
    Ok(())
}

fn color_to_bytes(data: &[Colour], samples_per_pixel: u32) -> Vec<u8> {
    let mut result = Vec::with_capacity(data.len() * 3);
    data.iter().for_each(|c| {
        c.write_color(samples_per_pixel, &mut result);
    });
    result
}
