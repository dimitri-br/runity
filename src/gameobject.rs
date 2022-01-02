use crate::{String, Transform};

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
    pub tag: String,
    pub transform: Transform,

    /* function pointers */
    get_gameobject_from_tag_callback: extern fn(&String) -> GameObject,
}

impl GameObject{
    /// # Get GameObject from tag
    ///
    /// Takes a string, returns the gameobject attached to the associated tag.
    /// 
    /// The string is consumed, so it is not possible to use the string after this function.
    pub fn get_gameobject_from_tag(&self, tag: &String) -> Self{
        (self.get_gameobject_from_tag_callback)(tag)
    }
}