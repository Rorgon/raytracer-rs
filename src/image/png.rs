use std::fs;

use crate::image::Image;
use image::png::PNGEncoder;
use image::ColorType;

pub fn write(image: &Image, filename: &str) -> image::error::ImageResult<()> {
    let mut buffer = fs::File::create(filename).unwrap();

    let png = PNGEncoder::new(buffer);

    let mut data :Vec<u8> = Vec::new();

    for p in &image.pixels {
        data.push((p.r()*255.99) as u8);
        data.push((p.g()*255.99) as u8);
        data.push((p.b()*255.99) as u8);
    }

    png.encode(&data,image.width() as u32,image.height() as u32,ColorType::Rgb8)
}
