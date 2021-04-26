use std::{cmp::max, ops::{Add, Div, Mul, Sub}};

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
    pub const k_epsilon: f32 = 0.00001;

    pub const k_epsilon_normal_sqrt: f32 = 1e-15;

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
    /// # Dot
    ///
    /// Dot product of two vectors
    pub fn dot(lhs: Self, rhs: Self) -> f32{
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    /// # Lerp
    ///
    /// Linearly interpolate between start `Vector3` and end `Vector3` across `t`, where `t` is clamped between 0 and 1.
    pub fn lerp(start: Self, end: Self, t: f32) -> Self{
        let t= Math::clamp01(t);
        Self::new(
            Math::lerp(start.x, end.x, t),
            Math::lerp(start.y, end.y, t),
            Math::lerp(start.z, end.z, t),
        )
    }

    /// # Lerp Unclamped
    ///
    /// Linearly interpolate between start `Vector3` and end `Vector3` across `t`, where `t` is unclamped.
    pub fn lerp_unclamped(start: Self, end: Self, t: f32) -> Self{
        Self::new(
            Math::lerp(start.x, end.x, t),
            Math::lerp(start.y, end.y, t),
            Math::lerp(start.z, end.z, t),
        )
    }

    /// # Move Towards
    ///
    /// Moves a point `current` in a straight line towards a `target` point.
    pub fn move_towards(current: Self, target: Self, max_distance_delta: f32) -> Self{
        let to_vector_x = target.x - current.x;
        let to_vector_y = target.y - current.y;
        let to_vector_z = target.z - current.z;

        let sqrdist = to_vector_x * to_vector_x + to_vector_y * to_vector_y + to_vector_z * to_vector_z;

        if sqrdist == 0.0 || (max_distance_delta >= 0.0 && sqrdist <= max_distance_delta * max_distance_delta){
            return target;
        }

        let dist = Math::sqrt(sqrdist);

        Self{
            x: current.x + to_vector_x / dist * max_distance_delta,
            y: current.y + to_vector_y / dist * max_distance_delta,
            z: current.z + to_vector_z / dist * max_distance_delta,
        }
    }

    /// # Smooth Damp
    ///
    /// Gradually changes a vector towards a desired goal over time.
    ///
    /// Due to lack of function overloading, all parameters must be defined here. 
    ///
    /// NOTE:
    /// DeltaTime won't work until timing is added to runity.
    pub fn smooth_damp(current: Self, mut target: Self, current_velocity: &mut Self, mut smooth_time: f32, max_speed: f32, delta_time: f32) -> Self{
        let mut output_x = 0.0;
        let mut output_y = 0.0;
        let mut output_z = 0.0;

        smooth_time = Math::max(0.00001, smooth_time);

        let omega = 2.0 / smooth_time;

        let x = omega * delta_time;

        let exp = 1.0 / (1.0 + x + 0.48 * x * x + 0.235 * x * x * x);

        let mut change_x = current.x - target.x;
        let mut change_y = current.y - target.y;
        let mut change_z = current.z - target.z;

        let original_to = target.clone();

        let max_change = max_speed * smooth_time;

        let max_change_sq = max_change * max_change;
        let sqrmag = change_x * change_x + change_y * change_y + change_z * change_z;
        if sqrmag > max_change_sq{
            let mag = Math::sqrt(sqrmag);
            change_x = change_x / mag * max_change;
            change_y = change_y / mag * max_change;
            change_z = change_z / mag * max_change;
        }

        target.x = current.x - change_x;
        target.y = current.y - change_y;
        target.z = current.z - change_z;

        let temp_x = (current_velocity.x + omega * change_x) * delta_time;
        let temp_y = (current_velocity.y + omega * change_y) * delta_time;
        let temp_z = (current_velocity.z + omega * change_z) * delta_time;

        current_velocity.x = (current_velocity.x - omega * temp_x) * exp;
        current_velocity.y = (current_velocity.y - omega * temp_y) * exp;
        current_velocity.z = (current_velocity.z - omega * temp_z) * exp;

        output_x = target.x + (change_x + temp_x) * exp;
        output_y = target.y + (change_y + temp_y) * exp;
        output_z = target.z + (change_z + temp_z) * exp;

        let orig_minus_current_x = original_to.x - current.x;
        let orig_minus_current_y = original_to.y - current.y;
        let orig_minus_current_z = original_to.z - current.z;

        let out_minus_orig_x = output_x - original_to.x;
        let out_minus_orig_y = output_y - original_to.y;
        let out_minus_orig_z = output_z - original_to.z;

        if orig_minus_current_x * out_minus_orig_x + orig_minus_current_x * out_minus_orig_y + orig_minus_current_z * out_minus_orig_z > 0.0{
            output_x = original_to.x;
            output_y = original_to.y;
            output_z = original_to.z;

            current_velocity.x = (output_x - original_to.x) / delta_time;
            current_velocity.y = (output_y - original_to.y) / delta_time;
            current_velocity.z = (output_z - original_to.z) / delta_time;
        } 

        Self{
            x: output_x,
            y: output_y,
            z: output_z
        }
    }

    /// # Set
    ///
    /// Set `x`, `y` and `z` components of an existing `Vector3`.
    pub fn set(&mut self, new_x: f32, new_y: f32, new_z: f32){
        self.x = new_x;
        self.y = new_y;
        self.z = new_z;
    }

    /// # Scale
    ///
    /// Multiplies two `Vector3` component-wise.
    pub fn scale(a: Self, b: Self) -> Self{
        Self{
            x: a.x * b.x,
            y: a.y * b.y,
            z: a.z * b.z
        }
    }
    
    /// # Cross
    /// 
    /// Cross Product of two vectors.
    pub fn cross(lhs: Self, rhs: Self) -> Self{
        Self{
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: lhs.z * rhs.x - lhs.x * rhs.z,
            z: lhs.x * rhs.y - lhs.y * rhs.x
        }
    }

    /// # Reflect
    ///
    /// Reflects a vector off the plane defined by a normal.
    pub fn reflect(in_direction: Self, in_normal: Self) -> Self{
        let factor = 2.0 * Vector3::dot(in_direction, in_normal);
        Self{
            x: factor * in_normal.x * in_direction.x,
            y: factor * in_normal.y * in_direction.y,
            z: factor * in_normal.z * in_direction.z,
        }
    }

    /// # Translate
    ///
    /// Translates `a` vector by `b` vector
    pub fn translate(a: Self, b: Self) -> Self{
        Self::new(a.x + b.x, a.y + b.y, a.z + b.z)
    }

    /// # Normalize
    ///
    /// Vector `value` will have a magnitude of 1
    pub fn normalize(value: Self) -> Self{
        let magnitude: f32 = Vector3::magnitude(value);

        if magnitude < Vector3::k_epsilon{
            return Vector3::zero();
        }

        Self::new(
            value.x / magnitude,
            value.y / magnitude,
            value.z / magnitude
        )
    }

    /// # Magnitude
    ///
    /// Return the length of this vector
    pub fn magnitude(value: Self) -> f32{
        (value.x * value.x + value.y * value.y).sqrt()
    }

    /// # Square Magnitude
    ///
    /// Return the squared length of this vector
    pub fn sqr_magnitude(value: Self) -> f32{
        value.x * value.x + value.y * value.y
    }

    /// # Project
    ///
    /// Projects one vector onto another vector.
    pub fn project(vector: Self, on_normal: Self) -> Self{
        let sqr_mag = Vector3::dot(on_normal, on_normal);
        if sqr_mag < Math::epsilon{
            return Self::zero();
        }

        let dot =  Vector3::dot(vector, on_normal);
        
        Self{
            x: on_normal.x * dot / sqr_mag,
            y: on_normal.y * dot / sqr_mag,
            z: on_normal.z * dot / sqr_mag
        }
    }

    /// # Project On Plane
    ///
    /// Projects a vector onto a plane defined by a normal orthogonal to the plane.
    pub fn project_on_plane(vector: Self, plane_normal: Self) -> Self{
        let sqr_mag = Self::dot(plane_normal, plane_normal);

        if sqr_mag < Math::epsilon{
            return vector;
        }

        let dot = Self::dot(vector, plane_normal);

        Self{
            x: vector.x - plane_normal.x * dot / sqr_mag,
            y: vector.y - plane_normal.y * dot / sqr_mag,
            z: vector.z - plane_normal.z * dot / sqr_mag,
        }
    }

    /// # Angle
    ///
    /// Returns the angle in degrees between `from` and `to`.
    pub fn angle(to: Self, from: Self) -> f32{
        let denominator = Math::sqrt(Self::sqr_magnitude(from) * Self::sqr_magnitude(to));

        if denominator < Self::k_epsilon_normal_sqrt{
            return 0.0;
        }

        let dot = Math::clamp(Self::dot(from, to) / denominator, -1.0, 1.0);

        Math::acos(dot) * Math::rad2deg
    }

    /// # Signed Angle
    ///
    /// The smaller of the two possible angles between the two vectors is returned, therefore the result will never be greater than 180 degrees or smaller than -180 degrees.
    /// If you imagine the from and to vectors as lines on a piece of paper, both originating from the same point, then the `axis` vector would point up out of the paper.
    /// The measured angle between the two vectors would be positive in a clockwise direction and negative in an anti-clockwise direction.
    pub fn signed_angle(to: Self, from: Self, axis: Self) -> f32{
        let unsigned_angle = Self::angle(from, to);

        let cross_x = from.y * to.z - from.z * to.y;
        let cross_y = from.z * to.x - from.x * to.z;
        let cross_z = from.x * to.y - from.y * to.x;

        let sign = Math::sign(axis.x * cross_x + axis.y * cross_y + axis.z * cross_z);

        unsigned_angle * sign
    }

    /// # Distance
    ///
    /// Returns the distance between `a` and `b`.   
    pub fn distance(a: Self, b: Self) -> f32{
        let diff_x = a.x - b.x;
        let diff_y = a.y - b.y;
        let diff_z = a.z - b.z;

        Math::sqrt(diff_x * diff_x + diff_y * diff_y + diff_z * diff_z)
    }

    /// # Clamp Magnitude
    ///
    /// Returns a copy of `vector` with its magnitude clamped to `max_length`.
    pub fn clamp_magnitude(vector: Self, max_length: f32) -> Self{
        let sqr_mag = Self::sqr_magnitude(vector);

        if sqr_mag > max_length * max_length{
            let mag = Math::sqrt(sqr_mag);

            //these intermediate variables force the intermediate result to be
            //of float precision. without this, the intermediate result can be of higher
            //precision, which changes behavior.
            let normalized_x =  vector.x / mag;
            let normalized_y =  vector.y / mag;
            let normalized_z =  vector.z / mag;

            return Self{
                x: normalized_x * max_length,
                y: normalized_y * max_length,
                z: normalized_z * max_length,
            };
        }

        vector
    }

    /// # Min
    ///
    /// Returns a vector made of the smallest components of two vectors.
    pub fn min(lhs: Self, rhs: Self) -> Self{
        Self{
            x: Math::min(lhs.x, rhs.x),
            y: Math::min(lhs.y, rhs.y),
            z: Math::min(lhs.z, rhs.z),
        }
    }

    /// # Max
    ///
    /// Returns a vector made of the largest components of two vectors.
    pub fn max(lhs: Self, rhs: Self) -> Self{
        Self{
            x: Math::max(lhs.x, rhs.x),
            y: Math::max(lhs.y, rhs.y),
            z: Math::max(lhs.z, rhs.z)
        }
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


/* Arithmetic for Vector3, so we don't need functions */

impl Add for Vector3{
    type Output = Vector3;

    fn add(self, rhs: Self::Output) -> Self::Output {
        Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl Sub for Vector3{
    type Output = Vector3;

    fn sub(self, rhs: Self::Output) -> Self::Output {
        Self{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl Mul for Vector3{
    type Output = Vector3;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        Self{
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl Div for Vector3{
    type Output = Vector3;

    /// # IMPORTANT
    /// Any attempted division by 0 will simply return 0, rather than attempting the division and panicking.
    fn div(self, rhs: Self) -> Self::Output {
        let x = if self.x == 0.0 || rhs.x == 0.0 { 0.0 } else { self.x / rhs.x };
        let y = if self.y == 0.0 || rhs.y == 0.0 { 0.0 } else { self.y / rhs.y };
        let z = if self.z == 0.0 || rhs.z == 0.0 { 0.0 } else { self.z / rhs.z };
        Self{
            x,
            y,
            z
        }
    }
}