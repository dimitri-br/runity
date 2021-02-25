use crate::Math;

#[repr(C)]
pub struct Vector3{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3{
    /// # New
    ///
    /// Creates a new `Vector3` from an `x`, `y` and `z`
    pub fn new(x: f32, y: f32, z: f32) -> Self{
        Self{
            x,
            y,
            z
        }
    }

    /* Functions to help with Vector3 usage */

    /// # Translate
    ///
    /// Translates `a` vector by `b` vector
    pub fn translate(a: Self, b: Self) -> Self{
        Self::new(a.x + b.x, a.y + b.y, a.z + b.z)
    }

    /// # Lerp
    ///
    /// Linearly interpolate between start `Vector3` and end `Vector3` across `t`.
    pub fn lerp(start: Self, end: Self, t: f32) -> Self{
        Self::new(
            Math::lerp(start.x, end.x, t),
            Math::lerp(start.y, end.y, t),
            Math::lerp(start.z, end.z, t),
        )
    }
}