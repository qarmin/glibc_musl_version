use glibc_musl_version::get_os_libc_versions;

fn main() {
    match get_os_libc_versions() {
        Ok(versions) => println!("glibc/musl: {versions}"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
