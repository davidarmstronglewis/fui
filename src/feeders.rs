//! Data providers for `views` with suggestion feature.
//!
//! `Views` with suggestion feature:
//! * [Autocomplete]
//! * [Multiselect]
//!
//! [Autocomplete]: ../views/struct.Autocomplete.html
//! [Multiselect]: ../views/struct.Multiselect.html

use dirs;
use glob::{glob_with, MatchOptions};
use std::fmt::Display;
use std::fs;
use std::path::Path;
use std::rc::Rc;

/// Makes data querable.
pub trait Feeder: 'static {
    /// Returns data filtered by `text`, `position` limited to `items_count`.
    fn query(&self, text: &str, position: usize, items_count: usize) -> Vec<String>;
}

#[derive(Clone, Debug)]
enum DirItemType {
    Dir,
    All,
}

/// Query file system for dirs, files, etc.
///
/// ```
/// # extern crate fui;
/// # use fui::feeders::DirItems;
/// # fn main() {
///
/// // Available in two variants:
/// let files_and_dirs = DirItems::new(); // suggests files and dirs
/// let only_dirs = DirItems::dirs(); // suggests only dirs
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct DirItems {
    dir_item_type: DirItemType,
    use_full_paths: bool,
}

impl DirItems {
    /// Creates a new `DirItems` which suggests files and dirs.
    pub fn new() -> Self {
        DirItems {
            dir_item_type: DirItemType::All,
            use_full_paths: false,
        }
    }
    /// Creates a new `DirItems` which suggests only dirs.
    pub fn dirs() -> Self {
        DirItems {
            dir_item_type: DirItemType::Dir,
            use_full_paths: false,
        }
    }

    /// Makes suggestion to be absolute paths (like `/home/user`).
    pub fn use_full_paths(mut self) -> Self {
        self.use_full_paths = true;
        self
    }
}

/// Add star to last component of path.
fn add_glob<P: AsRef<str>>(path: P) -> String {
    if path.as_ref().ends_with("/") {
        return format!("{}*", path.as_ref());
    }
    let as_path = Path::new(path.as_ref());
    if let Some(c) = as_path.components().last() {
        let last = c.as_os_str().to_str().unwrap();
        let converted = if !last.contains('*') {
            let last = if last == "/" {
                format!("{}*", last)
            } else {
                format!("*{}*", last)
            };
            as_path.with_file_name(last)
        } else {
            as_path.to_path_buf()
        };
        format!("{}", converted.display())
    } else {
        "*".to_string()
    }
}

impl Feeder for DirItems {
    fn query(&self, text: &str, position: usize, items_count: usize) -> Vec<String> {
        let path = if text == "" {
            format!("./")
        } else if text.starts_with('~') {
            // TODO: remove unwraps
            let path = text.replace("~", dirs::home_dir().unwrap().to_str().unwrap());
            format!("{}", path)
        } else {
            format!("{}", text)
        };
        let path = add_glob(path);
        if let Ok(v) = glob_with(
            &path,
            &MatchOptions {
                case_sensitive: text.chars().any(|c| c.is_uppercase()),
                require_literal_separator: false,
                require_literal_leading_dot: true,
            },
        ) {
            v.filter(|x| {
                if let Err(e) = x.as_ref() {
                    eprintln!("{:?}", e);
                    false
                } else {
                    true
                }
            }).filter(|x| {
                    let path = x.as_ref().unwrap().metadata().unwrap();
                    match self.dir_item_type {
                        DirItemType::Dir => path.is_dir(),
                        DirItemType::All => true,
                    }
                })
                .map(|x| {
                    let path = x.unwrap();
                    let path = if self.use_full_paths {
                        fs::canonicalize(path).unwrap()
                    } else {
                        path
                    };
                    let text = format!("{}", path.display());
                    text
                })
                .skip(position)
                .take(items_count)
                .collect()
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::fs;
    use std::iter::FromIterator;

    fn expected(start: &str) -> HashSet<String> {
        let found = {
            if let Ok(v) = fs::read_dir(start) {
                v.filter(|x| {
                    if let Ok(entry) = x.as_ref() {
                        if let Some(name) = entry.file_name().to_str() {
                            !name.starts_with(".")
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }).map(|x| {
                        let p = format!("{}", x.as_ref().unwrap().path().display());
                        p.replace("./", "")
                    })
                    .collect()
            } else {
                Vec::new()
            }
        };
        HashSet::<String>::from_iter(found)
    }

    #[test]
    fn test_dir_items_position_works_ok() {
        let di = DirItems::new();
        assert_eq!(di.query("", 0, 1), vec!["CHANGELOG.md"]);
        assert_eq!(di.query("", 1, 1), vec!["Cargo.lock"]);
    }

    #[test]
    fn test_glob_is_added_ok() {
        assert_eq!(add_glob(""), "*");
        assert_eq!(add_glob("/"), "/*");
        assert_eq!(add_glob("/home/"), "/home/*");
        assert_eq!(add_glob("/home/user/xxx"), "/home/user/*xxx*");
        assert_eq!(add_glob("/home/user/*xxx"), "/home/user/*xxx");
        assert_eq!(add_glob("/home/user/xxx*"), "/home/user/xxx*");
        assert_eq!(add_glob("**/xxx"), "**/*xxx*");
        assert_eq!(add_glob("**/*xxx"), "**/*xxx");
        assert_eq!(add_glob("**/xxx*"), "**/xxx*");
    }

    #[test]
    fn test_dir_item_works_with_current_dir() {
        let di = DirItems::new();
        let found = di.query("", 0, 100);
        assert_eq!(HashSet::<String>::from_iter(found), expected("./"));
    }

    #[test]
    fn test_dir_item_works_with_current_subdir() {
        let di = DirItems::new();
        let found = di.query("examples/", 0, 100);
        assert_eq!(HashSet::<String>::from_iter(found), expected("./examples"));
    }

    #[test]
    fn test_dir_item_works_with_current_missing_dir() {
        let di = DirItems::new();
        let found = di.query("missing-dir", 0, 10);
        assert_eq!(
            HashSet::<String>::from_iter(found),
            expected("./missing-dir")
        );
    }

    #[test]
    fn test_dir_item_works_with_homedir() {
        let di = DirItems::new();
        let found = di.query("~/", 0, 200);
        let homedir = dirs::home_dir().unwrap();
        assert_eq!(
            HashSet::<String>::from_iter(found),
            expected(homedir.to_str().unwrap())
        );
    }

    #[test]
    fn test_dir_item_works_with_root_dir() {
        let di = DirItems::new();
        let found = di.query("/root", 0, 100);
        assert_eq!(
            HashSet::<String>::from_iter(found),
            HashSet::<String>::from_iter(vec!["/root".to_string()])
        );
    }

    #[test]
    fn test_dir_item_works_with_root_subdir() {
        let di = DirItems::new();
        let found = di.query("/root/", 0, 100);
        assert_eq!(
            HashSet::<String>::from_iter(found),
            HashSet::<String>::new()
        );
    }

    #[test]
    fn test_dir_item_works_with_top_missing_dir() {
        let di = DirItems::new();
        let found = di.query("/missing-dir", 0, 10);
        assert_eq!(
            HashSet::<String>::from_iter(found),
            HashSet::<String>::new()
        );
    }

    #[test]
    fn test_dir_item_works_with_broken_glob() {
        let di = DirItems::new();
        let found = di.query("**.", 0, 10);
        assert_eq!(
            HashSet::<String>::from_iter(found),
            HashSet::<String>::new()
        );
    }
}

impl<T: Display + 'static> Feeder for Vec<T> {
    fn query(&self, text: &str, position: usize, items_count: usize) -> Vec<String> {
        self.iter()
            .map(|x| format!("{}", x))
            .filter(|x| x.to_lowercase().contains(text))
            .skip(position)
            .take(items_count)
            .collect()
    }
}

impl Feeder for Rc<Feeder> {
    fn query(&self, text: &str, position: usize, items_count: usize) -> Vec<String> {
        (**self).query(text, position, items_count)
    }
}
