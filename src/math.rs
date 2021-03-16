use num::{Float, Integer};

use crate::Vector3;

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
        // We use Into<T> so we can convert primitives into T, which
        // is type num::Float
        start * (1.0.into() - t) + end * t
    }

    /// # Log
    ///
    /// Returns the log of `f` to the power of `p`.
    pub fn log<T>(f: T, p: T) -> T where T: Float, f32: Into<T>{
        f.log(p)
    }

    /// # Abs
    ///
    /// Returns the absolute value of `f`
    pub fn abs<T>(f: T) -> T where T: Float{
        f.abs()
    }

    /// # Acos
    ///
    /// Returns the arc-cosine of `f` - the angle in radians whose cosine is `f`.
    pub fn acos<T>(f: T) -> T where T: Float{
        f.acos()
    }

    /// # Asin
    ///
    /// Returns the arc-sine of `f` - the angle in radians whose sine is `f`.
    pub fn asin<T>(f: T) -> T where T: Float{
        f.asin()
    }

    /// # Atan
    ///
    /// Returns the arc-tangent of `f` - the angle in radians whose tangent is `f`.
    pub fn atan<T>(f: T) -> T where T: Float{
        f.atan()
    }

    /// # Atan2
    ///
    /// Returns the angle in radians whose Tan is `y/x`.
    pub fn atan2<T>(f: T, o: T) -> T where T: Float{
        f.atan2(o)
    }

    /// # Ceil
    ///
    /// Returns the smallest integer greater to or equal to `f`.
    pub fn ceil<T>(f: T) -> T where T: Float{
        f.ceil()
    }

    /// # Ceil
    ///
    /// Returns the smallest integer greater to or equal to `f`.
    pub fn ceil_to_int<T, U>(f: T) -> U where T: Float, U: Integer, T: Into<U>{
        f.ceil().into()
    }

    /// # Clamp
    ///
    /// Clamp `val` between `min` and `max`
    pub fn clamp<T>(val: T, min: T, max: T) -> T where T: Float, f32: Into<T>{
        if val < min{
            return min;
        }
        else if val > max{
            return max;
        }

        return val;
    }

    /// # Clamp01
    ///
    /// Clamp `val` between 0 and 1
    pub fn clamp01<T>(f: T) -> T where T: Float, f32: Into<T>{
        if f < 0.0.into(){
            return 0.0.into();
        }
        else if f > 1.0.into(){
            return 1.0.into();
        }

        return f;
    }

    /// # ClosestPowerOfTwo
    ///
    /// Returns the closest power of two value `f`.
    pub fn closest_power_of_two<T>(f: T) -> T where T: Float, f32: Into<T>{
        f.log(2.0.into()) + 1.0.into()
    }

    /// # Correlated Color Temperature to RGB
    ///
    /// Convert a color temperature in Kelvin to RGB color.
    ///
    /// Given a correlated color temperature (in Kelvin), estimate the RGB equivalent. Curve fit error is max 0.008.
    ///
    /// Correlated color temperature is defined as the color temperature of the electromagnetic radiation emitted from an ideal black body with its surface temperature given in degrees Kelvin.
    ///
    /// Temperature must fall between 1000 and 40000 degrees.
    pub fn correlated_color_temperature_to_rgb(temperature: f32) -> Vector3{
        let kelvin: f32 = Math::clamp(temperature, 1000.0, 40000.0) / 1000.0;
        let kelvin2 = kelvin * kelvin;

        // Using 6570 as a pivot is an approximation, pivot point for red is around 6580 and for blue and green around 6560.
        // Calculate each color in turn (Note, clamp is not really necessary as all value belongs to [0..1] but can help for extremum).
        // Red
        let r = if kelvin < 6.570  {1.0} else {Math::clamp((1.35651 + 0.216422 * kelvin + 0.000633715 * kelvin2) / (-3.24223 + 0.918711 * kelvin), 0.0, 1.0)};
        // Green
        let g = if kelvin < 6.570 
            {Math::clamp((-399.809 + 414.271 * kelvin + 111.543 * kelvin2) / (2779.24 + 164.143 * kelvin + 84.7356 * kelvin2), 0.0, 1.0)}
            else {Math::clamp((1370.38 + 734.616 * kelvin + 0.689955 * kelvin2) / (-4625.69 + 1699.87 * kelvin), 0.0, 1.0)};
        //Blue
        let b = if kelvin > 6.570  {1.0} else { Math::clamp((348.963 - 523.53 * kelvin + 183.62 * kelvin2) / (2848.82 - 214.52 * kelvin + 78.8614 * kelvin2), 0., 1.0) };



        Vector3::new(r, g, b)
    }

    /// # Cos
    ///
    /// Returns the cosine of angle `f`.
    pub fn cos<T>(f: T) -> T where T: Float, f32: Into<T>{
        f.cos()
    }
}