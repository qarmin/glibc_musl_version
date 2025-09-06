#[cfg_attr(target_os = "linux", path = "detect_linux.rs")]
#[cfg_attr(not(target_os = "linux"), path = "detect_other.rs")]
mod detect_impl;

pub use detect_impl::get_os_libc_versions;
