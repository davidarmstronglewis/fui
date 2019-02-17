//! Various kinds of helpers.
use std::env;


/// Returns current working dir as String.
pub fn cwd() -> String {
    env::current_dir()
        .map(|p| p.into_os_string().into_string().unwrap())
        .ok()
        .unwrap()
}

/// Returns home-dir path.
pub fn home_dir() -> String {
    dirs::home_dir()
        .map(|p| p.into_os_string().into_string().unwrap())
        .unwrap()
}
