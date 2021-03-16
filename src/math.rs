use num::Float;

/// # Math
///
/// Helpful mathematical functions, similar to `Mathf` in unity.
///
/// The aim is to reimplement every function in `Mathf` in native rust,
/// to avoid the overhead of function pointers. These functions usually 
/// don't require the UnityEngine API anyways.
pub struct Math;

impl Math{
    /// # Lerp
    ///
    /// Linerarly interpolate between `start` and `end` across `t`
    pub fn lerp<T>(start: T, end: T, t: T) -> T where T: Float, f32: Into<T>{
        start * (1.0.into() - t) + end * t
    }

    /// # Clamp
    ///
    /// Clamp `val` between `min` and `max`
    pub fn clamp<T>(val: T, min: T, max: T) -> T where T: Float{
        if val < min{
            return min;
        }
        else if val > max{
            return max;
        }

        return val;
    }

    /// # Log
    ///
    /// Returns the log of `f` to the power of `p`.
    pub fn log<T>(f: T, p: T) -> T where T: Float, f32: Into<T>{
        f.log(p)
    }
}