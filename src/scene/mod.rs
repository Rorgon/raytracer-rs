pub mod camera;
pub mod sphere; 

use crate::scene::camera::Camera;
use crate::vec3::{Vec3};
use crate::ray::{Ray, hit::{Hittable}};
use crate::image::{Image};
use rand::prelude::*;
use rayon::prelude::*;

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Box<dyn Hittable>>
}

impl Scene {
    pub fn new(camera: Camera) -> Scene {
        Scene {camera,objects: Vec::new()}
    }

    pub fn render(&self, width: usize, height: usize, samples_per_pixel: u64, max_depth : u64) -> Image{
        
        let mut image= Image::new(width, height, Vec3(0.0,0.0,0.0));
        image.pixels_mut()
             .par_chunks_exact_mut(width).enumerate().for_each(
            |(i,row)| row.par_iter_mut().enumerate().for_each(
                |(j,pixel)| {

                    let mut color = Vec3(0.0,0.0,0.0);
                    for _sample in 0..samples_per_pixel {
                        let mut rng = rand::thread_rng();

                        let u = (i as f64 + rng.gen::<f64>() as f64) / (width as f64);
                        let v = (j as f64 + rng.gen::<f64>()) / (height as f64);

                        let r = self.camera.get_ray(u,v);

                        color = color + r.color(&self.objects, max_depth);

                    }
                    color.0 = (color.0 * (1.0/ samples_per_pixel as f64)).sqrt();
                    color.1 = (color.1 * (1.0/ samples_per_pixel as f64)).sqrt();
                    color.2 = (color.2 * (1.0/ samples_per_pixel as f64)).sqrt();
                    *pixel= color
                })
            ); 
        image
    }

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }
}
