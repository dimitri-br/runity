use std::{ffi::{CStr, CString}};

use libc::c_char;


use crate::{NULL, free_ptr};

/// # String
///
/// This struct defines a string that can be used to safely allocate a c-compatiable string
/// and a respective pointer. Will return an error upon allocation if the pointer is `NULL`
pub struct String<'a>{
    pub ptr: *mut c_char,
    pub string: &'a CStr
}

impl<'a> String<'a>{
    pub fn new(value: CString) -> Result<Self, &'a str>{
        let ptr = value.into_raw() as *mut c_char;

        if ptr == NULL{
            return Err("Error - pointer is null");
        }
        let string = unsafe{ CStr::from_ptr(ptr) };

        Ok(Self{
            ptr,
            string
        })
    }
}

impl<'a> Drop for String<'a>{
    fn drop(&mut self) {
        free_ptr(self.ptr);
    }
}