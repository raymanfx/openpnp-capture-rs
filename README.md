# Safe openpnp-capture bindings

[![license](https://img.shields.io/github/license/raymanfx/libv4l-rs?style=for-the-badge)](https://github.com/raymanfx/libv4l-rs/blob/master/LICENSE.txt)

This crate provides safe bindings to the openpnp-capture library for cross-platform camera capture.

## Layout
The `sys` subdir contains the `openpnp_capture_sys` crate which holds the actual FFI bindings wrapping the C API.

## Usage
```toml
openpnp_capture = "0.1"
```

## Example
```rust
use openpnp_capture::{Device, Format, Stream};

fn main() {
    // Fetch some generic device information
    let devices = Device::enumerate();
    println!("Found {} devices.", devices.len());

    // Choose the first device we see
    let dev = Device::new(devices[0]).expect("Failed to open device");

    // Create the stream
    let format = Format::default().width(1280).height(720).fps(30);
    let mut stream = Stream::new(&dev, &format).expect("Failed to create stream");

    // Print some format information
    println!(
        "[0] {} ({}x{}@{})",
        stream.format().fourcc,
        stream.format().width,
        stream.format().height,
        stream.format().fps
    );

    // Prepare a buffer to hold camera frames
    let mut rgb_buffer = Vec::new();

    // Capture some frames
    stream.advance();
    stream.read(&mut rgb_buffer);
}
```

Have a look at the provided `examples` for more sample applications.
