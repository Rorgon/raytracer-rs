use crate::vec3::{Vec3};
use super::Ray;


#[derive(Debug,Copy,Clone)]
pub struct HitRecord {
   pub point : Vec3,
   pub normal: Vec3,
   pub time : f64,
   pub front_face: bool
}

pub trait Hittable {
    fn hit(&self, ray : &Ray, interval : (f64,f64)) -> Option<HitRecord>;
}
