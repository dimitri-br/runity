use crate::{String, Transform};

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
    get_gameobject_from_tag_callback: extern "stdcall" fn(&String) -> GameObject,
}

impl GameObject{
    /// # Get GameObject from tag
    ///
    /// Takes a string, returns the gameobject attached to the associated tag.
    /// 
    /// The returned gameobject is a read-only gameobject, meaning that it cannot be modified.
    /// Any modifications to the gameobject is undefined behaviour.
    pub fn get_gameobject_from_tag(&self, tag: &String) -> Self{
        (self.get_gameobject_from_tag_callback)(tag)
    }
}