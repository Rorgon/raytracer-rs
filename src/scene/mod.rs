pub mod camera;
pub mod sphere; 

use crate::scene::camera::{Camera};
use crate::vec3::{Vec3};
use crate::ray::{Ray, hit::{Hittable}};
use crate::image::{Image};

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Box<dyn Hittable>>
}

impl Scene {
    pub fn new(camera: Camera) -> Scene {
        Scene {camera,objects: Vec::new()}
    }

    pub fn render(&self, width: usize, height: usize) -> Image{
        
        let mut image= Image::new(width, height, Vec3(0.0,0.0,0.0));
        for j in 0..image.height() {
            for i in 0..image.width() {
                let u = (i as f64) / (image.width() as f64);
                let v = (j as f64) / (image.height() as f64);

                let r = self.camera.get_ray(u,v);
                
                image[(i as usize, j as usize)] = r.color(&self.objects);
            }
        }
        image
    }

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }
}
