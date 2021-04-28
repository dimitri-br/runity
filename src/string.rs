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

impl<'a> From<CString> for String<'a>{
    /// Create a new `String` from a `CString`
    fn from(value: CString) -> Self{
        let len = value.to_str().unwrap().len() as i32;
        let ptr = value.into_raw() as *mut c_char;

        if ptr == NULL{
            panic!("Error - pointer is null");
        }
        let string = unsafe{ CStr::from_ptr(ptr) };

        Self{
            ptr,
            string,
            len
        }
    }
}

impl<'a> From<&'a str> for String<'a>{
    /// Create a new `String` from a `str`
    fn from(value: &str) -> Self{
        let len = value.len() as i32;

        let ptr = CString::new(value).unwrap().into_raw() as *mut c_char;

        if ptr == NULL{
            panic!("Error - pointer is null");
        }
        let string = unsafe{ CStr::from_ptr(ptr) };

        Self{
            ptr,
            string,
            len
        }
    }
}

impl<'a> From<std::string::String> for String<'a>{
    /// Create a new `String` from an `std::string::String`
    fn from(value: std::string::String) -> Self{
        let len = value.len() as i32;
        let ptr = CString::new(value).unwrap().into_raw() as *mut c_char;

        if ptr == NULL{
            panic!("Error - pointer is null");
        }
        let string = unsafe{ CStr::from_ptr(ptr) };

        Self{
            ptr,
            string,
            len
        }
    }
}

impl<'a> From<&'a CStr> for String<'a>{
    /// Create a new `String` from a `CStr`
    fn from(value: &CStr) -> Self{
        let len = value.to_str().unwrap().len() as i32;

        let ptr = value.as_ptr() as *mut c_char;

        if ptr == NULL{
            panic!("Error - pointer is null");
        }
        let string = unsafe{ CStr::from_ptr(ptr) };

        Self{
            ptr,
            string,
            len
        }
    }
}

/// Not sure if this is needed
impl<'a> Drop for String<'a>{
    fn drop(&mut self) {
        free_ptr(self.ptr);
    }
}