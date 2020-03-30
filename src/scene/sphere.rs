
use crate::vec3::Vec3;
use crate::ray::{Ray, Hittable};

pub struct Sphere {
    pub origin : Vec3,
    pub radius : f64,
    pub color : Vec3,
}

impl Sphere {
    pub fn new(origin: Vec3, radius: f64, color: Vec3) -> Sphere {
        Sphere{origin, radius, color}
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> bool {
        let oc = ray.origin - self.origin;
        let a = Vec3::dot(ray.direction,ray.direction);
        let b = Vec3::dot(oc,ray.direction)*2.0;
        let c = Vec3::dot(oc,oc) - self.radius*self.radius;
        let delta = b*b-4.0*a*c;
        delta>0.0
    }
}
