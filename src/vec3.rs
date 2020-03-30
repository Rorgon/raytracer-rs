use core::ops::{Add, Mul, Div, Sub, Neg};
use std::convert::From;


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3(pub f64,pub f64,pub f64);

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3(self.x() + other.x(), self.y() + other.y(), self.z() + other.z())
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3(self.x() - other.x(), self.y() - other.y(), self.z() - other.z())
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(self.x() * other.x(), self.y() * other.y(), self.z() * other.z())
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3(self.x() / other.x(), self.y() / other.y(), self.z() / other.z())
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3(self.x() * other, self.y() * other, self.z() * other)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3(self.x() / other, self.y() / other, self.z() / other)
    }
}


impl From<(f64, f64, f64)> for Vec3 {
    
    fn from(v : (f64, f64, f64)) -> Self {
        Vec3(v.0, v.1, v.2)
    }
}

impl Vec3 {
    
    pub fn all_zero() -> Vec3 {
        Vec3(0.0,0.0,0.0)
    }

    #[inline]
    pub fn x(&self) -> f64 { self.0 }
    #[inline]
    pub fn y(&self) -> f64 { self.1 }
    #[inline]
    pub fn z(&self) -> f64 { self.2 }
    #[inline]
    pub fn r(&self) -> f64 { self.0 }
    #[inline]
    pub fn g(&self) -> f64 { self.1 }
    #[inline]
    pub fn b(&self) -> f64 { self.2 }

    pub fn dot(lhs: Self, rhs: Self) -> f64 {
        lhs.0*rhs.0+lhs.1*rhs.1+lhs.2*rhs.2
    }

    pub fn cross(lhs: Self, rhs: Self) -> Self {
        Vec3(
            lhs.1*rhs.2 - lhs.2*rhs.1 ,
            lhs.2*rhs.0 - lhs.0*rhs.2 ,
            lhs.0*rhs.1 - lhs.1*rhs.0
        )
    }

    pub fn length(&self) -> f64 {
        (self.0*self.0+self.1*self.1+self.2*self.2).sqrt()
    }

    pub fn squared_length(&self) -> f64 {
        self.0*self.0+self.1*self.1+self.2*self.2
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }
}
