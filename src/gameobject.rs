use crate::{String, Transform};

use libc::c_char;


#[repr(C)]
pub struct GameObject{
    tag: *mut c_char,
    transform: Transform,
    
    get_gameobject_from_tag_callback: extern fn(*const c_char) -> GameObject,
}

impl GameObject{
    pub fn get_gameobject_from_tag(&self, tag: String) -> Self{
        (self.get_gameobject_from_tag_callback)(tag.ptr)
    }
}
