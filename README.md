# Hardkernel ODROID Smart Power - Rust Implementation

This Rust crate provides an interface for managing an [ODROID Smart Power](http://www.hardkernel.com/main/products/prdt_info.php?g_code=G137361754360) device.
It is roughly based on the [hosp](https://github.com/energymon/hosp) C library.


## Dependencies

You need an ODROID Smart Power device with a USB connection.

We use the [hidapi](https://crates.io/crates/hidapi) crate for managing the USB interface.
On Linux, you will need `libusb-1.0` headers installed and discoverable by `pkg-config` so the native [hidapi](https://github.com/signal11/hidapi/) library can be compiled (On Ubuntu, install `libusb-1.0-0-dev`).


## Usage
Add `hosp` as a dependency in `Cargo.toml`:

```toml
[dependencies]
hosp = "0.1"
```


## Project Source

Find this and related project sources at the [energymon organization on GitHub](https://github.com/energymon).  
This project originates at: https://github.com/energymon/hosp-rs

Bug reports and pull requests for bug fixes and enhancements are welcome.
