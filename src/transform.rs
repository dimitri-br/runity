use crate::{Vector3, Quaternion};
/// # Transform
/// 
/// This struct represents a transform in unity. It will aim, like the `GameObject` struct,
/// to implement as many unity-specific functions as possible. This will likely be done through
/// function pointers as it is cheaper to run most (such as child searching) through pointers rather than
/// natively. However, some, such as `translate` will be natively implemented.
#[repr(C)]
#[derive(Clone)]
pub struct Transform{
    pub position: Vector3,
    pub rotation: Quaternion,
}

impl Transform{
    /// # New
    ///
    /// Creates a new `Transform` struct from a position (`Vector3`)
    pub fn new(position: Vector3, rotation: Quaternion) -> Self{
        Self{
            position,
            rotation
        }
    }
}