# Safe openpnp-capture bindings

[![license](https://img.shields.io/github/license/raymanfx/libv4l-rs?style=for-the-badge)](https://github.com/raymanfx/libv4l-rs/blob/master/LICENSE.txt)

This crate provides safe bindings to the openpnp-capture library for cross-platform camera capture.

## Layout
The `sys` subdir contains the `openpnp_capture_sys` crate which holds the actual FFI bindings wrapping the C API.

## Usage
```toml
openpnp_capture = "0.1"
```

Have a look at the provided `examples` for more sample applications.
