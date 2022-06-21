mod vector3;
mod quaternion;
mod transform;
mod math;
mod data;
mod gameobject;
mod string;
mod time;
mod debug;

/// Define a NULL ptr for our string. This will help us
/// prevent allocating an invalid string
const NULL: *mut c_char = null::<c_char>() as *mut c_char;

use libc::c_char;
use std::ptr::null;

pub use vector3::Vector3;
pub use transform::Transform;
pub use gameobject::GameObject;
pub use math::Math;
pub use data::DataStruct;
pub use string::String;
pub use quaternion::Quaternion;
pub use time::Time;
use debug::Debug;
