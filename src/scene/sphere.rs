use crate::vec3::Vec3;
use crate::ray::{Ray, hit::{Hittable,HitRecord}, material::Material};
use std::rc::Rc;

pub struct Sphere {
    pub center : Vec3,
    pub radius : f64,
    pub material : Rc<dyn Material>
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Rc<dyn Material>) -> Sphere {
        Sphere{ radius, center, material}
    }
}

impl Hittable for Sphere {

    fn hit(&self, ray: &Ray, interval: (f64,f64)) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(oc,ray.direction);
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminant = half_b*half_b-a*c;

        let same_side = |x: Vec3, y: Vec3| Vec3::dot(x,y)<0.0;
        
        

        if discriminant > 0.0{
            let root = discriminant.sqrt();
            let time = (-half_b - root)/a;
            if interval.0 < time && time < interval.1 {
                let point = ray.point_at_parameter(time);
                let outward_normal = (point - self.center)/ self.radius;
                let front_face = same_side(ray.direction, outward_normal);
                let normal = if front_face { outward_normal } else { - outward_normal };

                return Some(HitRecord::new( point, normal, time, front_face, self.material.clone()) )
            };
            let time = (-half_b + root)/a;
            if interval.0 < time && time < interval.1 {
                let point = ray.point_at_parameter(time);
                let outward_normal = (point - self.center)/ self.radius;
                let front_face = same_side(ray.direction, outward_normal);
                let normal = if front_face { outward_normal } else { - outward_normal };

                return Some(HitRecord::new( point, normal, time, front_face, self.material.clone() ))
            };
        }; 
        None
    }
}
