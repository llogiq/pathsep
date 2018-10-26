# Get the path separator for your OS

When you want to `include!(_)` generated files, cargo will ask you to
put them in `$OUT_DIR`. However, the "usual" way of

```rust, ignore
include!(concat!(env!("OUT_DIR"), "/somefile.rs"));
```

will fail on some windows systems, because they fail to understand the
`/` path separator. This crate allows you to replace that with:

```rust, ignore
include!(concat!(env!("OUT_DIR"), path_separator!(), "somefile.rs"));
```

This will work on all operating systems.

# Usage 
Add this to your Cargo.toml:
```
pathsep = "0.1"
```
Then you can use `#[macro_use] extern crate pathsep;` in your crate root. As of
Rust 1.30, you can also omit the `#[macro_use]` and
`use pathsep::path_separator;` directly.

## License

This code is licensed under the terms of the Apache License 2.0 or the MIT
license, at your discretion.
