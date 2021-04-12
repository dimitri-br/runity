use std::ops::BitAnd;

use num::{CheckedSub, Float, Integer};

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

    /// # Delta Angle
    ///
    /// Calculates the shortest difference between two given angles.
    pub fn delta_angle<T>(current: T, target: T) -> T where T: Float, f32: Into<T>
    {
        let mut delta = Math::repeat(target - current, 360.0.into());
        if delta > 180.0.into(){
            delta = delta.sub(360.0.into());
        }

        delta
    }

    /// # Exp
    ///
    /// Returns `e` raised to `f`
    pub fn exp<T>(f: T) -> T where T: Float, f32: Into<T>{
        f.exp()
    }

    /// # Float to half
    ///
    /// Return float `f` as a half.
    /// (Currently not working)
    pub fn float_to_half<T>(f: T) -> u32 where T: Float, f32: Into<T>, T: Into<u32>{
        f.into()
    }

    /// # Floor
    ///
    /// Returns the largest integer smaller than or equal to `f`.
    ///
    /// Note: This should be used in place of FloorToInt.
    pub fn floor<T>(f: T) -> T where T: Float, f32: Into<T>{
        f.floor()
    }

    /// # Gamma to Linear Space
    ///
    /// Converts the given value from gamma (sRGB) to linear color space.
    pub fn gamma_to_linear_space<T>(f: T) -> T where T: Float, f32: Into<T>{
        Math::pow(f, 2.2.into())
    }

    /// # Inverse Lerp
    ///
    /// Calculates the linear parameter t that produces the interpolant value within the range `[a, b]`.
    pub fn inverse_lerp<T>(start: T, end: T, value: T) -> T where T: Float, f32: Into<T>{
        (value - start) / (end - start)
    }

    pub fn is_power_of_two<T>(f: T) -> bool where T: Float, f32: Into<T>, T: BitAnd<Output = T>{
        (f != 0.0.into()) && ((f & (f - 1.0.into())) == 0.0.into())
    }

    /// # Lerp
    ///
    /// Linerarly interpolate between `start` and `end` across `t`
    pub fn lerp<T>(start: T, end: T, t: T) -> T where T: Float, f32: Into<T>{ 
        // We use Into<T> so we can convert primitives into T, which
        // is type num::Float
        start * (1.0.into() - t) + end * Math::clamp01(t)
    }

    /// # Lerp Angle
    ///
    /// Same as lerp, but makes sure `f` values iterpolate correctly when they wrap around 360 degrees
    pub fn lerp_angle<T>(start: T, end: T, t: T) -> T where T: Float, f32: Into<T>{ 
        let mut delta = Math::repeat(end - start, 360.0.into());
        if delta > 180.0.into(){
            delta = delta.sub(360.0.into());
            
        }
        start + delta * Math::clamp01(t)
    }

    /// # Lerp Unclamped
    ///
    /// Linerarly interpolate between `start` and `end` across `t`, with no limit to `t`
    pub fn lerp_unclamped<T>(start: T, end: T, t: T) -> T where T: Float, f32: Into<T>{ 
        // We use Into<T> so we can convert primitives into T, which
        // is type num::Float
        start * (1.0.into() - t) + end * t
    }

    /// # Linear to Gamma Space
    ///
    /// Converts the given value from gamma (sRGB) to linear color space.
    pub fn linear_to_gamma_space<T>(f: T) -> T where T: Float, f32: Into<T>{
        Math::pow(f,  0.454545.into())
    }

    /// # Log
    ///
    /// Returns the log of `f` to the base of `p`.
    pub fn log<T>(f: T, p: T) -> T where T: Float, f32: Into<T>{
        f.log(p)
    }

    /// # Log 10
    ///
    /// Returns the log of `f` to the base 10.
    pub fn log10<T>(f: T) -> T where T: Float, f32: Into<T>{
        f.log(10.0.into())
    }

    /// # Max
    ///
    /// Returns largest of two values.
    ///
    /// TODO: make it two or more values
    pub fn max<T>(a: T, b: T) -> T where T: Float, f32: Into<T>{
        if a > b  {a} else {b}
    }

    /// # Min
    ///
    /// Returns smallest of two values.
    ///
    /// TODO: make it two or more values
    pub fn min<T>(a: T, b: T) -> T where T: Float, f32: Into<T>{
        if a < b  {a} else {b}
    }

    /// # Move Towards
    ///
    /// Moves a value `current` towards `target`.
    pub fn move_towards<T>(current: T, target: T, max_delta: T) -> T where T: Float, f32: Into<T>{
        if Math::abs(target - current) <= max_delta{
            return target;
        }
        current + Math::sign(target - current) * max_delta
    }    

    /// # Move Towards Angle
    ///
    /// Same as `MoveTowards` but makes sure the values interpolate correctly when they wrap around 360 degrees.
    pub fn move_towards_angle<T>(current: T, mut target: T, max_delta: T) -> T where T: Float, f32: Into<T>{
        let delta_angle = Math::delta_angle(current, target);
        if -max_delta < delta_angle && delta_angle < max_delta{
            return target;
        }
        target = current + delta_angle;
        return Math::move_towards(current, target, max_delta);
    }   

    /// # Next Power of Two
    ///
    /// Returns the next power of two that is equal to, or greater than, the argument.
    pub fn next_power_of_two<T>(f: T) -> T where T: Float, f32: Into<T>{
        Math::pow(2.0.into(), Math::ceil(Math::log10(f)/Math::log10(2.0.into())))
    }
    
    /// # Perlin Noise
    ///
    /// Generate 2D Perlin noise.
    ///
    /// NOTE: This is currently not working 
    pub fn perlin_noise<T>(x: T, y: T) -> T where T: Float, f32: Into<T>{
        x
    }

    /// # Ping Pong
    ///
    /// PingPong returns a value that will increment and decrement between the value 0 and length.
    pub fn ping_pong<T>(mut t: T, length: T) -> T where T: Float, f32: Into<T>{
        t = Math::repeat(t, length * 2.0.into());

        length - Math::abs(t - length)
    }


    /// # Pow
    ///
    /// Returns of `f` to the power of `n`
    pub fn pow<T>(f: T, n: T) -> T where T: Float, f32: Into<T>{
        f.powf(n)
    }

    /// # Repeat
    ///
    /// Loops the value t, so that it is never larger than length and never smaller than 0.
    pub fn repeat<T>(t: T, length: T) -> T where T: Float, f32: Into<T>{ 
        Math::clamp(t - Math::floor(t / length) * length, 0.0.into(), length)
    }

    /// # Round
    ///
    /// Returns `f`rounded to the nearest integer.
    pub fn round<T>(f: T) -> T where T: Float, f32: Into<T>{ 
        f.round()
    }

    /// # Sign
    ///
    /// Returns the sign of `f`
    pub fn sign<T>(f: T) -> T where T: Float, f32: Into<T>{
        if f >= 0.0.into() {1.0.into()} else {-1.0.into()}
    }

    /// # Sine
    ///
    /// Returns the sine of `f`
    pub fn sin<T>(f: T) -> T where T: Float, f32: Into<T>{
        f.sin()
    }

    /// # Smooth Damp
    ///
    /// Gradually changes a value towards a desired goal over time.
    ///
    /// **NOTE: This will not work *properly* until time related variables are added. Until then, feel free to use with placeholder values
    /// for `delta_time`**
    pub fn smooth_damp<T>(current: T, mut target: T, mut current_velocity: T, mut smooth_time: T, max_speed: T, delta_time: T) -> T where T: Float, f32: Into<T>{
        // Based on Game Programming Gems 4 Chapter 1.10
        smooth_time = Math::max(0.0001.into(), smooth_time);
        let omega = 2.0.into() / smooth_time;

        let x = omega * delta_time;
        let exp = 1.0.into() / (1.0.into() + x + 0.48.into() * x * x + 0.235.into() * x * x * x);
        let mut change = current - target;
        let original_to = target;

        // Clamp maximum speed
        let max_change = max_speed * smooth_time;
        change = Math::clamp(change, -max_change, max_change);
        target = current - change;

        let temp = (current_velocity + omega * change) * delta_time;
        current_velocity = (current_velocity - omega * temp) * exp;
        let mut output = target + (change + temp) * exp;

        // Prevent overshooting
        if (original_to - current > 0.0.into()) == (output > original_to)
        {
            output = original_to;
            current_velocity = (output - original_to) / delta_time;
        }

        return output;
    }

    /// # Smooth Damp Angle
    ///
    /// Gradually changes an angle given in degrees towards a desired goal angle over time.
    ///
    /// **NOTE: This will not work *properly* until time related variables are added. Until then, feel free to use with placeholder values
    /// for `delta_time`**
    pub fn smooth_damp_angle<T>(current: T, mut target: T, current_velocity: T, smooth_time: T, max_speed: T, delta_time: T) -> T where T: Float, f32: Into<T>{
        target = current + Math::delta_angle(current, target);
        Math::smooth_damp(current, target, current_velocity, smooth_time, max_speed, delta_time)
    }

    /// # Smooth Step
    ///
    /// Interpolates between `min` and `max` with smoothing at the limits.
    pub fn smooth_step<T>(from: T, to: T, mut t: T) -> T where T: Float, f32: Into<T>{
        t = Math::clamp01(t);
        t = -2.0.into() * t * t * t + 3.0.into() * t * t;

        to * t + from * (1.0.into() - t)
    }

    /// # Sqrt
    ///
    /// Returns square root of `f`.
    pub fn sqrt<T>(f: T) -> T where T: Float, f32: Into<T>{
        f.sqrt()
    }

    /// # Tan
    ///
    /// Returns the tangent of `f` in radians
    pub fn tan<T>(f: T) -> T where T: Float, f32: Into<T>{
        f.tan()
    }
}