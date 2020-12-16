use crate::vec3::Vec3;
use crate::ray::Ray;
use rand::prelude::*;
use std::f64::consts::PI;

pub struct ScatterInfo(pub Ray,pub Vec3);

pub trait Material : Sync + Send {
    fn scatter(&self, ray_in : Ray, point : Vec3, normal: Vec3) -> Option<ScatterInfo>;
}


fn random_unit_vector() -> Vec3 {
    let mut rng=rand::thread_rng();
    let a = rng.gen_range(0.0, 2.0*PI);
    let z = rng.gen_range(-1.0,1.0);
    let r = ((1.0-z*z) as f64).sqrt();
    Vec3(r*a.cos(), r*a.sin(),z)
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_vector(-1.0,1.0);
        if p.length_squared() < 1.0 {
            return p
        }
    }
}

#[derive(Copy,Clone,Debug)]
pub struct Lambertian {
    pub albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo :Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in : Ray, point: Vec3, normal: Vec3) -> Option<ScatterInfo> {
        let scatter_direction = normal + random_unit_vector();
        let scattered = Ray::new(point, scatter_direction);
        let attenuation = self.albedo;
        Some(ScatterInfo(scattered,attenuation))
    }
}

#[derive(Copy,Clone,Debug)]
pub struct Metallic {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metallic {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metallic {
        Metallic { albedo, fuzz }
    }
    
    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v-n*Vec3::dot(v,n)*2.0
    }
}

impl Material for Metallic {
    fn scatter(&self, ray_in: Ray, point: Vec3, normal: Vec3) -> Option<ScatterInfo> {
        let reflected = Self::reflect(ray_in.direction.unit_vector(), normal);
        let scattered = Ray::new(point, reflected+random_in_unit_sphere()*self.fuzz);
        let attenuation = self.albedo;
        match Vec3::dot(scattered.direction, normal) > 0.0 {
            true  => Some(ScatterInfo(scattered, attenuation)),
            false => None
        }
    }
}

