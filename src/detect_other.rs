use crate::types::LibcVersions;

pub fn get_os_libc_versions() -> Result<LibcVersions, String> {
    Ok(LibcVersions {
        glibc: None,
        musl: None,
    })
}
