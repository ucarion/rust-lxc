/* automatically generated by rust-bindgen */

#![allow(non_camel_case_types)]

pub type Enum_lxc_attach_env_policy_t = ::libc::c_uint;
pub const LXC_ATTACH_KEEP_ENV: ::libc::c_uint = 0;
pub const LXC_ATTACH_CLEAR_ENV: ::libc::c_uint = 1;
pub type lxc_attach_env_policy_t = Enum_lxc_attach_env_policy_t;
pub type Enum_Unnamed1 = ::libc::c_uint;
pub const LXC_ATTACH_MOVE_TO_CGROUP: ::libc::c_uint = 1;
pub const LXC_ATTACH_DROP_CAPABILITIES: ::libc::c_uint = 2;
pub const LXC_ATTACH_SET_PERSONALITY: ::libc::c_uint = 4;
pub const LXC_ATTACH_LSM_EXEC: ::libc::c_uint = 8;
pub const LXC_ATTACH_REMOUNT_PROC_SYS: ::libc::c_uint = 65536;
pub const LXC_ATTACH_LSM_NOW: ::libc::c_uint = 131072;
pub const LXC_ATTACH_DEFAULT: ::libc::c_uint = 65535;
pub type lxc_attach_exec_t =
    ::std::option::Option<extern "C" fn(payload: *mut ::libc::c_void)
                              -> ::libc::c_int>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_lxc_attach_options_t {
    pub attach_flags: ::libc::c_int,
    pub namespaces: ::libc::c_int,
    pub personality: ::libc::c_long,
    pub initial_cwd: *mut ::libc::c_char,
    pub uid: ::libc::uid_t,
    pub gid: ::libc::gid_t,
    pub env_policy: lxc_attach_env_policy_t,
    pub extra_env_vars: *mut *mut ::libc::c_char,
    pub extra_keep_env: *mut *mut ::libc::c_char,
    pub stdin_fd: ::libc::c_int,
    pub stdout_fd: ::libc::c_int,
    pub stderr_fd: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_lxc_attach_options_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_lxc_attach_options_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type lxc_attach_options_t = Struct_lxc_attach_options_t;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_lxc_attach_command_t {
    pub program: *mut ::libc::c_char,
    pub argv: *mut *mut ::libc::c_char,
}
impl ::std::clone::Clone for Struct_lxc_attach_command_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_lxc_attach_command_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type lxc_attach_command_t = Struct_lxc_attach_command_t;
#[link(name = "lxc")]
extern "C" {
    pub fn lxc_attach_run_command(payload: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn lxc_attach_run_shell(payload: *mut ::libc::c_void)
     -> ::libc::c_int;
}
