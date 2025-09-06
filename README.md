# glibc_musl_version

A Rust library for detecting the installed glibc or musl version on Linux systems.

Similar to [glibc_version](https://crates.io/crates/glibc_version) crate, but it supports also musl and have 0 additional dependencies.

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
glibc_musl_version = "0.1"
```

Example usage(you can run it via `cargo run --example print_libc`):

```rust
use glibc_musl_version::get_os_libc_versions;

fn main() {
    match get_os_libc_versions() {
        Ok(versions) => println!("glibc/musl: {versions}"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
```

## Note
I wanted also to get the version of libc used by the binary (because not always the system libc is used), but I didn't find any reliable and fast way to do it. If I find a good method, I will try to add it in the future.
