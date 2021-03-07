use crate::{String, Transform, free_ptr};

use libc::c_char;


#[repr(C)]
pub struct GameObject{
    pub tag: *mut c_char,
    pub transform: Transform,

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