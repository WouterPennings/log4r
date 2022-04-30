# Logr

A stupidly simple logging package for Rust.

The main advantage of Logr over [log](https://docs.rs/log/latest/log/) is the use of functions
instead of macros. This way you can disallow them in `clippy.toml`, making it more suitable for 
debugging.

## How to use

1. First add the crate to `Cargo.toml` by adding `logr = 0.1.0` under your dependencies.
2. Add `use logr::*;` to the top of the file.
3. You can now do something like this: `info(some_info.to_string())`

If only want to use these functions during development, add the following to 
your `clippy.toml` in the root of you project:

```toml
disallowed-methods = [
    { path = "logr::success", reason = "Logs are not allowed in builds" },
    { path = "logr::warning", reason = "Logs are not allowed in builds" },
    { path = "logr::info", reason = "Logs are not allowed in builds" },
    { path = "logr::error", reason = "Logs are not allowed in builds" },
    { path = "logr::critical", reason = "Logs are not allowed in builds" },
    { path = "logr::log", reason = "Logs are not allowed in builds" },
]
```

When you run Clippy, it will give errors/warnings over the use of these functions.