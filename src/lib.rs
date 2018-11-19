//! A small macro that gives us the path separator of the target system
//!
//! This can be used in conjunction with `include!(..)` or `include_str!(..)`
//! and `concat!(..)` to build paths.
//!
//! # Examples
//!
//! ```rust, ignore
//! #[macro_use] extern crate pathsep;
//!
//! include!(concat!(env!("OUT_DIR"), path_separator!(), "generated.rs"));
//! ```
//!
//! The same can be more succinctly written as:
//!
//! ```rust, ignore
//! #[macro_use] extern crate pathsep;
//!
//! include!(path_join!(env!("OUT_DIR"), "generated.rs"));
//! ```

/// Returns the path separator of the target system as a string.
///
/// On Unices, this is equal to "/", while on Windows it is "\\".
///
/// # Examples
///
/// ```rust
/// #[macro_use] extern crate pathsep;
///
/// assert_eq!(path_separator!(), "/");
/// ```
#[cfg(not(target_os = "windows"))]
#[macro_export]
macro_rules! path_separator {
    {} => { "/" }
}

/// Returns the path separator of the target system as a string.
///
/// On Unices, this is equal to "/", while on Windows it is "\\".
///
/// # Examples
///
/// ```rust
/// #[macro_use] extern crate pathsep;
///
/// assert_eq!(path_separator!(), "\\");
/// ```
#[cfg(target_os = "windows")]
#[macro_export]
macro_rules! path_separator {
    {} => { "\\" }
}

/// Join a path with the correct path separator
///
/// # Examples
///
/// Join a relative path of three args
///
/// ```rust
///# #[macro_use] extern crate pathsep;
/// assert_eq!(join_path!("src","def","impl"),
///     concat!("src", path_separator!(), "def", path_separator!(), "impl"));
/// ```
///
/// Make the path absolute by prepending a `/`
///
/// ```rust
///# #[macro_use] extern crate pathsep;
/// assert_eq!(join_path!(/ "var","tmp"),
///     concat!(path_separator!(), "var", path_separator!(), "tmp"));
/// ```
#[macro_export]
macro_rules! join_path {
    () => { "" };
    (/) => { path_separator!() };
    (/ $($s:expr),*) => { concat!(path_separator!(), join_path!($($s),*)) };
    ($first:expr) => { $first };
    ($first:expr, $( $s:expr ),*) => {
        concat!($first, path_separator!(), join_path!($($s),*));
    };
}
