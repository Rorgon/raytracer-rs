use crate::vec3::{Vec3};
use super::{material::Material, Ray};
use std::sync::Arc;


#[derive(Clone)]
pub struct HitRecord {
   pub point : Vec3,
   pub normal: Vec3,
   pub time : f64,
   pub front_face: bool,
   pub material: Arc<dyn Material>,
}

impl HitRecord {
   pub fn new(point: Vec3, normal: Vec3, time: f64, front_face: bool, material: Arc<dyn Material>) -> Self {
      Self { point, normal, time, front_face, material }
   }
}



pub trait Hittable : Sync {
    fn hit(&self, ray : &Ray, interval : (f64,f64)) -> Option<HitRecord>;
}
