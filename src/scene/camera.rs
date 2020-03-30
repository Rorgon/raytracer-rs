
use crate::ray::{Hittable};
use crate::vec3::Vec3;

pub struct Camera {
    pub vfov: f64,
    pub aspect: f64,
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub look_up: Vec3
}

impl Camera {
    pub fn new( look_from: Vec3, look_at: Vec3, look_up: Vec3, vfov: f64, aspect: f64) -> Camera {
        Camera { look_from, look_at, look_up, vfov, aspect }
    }

    pub fn origin(&self) -> Vec3 {
        self.look_from
    }

    pub fn upper_left_corner(&self) -> Vec3 {
        let half_height = (self.vfov/2.0).tan();
        let half_width = half_height * self.aspect;
        
        Vec3(-half_width, half_height, -1.0)
    }
    
    pub fn horizontal(&self) -> Vec3 {
        let half_width = (self.vfov/2.0).tan() * self.aspect;
        Vec3(2.0*half_width, 0.0, 0.0)
    }

    pub fn vertical(&self) -> Vec3 {
        let half_height = (self.vfov/2.0).tan();
        Vec3(0.0,2.0*half_height, 0.0)
    }
}
