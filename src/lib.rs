extern crate libc;

pub mod ffi;

use ffi::lxccontainer;

use std::ffi::CStr;
use std::str;

pub fn version<'a>() -> &'a str {
    let version = unsafe { CStr::from_ptr(lxccontainer::lxc_get_version()) };
    str::from_utf8(version.to_bytes()).unwrap()
}

#[test]
fn it_works() {
    assert_eq!("1.1.2", version());
}
