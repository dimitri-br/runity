use crate::Math;

/// # Vector3
/// 
/// This struct reimplements the Vector3 class as a struct in rust.
/// It inherits its structure from C as this data is able to be passed from c# into
/// rust and vice versa through FFI (It does this by being stored in the `transform` struct,
/// which is stored in the `DataStruct` struct which is what is being passed around).
/// It aims to implement as many functions possible from
/// c# so rust will be able to get maximal performance and compatibility with
/// unity.
///
/// It uses almost exclusively `f32` as that is the standard unit of measurement in
/// unity. 
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
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
}

/* Functions to help with Vector3 usage */
impl Vector3{

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

    pub fn normalize(value: Self) -> Self{
        let magnitude: f32 = Vector3::magnitude(value);
        Self::new(
            value.x / magnitude,
            value.y / magnitude,
            value.z / magnitude
        )
    }

    pub fn magnitude(value: Self) -> f32{
        (value.x * value.x + value.y * value.y).sqrt()
    }

    pub fn sqr_magnitude(value: Self) -> f32{
        value.x * value.x + value.y * value.y
    }
}

/* Static properties (such as Up, Down etc) */
impl Vector3{
    pub fn back() -> Self{
        Self::new(0.0, 0.0, -1.0)
    }
    pub fn forward() -> Self{
        Self::new(0.0, 0.0, 1.0)
    }
    pub fn left() -> Self{
        Self::new(-1.0, 0.0, 0.0)
    }
    pub fn right() -> Self{
        Self::new(1.0, 0.0, 0.0)
    }
    pub fn down() -> Self{
        Self::new(0.0, -1.0, 0.0)
    }
    pub fn up() -> Self{
        Self::new(0.0, 1.0, 0.0)
    }

    pub fn one() -> Self{
        Self::new(1.0, 1.0, 1.0)
    }
    pub fn zero() -> Self{
        Self::new(0.0, 0.0, 0.0)
    }
}
