//! Provides data validators used by `Fields`.
use regex::Regex;
use std::any::Any;
use std::fmt::Debug;
use std::ops::Deref;
use std::path::Path;

/// Adds behaviour of validation.
pub trait Validator: Debug {
    /// Validates data returning None (when Ok) or String with error.
    fn validate(&self, data: &str) -> Option<String>;
    /// Allows downcasting `self` to a `Any`.
    fn as_any(&self) -> &Any;
}

/// Ensures data is included.
///
/// # Examples
///
/// ```
/// use fui::validators::Required;
/// use fui::validators::Validator;
///
/// assert_eq!(Required.validate("some-data"), None);
/// assert_eq!(Required.validate(""), Some("Field is required".to_string()));
/// ```
#[derive(Clone, Debug)]
pub struct Required;

impl Validator for Required {
    fn validate(&self, data: &str) -> Option<String> {
        if data.len() == 0 {
            Some("Field is required".to_string())
        } else {
            None
        }
    }

    fn as_any(&self) -> &Any {
        self
    }
}

/// Ensures path is free.
///
/// # Examples
///
/// ```
/// extern crate fui;
///
/// use fui::validators::PathFree;
/// use fui::validators::Validator;
///
/// # fn main() {
/// assert_eq!(PathFree.validate("./free-path"), None);
/// assert_eq!(PathFree.validate("./"), Some("Path is already used".to_string()));
/// # }
///
/// ```
#[derive(Clone, Debug)]
pub struct PathFree;

impl Validator for PathFree {
    fn validate(&self, data: &str) -> Option<String> {
        let path = Path::new(data);
        if path.exists() {
            Some("Path is already used".to_string())
        } else {
            None
        }
    }

    fn as_any(&self) -> &Any {
        self
    }
}

/// Ensures data is dir path which exists.
///
/// # Examples
///
/// ```
/// extern crate fui;
///
/// use fui::validators::DirExists;
/// use fui::validators::Validator;
///
/// # fn main() {
/// assert_eq!(DirExists.validate("./src"), None);
/// assert_eq!(DirExists.validate("./Cargo.toml"), Some("It's not a dir".to_string()));
/// assert_eq!(DirExists.validate("./missing-dir").unwrap(), "Dir doesn't exist");
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct DirExists;

impl Validator for DirExists {
    fn validate(&self, data: &str) -> Option<String> {
        let path = Path::new(data);
        if path.exists() {
            if path.metadata().unwrap().is_dir() {
                None
            } else {
                Some("It's not a dir".to_string())
            }
        } else {
            Some("Dir doesn't exist".to_string())
        }
    }

    fn as_any(&self) -> &Any {
        self
    }
}

/// Ensures data is file path which exists.
///
/// # Examples
///
/// ```
/// extern crate fui;
///
/// use fui::validators::FileExists;
/// use fui::validators::Validator;
///
/// # fn main() {
/// assert_eq!(FileExists.validate("./Cargo.toml"), None);
/// assert_eq!(FileExists.validate("./missing-file"), Some("File doesn't exist".to_string()));
/// assert_eq!(FileExists.validate("./src"), Some("It's not a file".to_string()));
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct FileExists;

impl Validator for FileExists {
    fn validate(&self, data: &str) -> Option<String> {
        let path = Path::new(data);
        if path.exists() {
            if path.metadata().unwrap().is_file() {
                None
            } else {
                Some("It's not a file".to_string())
            }
        } else {
            Some("File doesn't exist".to_string())
        }
    }

    fn as_any(&self) -> &Any {
        self
    }
}

/// Ensures value is one of provided options.
///
/// # Examples
///
/// ```
/// use fui::validators::OneOf;
/// use fui::validators::Validator;
///
/// let v = OneOf(vec!["a", "b"]);
/// assert_eq!(v.validate("a"), None);
/// assert_eq!(v.validate("xxx"), Some("Value must be one of options".to_string()));
/// ```
#[derive(Clone, Debug)]
pub struct OneOf<T>(pub T);

impl<T: Debug + 'static> Validator for OneOf<Vec<T>>
where
    T: Deref<Target = str>,
{
    fn validate(&self, data: &str) -> Option<String> {
        if let None = self.0.iter().position(|x| &**x == data) {
            Some("Value must be one of options".to_string())
        } else {
            None
        }
    }

    fn as_any(&self) -> &Any {
        self
    }
}

impl Validator for Regex {
    fn validate(&self, data: &str) -> Option<String> {
        if self.is_match(data) {
            None
        } else {
            Some(format!(
                "Value {:?} does not match: \"{:?}\" regular exp.",
                data, self
            ))
        }
    }

    fn as_any(&self) -> &Any {
        self
    }
}
