use std::fmt;

pub struct Version {
    pub major: usize,
    pub minor: usize,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

pub struct LibcVersions {
    pub glibc: Option<Version>,
    pub musl: Option<Version>,
}

impl fmt::Display for LibcVersions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let glibc_str = match &self.glibc {
            Some(v) => format!("glibc {v}"),
            None => "glibc <not detected>".to_string(),
        };
        let musl_str = match &self.musl {
            Some(v) => format!("musl {v}"),
            None => "musl <not detected>".to_string(),
        };
        write!(f, "{glibc_str} | {musl_str}")
    }
}
