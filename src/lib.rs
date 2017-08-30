use std::fmt;
use std::process::Command;

pub const CURRENT_STABLE: Version = Version {
    major: 1,
    minor: 20,
    patch: 0,
};

pub const CURRENT_BETA: Version = Version {
    major: 1,
    minor: 21,
    patch: 0,
};

pub const CURRENT_NIGHTLY: Version = Version {
    major: 1,
    minor: 22,
    patch: 0,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Version {
        Version {
            major,
            minor,
            patch,
        }
    }

    pub fn from_env_var() -> Option<Version> {
        option_env!("CFG_VERSION").and_then(|s| Self::from_string(&s))
    }

    fn from_string(version_str: &str) -> Option<Version> {
        use std::str::FromStr;
        macro_rules! try_opt {
            ($x: expr) => {
                match $x {
                    Some(Ok(x)) => x,
                    _ => return None,
                }
            }
        }

        let mut nums = version_str.split('.').map(u32::from_str);
        Some(Version::new(try_opt!(nums.next()), try_opt!(nums.next()), try_opt!(nums.next())))
    }

    pub fn to_unstable_tool_string(&self) -> String {
        format!("0.{}{}.{}", self.major, self.minor, self.patch)
    }

    pub fn to_stable_tool_string(&self) -> String {
        format!("{}{}.{}.0", self.major, self.minor, self.patch)
    }
}


pub fn channel() -> Option<&'static str> {
    option_env!("CFG_RELEASE_CHANNEL")
}

pub fn commit_hash() -> Option<String> {
    Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|r| String::from_utf8(r.stdout).ok())
}

pub fn commit_date() -> Option<String> {
    Command::new("git")
        .args(&["log",
                "-1",
                "--date=short",
                "--pretty=format:%cd"])
        .output()
        .ok()
        .and_then(|r| String::from_utf8(r.stdout).ok())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formatting() {
        let v = Version::new(1, 2, 3);
        assert_eq!(&v.to_string(), "1.2.3");
        assert_eq!(&v.to_unstable_tool_string(), "0.12.3");
        assert_eq!(&v.to_stable_tool_string(), "12.3.0");
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Version::from_string("0.0"), None);
        assert_eq!(Version::from_string("0.0.a"), None);
        assert_eq!(Version::from_string("0.."), None);
        assert_eq!(Version::from_string("0.0.0"), Some(Version::new(0, 0, 0)));
        assert_eq!(Version::from_string("3.2.1"), Some(Version::new(3, 2, 1)));
    }
}
