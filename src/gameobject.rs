use crate::{String, Transform, Vector3, Quaternion};

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

    // Get gameobject by tag - this takes in a string and a pointer to a gameobject
    get_gameobject_from_tag_callback: extern "C" fn(&String, *mut GameObject),
}

impl GameObject{
    /// # Get GameObject from tag
    ///
    /// Takes a string, returns the gameobject attached to the associated tag.
    /// 
    /// The returned gameobject is a read-only gameobject, meaning that it cannot be modified.
    /// Any modifications to the gameobject is undefined behaviour.
    pub fn get_gameobject_from_tag(&self, tag: &String) -> Self{
        // Create a new gameobject using this gameobject's function pointer
        let mut gameobject = GameObject{
            tag: tag.clone(),
            transform: Transform::new(Vector3::new(0.0, 0.0, 0.0), Quaternion::new(0.0, 0.0, 0.0, 0.0)),
            get_gameobject_from_tag_callback: self.get_gameobject_from_tag_callback,
        };

        // Convert the gameobject to a mutable gameobject pointer
        let gameobject_ptr = &mut gameobject as *mut GameObject;

        (self.get_gameobject_from_tag_callback)(tag, gameobject_ptr);

        // Return the gameobject
        gameobject
    }
}
