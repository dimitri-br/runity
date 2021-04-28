/// # Quaternion
///
/// This module provides access to various functions and
/// methods used for quaternions in unity.

use std::ops::{Mul, MulAssign};

use crate::{Math, Vector3};

/// # Quaternion
///
/// This struct reimplements the `Quaternion` in unity.
///
/// Quaternions are used to represent rotations, with the added
/// benefit that they don't suffer from gimbal lock. This struct
/// provides a safe way to interface with rotations in unity,
/// and has many quaternion related functions to help.
///
/// It takes four `f32`'s:
///
/// - `x`
/// 
/// - `y`
///
/// - `z`
///
/// - `w`
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
        let x = x * Math::DEG2RAD;
        let y = y * Math::DEG2RAD;
        let z = z * Math::DEG2RAD;

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

        let x = x * Math::RAD2DEG;
        let y = y * Math::RAD2DEG;
        let z = z * Math::RAD2DEG;

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

impl MulAssign for Quaternion{
    fn mul_assign(&mut self, rhs: Self) {
        self.w = -rhs.x * self.x - rhs.y * self.y - rhs.z * self.z + rhs.w * self.w;
        self.x = rhs.x * self.w + rhs.y * self.z - rhs.z * self.y + rhs.w * self.x;
        self.y = -rhs.x * self.z + rhs.y * self.w + rhs.z * self.x + rhs.w * self.y;
        self.z = rhs.x * self.y - rhs.y * self.x + rhs.z * self.w + rhs.w * self.z;
    }
}