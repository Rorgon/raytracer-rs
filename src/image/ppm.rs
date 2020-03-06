use std::fs;

use crate::image::Image;

pub fn write(image: &Image, filename: &str) -> std::io::Result<()> {
    let mut s = format!("P3\n{} {}\n255\n",image.width(),image.height());
    
    for p in &image.pixels {
        s.push_str(&format!("{} {} {}\n", (p.r()*255.99) as u8, (p.g()*255.99) as u8, (p.b() *255.99) as u8 ) );
    };

    fs::write(filename, s.as_bytes())
}
