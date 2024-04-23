//! Debug is a representation of Debug in unity.
//! 
//! It contains static functions that can be used to print to the console in unity.
//! 
//! This is useful for debugging purposes.

use crate::String;

/// # Debug
/// 
/// Debug is a representation of Debug in unity. This contains
/// functions that are used to print to the console in unity.
#[repr(C)]
pub struct Debug{
    /// # Log
    /// 
    /// Logs a string to the console in unity.
    log: extern "C" fn(&String),

    /// # Log Warning
    ///
    /// Logs a string to the console in unity as a warning.
    log_warning: extern "C" fn(&String),

    /// # Log Error
    /// 
    /// Logs a string to the console in unity as an error.
    log_error: extern "C" fn(&String),
}

impl Debug{
    /// # Log
    /// 
    /// Logs a string to the console in unity.
    pub fn log(&self, message: std::string::String){
        let message = String::from(message);
        (self.log)(&message);
        message.free();
    }

    /// # Log Warning
    /// 
    /// Logs a string to the console in unity as a warning.
    pub fn log_warning(&self, message: std::string::String){
        let message = String::from(message);
        (self.log_warning)(&message);
        message.free();
    }

    /// # Log Error
    /// 
    /// Logs a string to the console in unity as an error.
    pub fn log_error(&self, message: std::string::String){
        let message = String::from(message);
        (self.log_error)(&message);
        message.free();
    }
}
