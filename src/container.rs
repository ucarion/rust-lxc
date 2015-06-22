use std::str;
use std::ptr;
use std::ffi::{CStr, CString};

use ffi::lxccontainer;

pub struct Container {
    container: *mut lxccontainer::Struct_lxc_container
}

impl Container {
    pub fn new(name: &str, config_path: Option<&str>) -> Option<Container> {
        let name = CString::new(name).unwrap().as_ptr();
        let config_path = config_path.map_or(ptr::null(), |config_path| {
            CString::new(config_path).unwrap().as_ptr()
        });
        let container = unsafe {
            lxccontainer::lxc_container_new(name, config_path)
        };

        if container.is_null() {
            None
        } else {
            Some(Container { container: container })
        }
    }

    // TODO: Test this
    pub fn states<'a>() -> Vec<&'a str> {
        let num_states = unsafe {
            lxccontainer::lxc_get_wait_states(ptr::null_mut())
        };

        let mut states = Vec::with_capacity(num_states as usize);
        unsafe { lxccontainer::lxc_get_wait_states(states.as_mut_ptr()); }

        states.iter().map(|&state| {
            unsafe { str::from_utf8(CStr::from_ptr(state).to_bytes()).unwrap() }
        }).collect()
    }

    pub fn is_defined(&self) -> bool {
        unsafe {
            ((*self.container).is_defined.unwrap())(self.container) != 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_states() {
        println!("{:?}", Container::states());
        assert!(false);
    }

    #[test]
    fn test_is_defined() {
        // You must run lxc-create -n foobar ... to make this test pass
        //
        // TODO: Automate this test.
        assert!(Container::new("foobar", None).unwrap().is_defined());
        assert!(!Container::new("does-not-exist", None).unwrap().is_defined());
    }
}
