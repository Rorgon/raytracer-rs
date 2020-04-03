use crate::vec3::{Vec3};
use super::{material::Material, Ray};
use std::rc::Rc;


#[derive(Clone)]
pub struct HitRecord {
   pub point : Vec3,
   pub normal: Vec3,
   pub time : f64,
   pub front_face: bool,
   pub material: Rc<dyn Material>,
}

impl HitRecord {
   pub fn new(point: Vec3, normal: Vec3, time: f64, front_face: bool, material: Rc<dyn Material>) -> Self {
      Self { point, normal, time, front_face, material }
   }
}



pub trait Hittable {
    fn hit(&self, ray : &Ray, interval : (f64,f64)) -> Option<HitRecord>;
}
