/// # Quaternion
///
/// This module provides access to various functions and
/// methods used for quaternions in unity.


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