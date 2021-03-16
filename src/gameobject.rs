use crate::{String, Transform, free_ptr};

use libc::c_char;

/// # GameObject
///
/// This is a representation of a gameobject in rust.
/// It aims to implement many of the gameobject functions through function pointers
/// (as many functions rely on unityengine functions, which is cheaper to just run across
/// rather than pass through every value possible). It is currently incomplete, but
/// stores a tag, transform and function(s).
///
/// The aim is to get as much compatibility as possible between unity and rust.
#[repr(C)]
pub struct GameObject{
    /* gameobject info */
    pub tag: *mut c_char,
    pub transform: Transform,

    /* function pointers */
    get_gameobject_from_tag_callback: extern fn(*const c_char) -> GameObject,
}

impl GameObject{
    pub fn get_gameobject_from_tag(&self, tag: String) -> Self{
        (self.get_gameobject_from_tag_callback)(tag.ptr)
    }
}

impl Drop for GameObject{
    fn drop(&mut self) {
        free_ptr(self.tag);
    }
}