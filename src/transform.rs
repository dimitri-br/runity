use crate::Vector3;

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