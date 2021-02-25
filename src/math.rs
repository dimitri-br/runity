/// # Math
///
/// Helpful mathematical functions, similar to `Mathf` in unity.
pub struct Math;

impl Math{
    /// # Lerp
    ///
    /// Linerarly interpolate between `start` and `end` across `t`
    pub fn lerp(start: f32, end: f32, t: f32) -> f32{
        start * (1.0 - t) + end * t
    }
}