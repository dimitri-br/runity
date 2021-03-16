use crate::Vector3;
/// # Transform
/// 
/// This struct represents a transform in unity. It will aim, like the `GameObject` struct,
/// to implement as many unity-specific functions as possible. This will likely be done through
/// function pointers as it is cheaper to run most (such as child searching) through pointers rather than
/// natively. However, some, such as `translate` will be natively implemented.
#[repr(C)]
pub struct Transform{
    pub position: Vector3,
}

impl Transform{
    /// # New
    ///
    /// Creates a new `Transform` struct from a position (`Vector3`)
    pub fn new(position: Vector3) -> Self{
        Self{
            position
        }
    }
}