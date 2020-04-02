
use crate::vec3::Vec3;
use crate::ray::Ray;

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

        let w = (self.look_from-self.look_at).unit_vector();
        let u = Vec3::cross(self.look_up,w).unit_vector();
        let v = Vec3::cross(w,u);

        self.look_from - u*half_width + v*half_height - w
    }
    
    pub fn horizontal(&self) -> Vec3 {
        let half_width = (self.vfov/2.0).tan() * self.aspect;

        let w = (self.look_from-self.look_at).unit_vector();
        let u = Vec3::cross(self.look_up,w).unit_vector();

        u*(2.0*half_width)
    }

    pub fn vertical(&self) -> Vec3 {
        let half_height = (self.vfov/2.0).tan();

        let w = (self.look_from-self.look_at).unit_vector();
        let u = Vec3::cross(self.look_up,w).unit_vector();
        let v = Vec3::cross(w,u);

        v*(2.0*half_height)
    }

    pub fn get_ray( &self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin() , self.upper_left_corner() + self.horizontal()*u + self.vertical()*(-v))
    }
}
