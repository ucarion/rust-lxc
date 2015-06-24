use std::str;
use std::ptr;
use std::ffi::{CStr, CString};
use libc::pid_t;

use super::Result;
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

    // TODO: Why doesn't this work?
    //
    // pub fn states<'a>() -> Vec<&'a str> {
    //     let num_states = unsafe {
    //         lxccontainer::lxc_get_wait_states(ptr::null_mut())
    //     };

    //     let mut states = Vec::with_capacity(num_states as usize);
    //     unsafe { lxccontainer::lxc_get_wait_states(states.as_mut_ptr()); }

    //     states.iter().map(|&state| {
    //         unsafe { str::from_utf8(CStr::from_ptr(state).to_bytes()).unwrap() }
    //     }).collect()
    // }

    pub fn is_defined(&self) -> bool {
        unsafe {
            ((*self.container).is_defined.unwrap())(self.container) != 0
        }
    }

    pub fn state<'a>(&self) -> &'a str {
        let state = unsafe {
            CStr::from_ptr(((*self.container).state.unwrap())(self.container))
        };
        str::from_utf8(state.to_bytes()).unwrap()
    }

    pub fn is_running(&self) -> bool {
        unsafe {
            ((*self.container).is_running.unwrap())(self.container) != 0
        }
    }

    pub fn freeze(&mut self) -> Result {
        let ret = unsafe {
            ((*self.container).freeze.unwrap())(self.container) != 0
        };

        if ret {
            Ok(())
        } else {
            Err("Freezing the container failed")
        }
    }

    pub fn unfreeze(&mut self) -> Result {
        let ret = unsafe {
            ((*self.container).unfreeze.unwrap())(self.container) != 0
        };

        if ret {
            Ok(())
        } else {
            Err("Unfreezing the container failed")
        }
    }

    pub fn init_pid(&self) -> pid_t {
        unsafe { ((*self.container).init_pid.unwrap())(self.container) }
    }

    pub fn load_config(&mut self, alt_file: Option<&str>) -> Result {
        let alt_file = alt_file.map_or(ptr::null(), str::as_ptr) as *const i8;
        let ret = unsafe {
            ((*self.container).load_config.unwrap())(self.container, alt_file)
        };

        if ret != 0 {
            Ok(())
        } else {
            Err("Loading config for the container failed")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Why is this returning an empty list?
    // #[test]
    // fn test_states() {
    //     println!("{:?}", Container::states());
    //     assert!(false);
    // }

    #[test]
    fn test_is_defined() {
        // You must run lxc-create -n foobar ... to make this test pass
        //
        // TODO: Automate this test.
        assert!(Container::new("foobar", None).unwrap().is_defined());
        assert!(!Container::new("does-not-exist", None).unwrap().is_defined());
    }

    #[test]
    fn test_state() {
        assert_eq!("RUNNING", Container::new("foobar", None).unwrap().state());
    }

    #[test]
    fn test_is_running() {
        // TODO: Automate this test.
        assert!(Container::new("foobar", None).unwrap().is_running());
    }

    #[test]
    fn test_freeze_unfreeze() {
        let mut c = Container::new("foobar", None).unwrap();
        c.freeze().unwrap();
        assert_eq!("FROZEN", c.state());
        c.unfreeze().unwrap();
    }
}
