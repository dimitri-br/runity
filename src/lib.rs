mod vector3;
mod quaternion;
mod transform;
mod math;
mod data;
mod gameobject;
mod string;


// Define a NULL ptr for our string. This will help us
// prevent allocating an invalid string
pub const NULL: *mut c_char = null::<c_char>() as *mut c_char;

pub fn free_ptr(mut _ptr: *mut i8){
    _ptr = NULL;
}

use libc::{c_char};
use std::{ptr::null};

pub use vector3::Vector3;
pub use transform::Transform;
pub use gameobject::GameObject;
pub use math::Math;
pub use data::DataStruct;
pub use string::String;
pub use quaternion::Quaternion;