use raytracer::image::{Image, png};
use raytracer::vec3::Vec3;
use raytracer::scene::{Scene, sphere::Sphere, camera::Camera};
use std::error::Error;
use core::f64::consts::PI;


fn main() -> Result<(), Box<dyn Error> > {
    
    let height: usize = 400;
    let width: usize = 800;

    let vfov: f64 = PI/2.0;
    let aspect: f64 = (width as f64)/(height as f64);
    let look_from: Vec3 = Vec3(0.0,0.0,0.0);
    let look_to: Vec3 = Vec3(0.0,0.0,-1.0);
    let look_up: Vec3 = Vec3(0.0,1.0,0.0);
    let mut scene = Scene::new(Camera::new(look_from,look_to,look_up,vfov,aspect));
    
    scene.add(Box::new(Sphere::new(Vec3(0.0,0.0,-1.0),2.0,Vec3(1.0,0.0,0.0))));

    let image = scene.render(width, height);
    png::write(&image, "test.png")?;

    Ok(())
}
