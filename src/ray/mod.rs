pub mod hit;
pub mod material;

use crate::vec3::Vec3;
use hit::{Hittable, HitRecord};
use material::ScatterInfo;

#[derive(Debug,Copy,Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
    
    pub fn color(&self, objects: &Vec<Box<dyn Hittable>>, depth: u64) -> Vec3 {

        if depth == 0 {
            return Vec3(0.0,0.0,0.0)
        }

        let mut closest_hit : Option<HitRecord> = None;
        for obj in objects {
            let hit = obj.hit(self, (0.0,std::f64::INFINITY) );
            closest_hit = match &hit {
                Some(record) => match &closest_hit {
                    Some(closest_record)=> if record.time < closest_record.time { hit } else { closest_hit },
                    None => hit
                }
                None => closest_hit
            };
        }
         
        let t = 0.5 * (self.direction.unit_vector().y() + 1.0);
        match closest_hit {
            Some(record) => match record.material.scatter(self, record.point, record.normal) {
                Some(ScatterInfo(scatter, attenuation)) => attenuation * scatter.color(objects, depth-1),
                None => Vec3(0.0,0.0,0.0)
            }
            None => Vec3(1.0,1.0,1.0) * (1.0-t)  + Vec3(0.5,0.7,1.0) * t

        }
    }
}


