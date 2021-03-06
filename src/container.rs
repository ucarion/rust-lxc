use std::str;
use std::ptr;
use std::ffi::{CStr, CString};
use libc::{c_char, pid_t};

use super::{Result, LxcError};
use ffi::lxccontainer;

macro_rules! lxc_call {
    // lxc_call!(self.container, is_defined)
    //   => ((*self.container).is_defined.unwrap())(self.container)
    ($container: expr, $func: ident) => {
        ((*$container).$func.unwrap())($container)
    };

    // lxc_call!(self.container, load_config, alt_file)
    //   => ((*self.container).load_config.unwrap())(self.container, alt_file)
    ($container: expr, $func: ident, $( $x: expr ),*) => {
        ((*$container).$func.unwrap())($container, $($x,)*)
    };
}

fn check_lxc_error(lxc_return_code: u8, error_msg: &'static str) -> Result<()> {
    if lxc_return_code != 0 {
        Ok(())
    } else {
        Err(LxcError::Unknown(error_msg))
    }
}

pub struct Container {
    container: *mut lxccontainer::Struct_lxc_container
}

impl Container {
    pub fn new(name: &str, config_path: Option<&str>) -> Result<Container> {
        let name = CString::new(name).unwrap().as_ptr();
        let config_path = config_path.map_or(ptr::null(), |config_path| {
            CString::new(config_path).unwrap().as_ptr()
        });
        let container = unsafe {
            lxccontainer::lxc_container_new(name, config_path)
        };

        if container.is_null() {
            Err(LxcError::Unknown("Creating the container failed"))
        } else {
            Ok(Container { container: container })
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
            lxc_call!(self.container, is_defined) != 0
        }
    }

    pub fn state<'a>(&self) -> &'a str {
        let state = unsafe {
            CStr::from_ptr(lxc_call!(self.container, state))
        };
        str::from_utf8(state.to_bytes()).unwrap()
    }

    pub fn is_running(&self) -> bool {
        unsafe {
            lxc_call!(self.container, is_running) != 0
        }
    }

    pub fn freeze(&mut self) -> Result<()> {
        check_lxc_error(unsafe { lxc_call!(self.container, freeze) },
                        "Freezing the container failed")
    }

    pub fn unfreeze(&mut self) -> Result<()> {
        check_lxc_error(unsafe { lxc_call!(self.container, unfreeze) },
                        "Unfreezing the container failed")
    }

    pub fn init_pid(&self) -> pid_t {
        unsafe { lxc_call!(self.container, init_pid) }
    }

    pub fn load_config(&mut self, alt_file: Option<&str>) -> Result<()> {
        let alt_file = alt_file.map_or(ptr::null(), |alt_file| {
            CString::new(alt_file).unwrap().as_ptr()
        });
        let ret = unsafe { lxc_call!(self.container, load_config, alt_file) };

        check_lxc_error(ret, "Loading config for the container failed")
    }

    pub fn start_with_args(&mut self, use_init: bool, argv: &[&str]) -> Result<()> {
        let argv_ptrs: Vec<_> = argv.iter().map(|&arg| {
            CString::new(arg).unwrap().as_ptr()
        }).collect();

        self.start_internal(use_init, argv_ptrs.as_ptr())
    }

    pub fn start(&mut self, use_init: bool) -> Result<()> {
        self.start_internal(use_init, ptr::null())
    }

    fn start_internal(&mut self, use_init: bool, argv: *const *const c_char)
            -> Result<()> {
        let use_init = if use_init { 1 } else { 0 };
        let ret = unsafe {
            lxc_call!(self.container, start, use_init, argv)
        };

        check_lxc_error(ret, "Starting the container failed")
    }

    pub fn stop(&mut self) -> Result<()> {
        check_lxc_error(unsafe { lxc_call!(self.container, stop) },
                        "Stopping the container failed")
    }

    pub fn want_daemonize(&self, state: bool) -> bool {
        let state = if state { 1 } else { 0 };
        unsafe { lxc_call!(self.container, want_daemonize, state) != 0 }
    }

    pub fn want_close_all_fds(&self, state: bool) -> bool {
        let state = if state { 1 } else { 0 };
        unsafe { lxc_call!(self.container, want_close_all_fds, state) != 0 }
    }

    pub fn config_file_name(&self) -> Result<String> {
        let config_ptr = unsafe { lxc_call!(self.container, config_file_name) };
        if config_ptr.is_null() {
            Err(LxcError::Unknown("Getting config file name failed"))
        } else {
            let config = unsafe { CStr::from_ptr(config_ptr).to_bytes() };
            Ok(str::from_utf8(config).unwrap().to_owned())
        }
    }

    pub fn wait(&self, state: &str, timeout: i32) -> bool {
        let state = CString::new(state).unwrap().as_ptr();
        unsafe { lxc_call!(self.container, wait, state, timeout) != 0 }
    }

    pub fn set_config_item(&mut self, key: &str, value: &str) -> Result<()> {
        let key = CString::new(key).unwrap().as_ptr();
        let value = CString::new(value).unwrap().as_ptr();
        let ret = unsafe {
            lxc_call!(self.container, set_config_item, key, value)
        };

        check_lxc_error(ret, "Setting the config item failed")
    }

    pub fn set_config_path(&mut self, path: &str) -> Result<()> {
        let path = CString::new(path).unwrap().as_ptr();
        let ret = unsafe { lxc_call!(self.container, set_config_path, path) };

        check_lxc_error(ret, "Setting the config path failed")
    }

    pub fn destroy(&mut self) -> Result<()> {
        check_lxc_error(unsafe { lxc_call!(self.container, destroy) },
                        "Destroying the container failed")
    }

    pub fn save_config(&mut self, alt_file: &str) -> Result<()> {
        let alt_file = CString::new(alt_file).unwrap().as_ptr();
        let ret = unsafe { lxc_call!(self.container, save_config, alt_file) };

        check_lxc_error(ret, "Saving config to file failed")
    }

    pub fn create(&mut self, template: &str, bdev_type: &str,
            specs: &mut BDevSpecs, flags: i32, argv: &[&str]) -> Result<()> {

        let template = CString::new(template).unwrap().as_ptr();
        let bdev_type = CString::new(bdev_type).unwrap().as_ptr();
        let specs: *mut _ = &mut specs.bdev_specs;
        let argv: Vec<_> = argv.iter().map(|&arg| {
            CString::new(arg).unwrap().as_ptr() as *mut i8
        }).collect();
        let ret = unsafe {
            lxc_call!(self.container, create, template, bdev_type, specs, flags,
                      argv.as_ptr())
        };

        check_lxc_error(ret, "Creating container failed")
    }

    pub fn rename(&mut self, new_name: &str) -> Result<()> {
        let new_name = CString::new(new_name).unwrap().as_ptr();
        check_lxc_error(unsafe { lxc_call!(self.container, rename, new_name) },
                        "Renaming container failed")
    }

    pub fn reboot(&mut self) -> Result<()> {
        check_lxc_error(unsafe { lxc_call!(self.container, reboot) },
                        "Rebooting container failed")
    }

    pub fn shutdown(&mut self, timeout: i32) -> Result<()> {
        check_lxc_error(unsafe { lxc_call!(self.container, shutdown, timeout) },
                        "Shutting down container failed")
    }

    // this returns Result<()> for consistency's sake
    pub fn clear_config(&mut self) -> Result<()> {
        Ok(unsafe { lxc_call!(self.container, clear_config) })
    }
}

pub struct BDevSpecs {
    bdev_specs: lxccontainer::Struct_bdev_specs
}

impl BDevSpecs {
    pub fn new(fs_type: String, fs_size: u64, zfs: Option<ZfsRoot>,
            lvm: Option<Lvm>, dir: String) -> BDevSpecs {
        let fs_type = CString::new(fs_type).unwrap().as_ptr() as *mut i8;
        let zfs = zfs.unwrap_or(ZfsRoot::new(None));
        let lvm = lvm.unwrap_or(Lvm::new(None, None, None));
        let dir = CString::new(dir).unwrap().as_ptr() as *mut i8;

        BDevSpecs { bdev_specs: lxccontainer::Struct_bdev_specs {
            fstype: fs_type,
            fssize: fs_size,
            zfs: zfs.zfs_root,
            lvm: lvm.lvm,
            dir: dir
        }}
    }
}

pub struct ZfsRoot {
    zfs_root: lxccontainer::Struct_Unnamed1
}

impl ZfsRoot {
    pub fn new(zfs_root: Option<String>) -> ZfsRoot {
        let zfs_root = zfs_root.map_or(ptr::null_mut(), { |zfs_root|
            CString::new(zfs_root).unwrap().as_ptr() as *mut i8
        });
        ZfsRoot { zfs_root: lxccontainer::Struct_Unnamed1 {
            zfsroot: zfs_root
        }}
    }
}

pub struct Lvm {
    lvm: lxccontainer::Struct_Unnamed2
}

impl Lvm {
    pub fn new(vg: Option<String>, lv: Option<String>, thinpool: Option<String>)
            -> Lvm {
        let vg = vg.map_or(ptr::null_mut(), { |vg|
            CString::new(vg).unwrap().as_ptr() as *mut i8
        });
        let lv = lv.map_or(ptr::null_mut(), { |lv|
            CString::new(lv).unwrap().as_ptr() as *mut i8
        });
        let thinpool = thinpool.map_or(ptr::null_mut(), { |thinpool|
            CString::new(thinpool).unwrap().as_ptr() as *mut i8
        });

        Lvm { lvm: lxccontainer::Struct_Unnamed2 {
            vg: vg,
            lv: lv,
            thinpool: thinpool
        }}
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
