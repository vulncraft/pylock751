use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum PylockErrors {
    DeserializationError,
}

#[derive(Deserialize, Serialize, PartialEq, PartialOrd, Debug)]
pub enum PylockVersion {
    #[serde(alias = "1.0")]
    V1_0,
}

#[derive(Deserialize, Serialize)]
pub struct PyPackage {
    name: String,
    version: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Pylock {
    #[serde(alias = "lock-version")]
    pub lock_version: PylockVersion,
    #[serde(alias = "requires-python")]
    pub requires_python: Option<String>,
    pub packages: Vec<PyPackage>,
}

impl FromStr for Pylock {
    type Err = PylockErrors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pylock: Pylock = toml::from_str(s).unwrap();
        Ok(pylock)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_successful_parse() {
        let test_case = include_str!("../tests/corpus/pylock.toml");
        let parsed = Pylock::from_str(test_case).unwrap();
        assert_eq!(parsed.lock_version, PylockVersion::V1_0);
        assert_eq!(parsed.requires_python, Some(">=3.14".into()));
        assert_eq!(parsed.packages.len(), 10);
    }
}
