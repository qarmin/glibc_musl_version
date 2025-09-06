pub mod detect;
pub mod types;

pub use detect::get_os_libc_versions;
pub use types::{LibcVersions, Version};
