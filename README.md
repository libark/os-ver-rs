# os-ver

[![Version](https://img.shields.io/crates/v/os-ver)](https://crates.io/crates/os-ver)
[![Documentation](https://docs.rs/os-ver/badge.svg)](https://docs.rs/os-ver)
[![License](https://img.shields.io/badge/License-Apache%202-blue.svg)](LICENSE-APACHE)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE-MIT)

This library can be used to detect version number of operating system, such as Windows, macOS, iOS and Linux.

## Example

```toml
[dependencies]
os-ver = "0.2"
```

```rust
use os_ver::*;

#[cfg(target_os = "windows")]
if_greater_than! {windows::WIN11 => {
    // Code for Windows 11 or newer
} else {
    // Code for older versions
}};

#[cfg(target_os = "macos")]
if_greater_than! {macos::SONOMA => {
    // Code for macOS Sonoma or newer
} else {
    // Code for older versions
}};

if_greater_than! {(10, 0, 0, 1) => {
    // Code for OS version >= 10.0.0.1
}};
```

```rust
let version = os_version();

// Print OS version
println!("OS version: {:?}", version);

#[cfg(target_os = "windows")]
if version >= &windows::WIN11 {
    // Code for Windows 11 or newer 
} else if version >= &windows::WIN10 {
    // Code for Windows 10 or newer
} else if version >= &Version::new(10, 0, 0, 10586) {
    // Code for Windows 10 TH2 or newer
}
```
