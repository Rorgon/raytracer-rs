use crate::vec3::Vec3;
use crate::scene::sphere::Sphere;

#[derive(Debug,Copy,Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin: origin, direction: direction }
    }

    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn color(&self, objects: &Vec<Box<dyn Hittable>>) -> Vec3 {
        let sphere = Sphere {origin : Vec3(0.0,0.0,-1.0), radius: 0.5 , color : Vec3(1.0,0.0,0.0)};

        if sphere.hit(self) {
             return sphere.color
        }

        let t = 0.5 * (self.direction.unit_vector().y() + 1.0);
        Vec3(1.0,1.0,1.0) * (1.0-t)  + Vec3(0.5,0.7,1.0) * t
    }
}

pub trait Hittable {
    fn hit(&self, ray : &Ray) -> bool;
}
