extern crate libc;

use std::ffi::CStr;
use std::str;

pub mod ffi;
pub mod container;

pub type Result<T> = std::result::Result<T, LxcError>;

#[derive(Debug)]
pub enum LxcError {
    Unknown(&'static str)
}

pub fn version<'a>() -> &'a str {
    use ffi::lxccontainer;

    let version = unsafe { CStr::from_ptr(lxccontainer::lxc_get_version()) };
    str::from_utf8(version.to_bytes()).unwrap()
}

#[test]
fn test_version() {
    assert_eq!("1.1.2", version());
}
