use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::image::Image;

pub fn save_image<P: AsRef<Path>>(image: &Image, path: P) {
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, image.width as u32, image.height as u32);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    let mut buffer: Vec<u8> = Vec::with_capacity(image.width * image.height * 4);

    for pix in image.pixels.iter() {
        buffer.push(pix.r);
        buffer.push(pix.g);
        buffer.push(pix.b);
        buffer.push(pix.alpha);
    }

    writer.write_image_data(buffer.as_slice()).unwrap();
}
