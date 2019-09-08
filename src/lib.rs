//#![allow(dead_code)]

use std::os::raw::c_char;
use std::string;

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

use libc::size_t;

pub mod error;
pub mod native_types;
pub mod raw;
pub mod rediserror;
mod redismodule;
pub mod redisraw;
pub mod redisvalue;

#[macro_use]
mod macros;
mod context;
mod key;

#[cfg(feature = "experimental-api")]
mod thread_safe_context;

#[cfg(feature = "experimental-api")]
pub use crate::thread_safe_context::ThreadSafeContext;

pub use crate::context::Context;
pub use crate::redismodule::*;

/// `LogLevel` is a level of logging to be specified with a Redis log directive.
#[derive(Clone, Copy, Debug)]
pub enum LogLevel {
    Debug,
    Notice,
    Verbose,
    Warning,
}

fn from_byte_string(
    byte_str: *const c_char,
    length: size_t,
) -> Result<String, string::FromUtf8Error> {
    let mut vec_str: Vec<u8> = Vec::with_capacity(length as usize);
    for j in 0..length {
        let byte = unsafe { *byte_str.offset(j as isize) } as u8;
        vec_str.insert(j, byte);
    }

    String::from_utf8(vec_str)
}
