use std::ffi::CString;
use std::ptr;

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
}
