use std::{ffi::{CStr, CString}};

use libc::c_char;


use crate::{NULL, free_ptr};

/// # String
///
/// This struct defines a string that can be used to safely allocate a c-compatiable string
/// and a respective pointer. Will return an error upon allocation if the pointer is `NULL`
pub struct String<'a>{
    pub ptr: *mut c_char,
    pub string: &'a CStr,
    pub len: i32,
}

impl<'a> String<'a>{
    /// Create a new `String` from a `CString`
    pub fn from_cstring(value: CString) -> Result<Self, &'a str>{
        let len = value.to_str().unwrap().len() as i32;
        let ptr = value.into_raw() as *mut c_char;

        if ptr == NULL{
            return Err("Error - pointer is null");
        }
        let string = unsafe{ CStr::from_ptr(ptr) };

        Ok(Self{
            ptr,
            string,
            len
        })
    }

    /// Create a new `String` from a `str`
    pub fn from_str(value: &str) -> Result<Self, &'a str>{
        let len = value.len() as i32;

        let ptr = CString::new(value).unwrap().into_raw() as *mut c_char;

        if ptr == NULL{
            return Err("Error - pointer is null");
        }
        let string = unsafe{ CStr::from_ptr(ptr) };

        Ok(Self{
            ptr,
            string,
            len
        })
    }

    /// Create a new `String` from an `std::string::String`
    pub fn from_string(value: std::string::String) -> Result<Self, &'a str>{
        let len = value.len() as i32;
        let ptr = CString::new(value).unwrap().into_raw() as *mut c_char;

        if ptr == NULL{
            return Err("Error - pointer is null");
        }
        let string = unsafe{ CStr::from_ptr(ptr) };

        Ok(Self{
            ptr,
            string,
            len
        })
    }

    /// Create a new `String` from a `CStr`
    pub fn from_cstr(value: &CStr) -> Result<Self, &'a str>{
        let len = value.to_str().unwrap().len() as i32;

        let ptr = value.as_ptr() as *mut c_char;

        if ptr == NULL{
            return Err("Error - pointer is null");
        }
        let string = unsafe{ CStr::from_ptr(ptr) };

        Ok(Self{
            ptr,
            string,
            len
        })
    }
}

/// Not sure if this is needed
impl<'a> Drop for String<'a>{
    fn drop(&mut self) {
        free_ptr(self.ptr);
    }
}