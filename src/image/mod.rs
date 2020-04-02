pub mod ppm;
pub mod png;
use crate::vec3::Vec3;
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


    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
     
