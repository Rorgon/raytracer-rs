pub mod ppm;

use crate::vec3::Vec3;
use crate::ray::{Ray};
use core::ops::{Index, IndexMut};

pub struct Image {
    width : usize,
    height : usize,
    pixels : Vec<Vec3>
}


impl Index<(usize,usize)> for Image {
    type Output = Vec3;

    fn index(&self, idx: (usize,usize)) -> &Vec3 {
        &self.pixels[idx.1 * self.width + idx.0]
    }
}

impl IndexMut<(usize,usize)> for Image {
    
    fn index_mut(&mut self, idx: (usize,usize)) -> &mut Vec3 {
        &mut self.pixels[idx.1 * self.width + idx.0]
    }
}

impl Image {
    pub fn new(width: usize, height: usize, color: Vec3) -> Image {
        Image {width, height, pixels: vec![color; height*width]}
    }

    pub fn render(&mut self){

        let origin = Vec3(0.0,0.0,0.0);
        let upper_left_corner = Vec3(-2.0,1.0,-1.0);
        let horizontal = Vec3(4.0,0.0,0.0);
        let vertical = Vec3(0.0,2.0,0.0);

        for j in 0..self.height {
            for i in 0..self.width {
                let u = (i as f64) / (self.width as f64);
                let v = (j as f64) / (self.height as f64);

                let r = Ray::new(origin , upper_left_corner + horizontal*u + vertical*(-v));

               

                self[(i,j)] = r.color();
            }
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
     