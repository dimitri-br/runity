use std::{ffi::{CStr, CString}};

use libc::c_char;


use crate::{NULL};

/// # String
///
/// This struct defines a string that can be used to safely allocate a c-compatiable string
/// and a respective pointer. Will return an error upon allocation if the pointer is `NULL`
#[repr(C)]
#[derive(Clone)]
pub struct String{
    /// This is an immutable pointer.
    pub ptr: *mut c_char,
    pub len: u32,
}

impl String{
    // Free the string
    pub fn free(&self){
        unsafe{
            if self.ptr.is_null() {
                return;
            }
            CString::from_raw(self.ptr)
        };
    }
}



impl From<CString> for String{
    /// Create a new `String` from a `CString`
    fn from(value: CString) -> Self{
        let len = value.to_str().unwrap().len() as u32;
        let ptr = value.into_raw();

        if ptr == NULL{
            panic!("Error - pointer is null");
        }

        Self{
            ptr,
            len
        }
    }
}

impl From<&'static str> for String{
    /// Create a new `String` from a `str`
    fn from(value: &str) -> Self{
        let len = value.len() as u32;
        let string = CString::new(value).unwrap();
        let ptr = string.into_raw() as *mut c_char;

        if ptr == NULL{
            panic!("Error - pointer is null");
        }

        Self{
            ptr,
            len
        }
    }
}

impl From<std::string::String> for String{
    /// Create a new `String` from an `std::string::String`
    fn from(value: std::string::String) -> Self{
        let len = value.len() as u32;
        let string = CString::new(value).unwrap();
        let ptr = string.into_raw() as *mut c_char;

        if ptr == NULL{
            panic!("Error - pointer is null");
        }

        Self{
            ptr,
            len
        }
    }
}

impl From<&'static CStr> for String{
    /// Create a new `String` from a `CStr`
    fn from(value: &CStr) -> Self{
        let len = value.to_str().unwrap().len() as u32;
        let string = CString::new(value.to_str().unwrap()).unwrap();
        let ptr = string.into_raw() as *mut c_char;

        if ptr == NULL{
            panic!("Error - pointer is null");
        }

        Self{
            ptr,
            len
        }
    }
}
