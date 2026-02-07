use std::{
    fs::read_to_string,
    io::{BufReader, Read},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum PylockErrors {
    #[error("Error parsing pylock.toml")]
    DeserializationError,
    #[error("IO error reading file")]
    IOError(#[source] std::io::Error),
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

impl Pylock {
    /// Read a pylock.toml file into a Pylock
    /// # Examples
    ///
    /// ```
    /// use pylock751::pylock::Pylock;
    /// let mut file = std::fs::File::open("tests/corpus/pylock.toml").unwrap();
    /// let lock_file = Pylock::read_from_file(&mut file);
    ///
    /// ```
    pub fn read_from_file(file: &mut std::fs::File) -> Result<Self, PylockErrors> {
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(PylockErrors::IOError)?;

        Pylock::from_str(&contents)
    }
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
