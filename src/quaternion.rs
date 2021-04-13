/// # Quaternion
///
/// This module provides access to various functions and
/// methods used for quaternions in unity.

use std::ops::{Add, Sub, Mul, Div};

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
    /// Creates a new Quaternion from Euler angles.
    pub fn from_euler(x: f32, y: f32, z: f32) -> Self{
        let x = y.sin() * z.sin() * z.cos() + y.cos() * z.cos() * x.sin();
        let y = y.sin() * z.cos() * z.cos() + y.cos() * z.sin() * x.sin();
        let z = y.cos() * z.sin() * z.cos() - y.sin() * z.cos() * x.sin();
        let w = y.cos() * z.cos() * z.cos() - y.sin() * z.sin() * x.sin();
        
        Quaternion::new(x, y, z, w)
    }
}

impl Add for Quaternion{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w
        }
    }
}

impl Sub for Quaternion{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w
        }
    }
}

impl Mul for Quaternion{
    type Output = Self;

    fn mul(self, b: Self) -> Self::Output {
        Self {
            x: self.w * b.w - self.x * b.x - self.y * b.y - self.z * b.z,  // 1
            y: self.w * b.x + self.x * b.w + self.y * b.z - self.z * b.y,  // i
            z: self.w * b.y - self.x * b.z + self.y * b.w + self.z * b.x,  // j
            w: self.w * b.z + self.x * b.y - self.y * b.x + self.z * b.w   // k
        }
    }
}

impl Div for Quaternion{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.x == 0.0 || rhs.y == 0.0 || rhs.z == 0.0 || rhs.w == 0.0{
            panic!("Can't divide by 0!")
        }

        if self.x == 0.0 || self.y == 0.0 || self.z == 0.0 || self.w == 0.0{
            panic!("Can't divide by 0!")
        }


        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w
        }
    }
}