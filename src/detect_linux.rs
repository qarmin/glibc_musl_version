use std::process::Command;

use crate::types::{LibcVersions, Version};

pub fn get_libc_version_output(app: &str) -> Result<String, String> {
    let output = Command::new(app)
        .args(["--version"])
        .output()
        .map_err(|e| format!("failed to execute ldd: {e}"))?;

    let output_str = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
    .trim()
    .to_string();

    Ok(output_str)
}

pub fn get_os_libc_versions() -> Result<LibcVersions, String> {
    let ldd_output = get_libc_version_output("ldd")?;

    let glibc = if let Ok(version_str) = ldd_output_to_glibc_version_str(&ldd_output) {
        parse_glibc_version(version_str)
    } else {
        None
    };

    let musl_output = get_libc_version_output("musl-ldd").unwrap_or(ldd_output);

    let musl = parse_musl_version(&musl_output);

    Ok(LibcVersions { glibc, musl })
}

fn ldd_output_to_glibc_version_str(output_str: &str) -> Result<&str, String> {
    for line in output_str.lines() {
        if line.contains("ldd (") {
            if let Some(idx) = line.find(')') {
                let after = &line[idx + 1..];
                if let Some(tok) = find_version_token(after) {
                    return Ok(tok);
                }
            }
        }
    }
    Err("no glibc version in ldd output".to_string())
}

fn parse_glibc_version(version: &str) -> Option<Version> {
    let mut parts = version.split('.').map(|s| s.parse::<usize>());
    match (parts.next(), parts.next()) {
        (Some(Ok(major)), Some(Ok(minor))) => Some(Version { major, minor }),
        _ => None,
    }
}

fn parse_musl_version(output: &str) -> Option<Version> {
    if let Some(pos) = output.find("musl libc") {
        let rest = &output[pos..];
        if let Some(tok) = find_version_token(rest) {
            return parse_glibc_version(tok);
        }
    }

    for line in output.lines() {
        if let Some(rest) = line.strip_prefix("Version ") {
            if let Some(tok) = find_version_token(rest) {
                return parse_glibc_version(tok);
            }
        }
    }

    None
}

fn find_version_token(s: &str) -> Option<&str> {
    for token in s.split(|c: char| !(c.is_ascii_digit() || c == '.')) {
        if token.is_empty() {
            continue;
        }
        if token.contains('.') {
            let mut parts = token.split('.');
            if let (Some(a), Some(b)) = (parts.next(), parts.next()) {
                if a.chars().all(|c| c.is_ascii_digit()) && b.chars().all(|c| c.is_ascii_digit()) {
                    return Some(token);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{LibcVersions, Version};

    #[test]
    fn parse_glibc_ldd_output() {
        let out = r#"ldd (GNU libc) 2.12
Copyright (C) 2010 Free Software Foundation, Inc."#;
        let ver_res = ldd_output_to_glibc_version_str(out);
        assert!(ver_res.is_ok());
        let ver_str = ver_res.unwrap();
        assert_eq!(ver_str, "2.12");
        let parsed = parse_glibc_version(ver_str);
        let parsed = parsed.unwrap();
        assert_eq!(parsed.major, 2);
        assert_eq!(parsed.minor, 12);

        let out2 = r#"ldd (Ubuntu GLIBC 2.41-6ubuntu1.1) 2.41
Copyright (C) 2024 Free Software Foundation, Inc.
This is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
Written by Roland McGrath and Ulrich Drepper.
"#;
        let ver_res2 = ldd_output_to_glibc_version_str(out2);
        assert!(ver_res2.is_ok());
        let ver_str2 = ver_res2.unwrap();
        assert_eq!(ver_str2, "2.41");
        let parsed2 = parse_glibc_version(ver_str2);
        let parsed2 = parsed2.unwrap();
        assert_eq!(parsed2.major, 2);
        assert_eq!(parsed2.minor, 41);
    }

    #[test]
    fn parse_musl_from_ldd_output() {
        let out = "musl libc (x86_64)\nVersion 1.2.3\n";
        let parsed = parse_musl_version(out);
        let parsed = parsed.unwrap();
        assert_eq!(parsed.major, 1);
        assert_eq!(parsed.minor, 2);

        let out2 = "musl libc (x86_64) 1.3.0\n";
        let parsed2 = parse_musl_version(out2);
        let parsed2 = parsed2.unwrap();
        assert_eq!(parsed2.major, 1);
        assert_eq!(parsed2.minor, 3);
    }

    #[test]
    fn get_version_fallbacks() {
        let out = "some random output not containing versions";
        assert!(ldd_output_to_glibc_version_str(out).is_err());
        assert!(parse_musl_version(out).is_none());
    }

    #[test]
    fn get_os_libc_versions_both_none() {
        // Simulate output with neither glibc nor musl present
        let out = "random output without libc info";
        let glibc = parse_glibc_version(out);
        let musl = parse_musl_version(out);
        assert!(glibc.is_none());
        assert!(musl.is_none());
    }

    #[test]
    fn get_os_libc_versions_glibc_only() {
        // Simulate glibc output
        let out = r#"ldd (GNU libc) 2.17\nCopyright (C) 2013 Free Software Foundation, Inc."#;
        let glibc = parse_glibc_version("2.17");
        let musl = parse_musl_version(out);
        assert!(glibc.is_some());
        assert!(musl.is_none());
        let v = glibc.unwrap();
        assert_eq!(v.major, 2);
        assert_eq!(v.minor, 17);
    }

    #[test]
    fn get_os_libc_versions_musl_only() {
        // Simulate musl output
        let out = "musl libc (x86_64)\nVersion 1.1.24\n";
        let glibc = parse_glibc_version(out);
        let musl = parse_musl_version(out);
        assert!(glibc.is_none());
        assert!(musl.is_some());
        let v = musl.unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 1);
    }

    #[test]
    fn libc_versions_display() {
        let both = LibcVersions {
            glibc: Some(Version { major: 2, minor: 31 }),
            musl: Some(Version { major: 1, minor: 2 }),
        };
        assert_eq!(both.to_string(), "glibc 2.31 | musl 1.2");
        let only_glibc = LibcVersions {
            glibc: Some(Version { major: 2, minor: 17 }),
            musl: None,
        };
        assert_eq!(only_glibc.to_string(), "glibc 2.17 | musl <not detected>");
        let only_musl = LibcVersions {
            glibc: None,
            musl: Some(Version { major: 1, minor: 1 }),
        };
        assert_eq!(only_musl.to_string(), "glibc <not detected> | musl 1.1");
        let none = LibcVersions {
            glibc: None,
            musl: None,
        };
        assert_eq!(none.to_string(), "glibc <not detected> | musl <not detected>");
    }
}
