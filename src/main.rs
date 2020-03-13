use raytracer::image::{Image, png};
use raytracer::vec3::Vec3;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error> > {
    let mut image = Image::new(800,400,Vec3(1.0,0.0,0.0));
    image.render();
    png::write(&image, "test.png")?;

    Ok(())
}
