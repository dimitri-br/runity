mod vector3;
mod quaternion;
mod transform;
mod math;
mod data;
mod gameobject;
mod string;
mod time;


/// Define a NULL ptr for our string. This will help us
/// prevent allocating an invalid string
const NULL: *mut c_char = null::<c_char>() as *mut c_char;

/// # Free Ptr
///
/// Free a pointer
fn free_ptr(ptr: *mut i8){
    // This isn't really recommended, but
    // it's the only real way to generally
    // free pointers and avoid memory leaks.
    //
    // FFI is unsafe :(
    unsafe{ 
        libc::free(ptr as *mut c_void); 
    }
}

use libc::{c_char, c_void};
use std::{ptr::null};

pub use vector3::Vector3;
pub use transform::Transform;
pub use gameobject::GameObject;
pub use math::Math;
pub use data::DataStruct;
pub use string::String;
pub use quaternion::Quaternion;
pub use time::Time;