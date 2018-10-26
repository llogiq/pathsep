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

