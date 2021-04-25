/// # Quaternion
///
/// This module provides access to various functions and
/// methods used for quaternions in unity.

use std::ops::{Add, Sub, Mul, Div};

use crate::{Math, Vector3};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Quaternion{
    x: f32,
    y: f32,
    z: f32,
    w: f32
}

impl Quaternion{
    /// # New
    ///
    /// Creates a new quaternion from `x`, `y`, `z`, and `w`
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self{
        Self {
            x,
            y,
            z,
            w
        }
    }

    /// # From Euler 
    ///
    /// Creates a new Quaternion from Euler angles (expects degrees).
    pub fn from_euler(x: f32, y: f32, z: f32) -> Self{
        let x = x * Math::deg2rad;
        let y = y * Math::deg2rad;
        let z = z * Math::deg2rad;

        let qx = (z/2.0).sin() * (y/2.0).cos() * (x/2.0).cos() - (z/2.0).cos() * (y/2.0).sin() * (x/2.0).sin();
        let qy = (z/2.0).cos() * (y/2.0).sin() * (x/2.0).cos() + (z/2.0).sin() * (y/2.0).cos() * (x/2.0).sin();
        let qz = (z/2.0).cos() * (y/2.0).cos() * (x/2.0).sin() - (z/2.0).sin() * (y/2.0).sin() * (x/2.0).cos();
        let qw = (z/2.0).cos() * (y/2.0).cos() * (x/2.0).cos() + (z/2.0).sin() * (y/2.0).sin() * (x/2.0).sin();
        
        Quaternion::new(qx, qy, qz, qw)
    }


    /// # To Euler
    ///
    /// Returns the Euler angles of this Quaternion (in degrees).
    pub fn to_euler(&self) -> Vector3{
        let (x, y, z, w) = (self.x, self.y, self.z, self.w);

        let t0 = 2.0 * (w * x + y * z);
        let t1 = 1.0 - 2.0 * (x * x + y * y);
        let z = Math::atan2(t0, t1);

        let mut t2 = 2.0 * (w * y - z * x);
        t2 = if t2 > 1.0 { 1.0 } else { t2 };
        t2 = if t2 < -1.0 { -1.0 } else { t2 };
        let y = Math::asin(t2);

        let t3 = 2.0 * (w * z + x * y);
        let t4 = 1.0 - 2.0 * (y * y + z * z);
        let x = Math::atan2(t3, t4);

        let x = x * Math::rad2deg;
        let y = y * Math::rad2deg;
        let z = z * Math::rad2deg;

        Vector3::new(x, y, z)
    }

    /// # Rotate
    ///
    /// Rotate Quaternion `self` by Quaternion `rhs`.
    pub fn rotate(&mut self, rhs: Self){
        
        self.w = -rhs.x * self.x - rhs.y * self.y - rhs.z * self.z + rhs.w * self.w;
        self.x = rhs.x * self.w + rhs.y * self.z - rhs.z * self.y + rhs.w * self.x;
        self.y = -rhs.x * self.z + rhs.y * self.w + rhs.z * self.x + rhs.w * self.y;
        self.z = rhs.x * self.y - rhs.y * self.x + rhs.z * self.w + rhs.w * self.z;
        
    }
}


impl Mul for Quaternion{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let w = -rhs.x * self.x - rhs.y * self.y - rhs.z * self.z + rhs.w * self.w;
        let x = rhs.x * self.w + rhs.y * self.z - rhs.z * self.y + rhs.w * self.x;
        let y = -rhs.x * self.z + rhs.y * self.w + rhs.z * self.x + rhs.w * self.y;
        let z = rhs.x * self.y - rhs.y * self.x + rhs.z * self.w + rhs.w * self.z;

        Self{
            x, 
            y, 
            z, 
            w
        }
    }
}