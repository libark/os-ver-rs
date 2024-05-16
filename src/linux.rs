use std::{ffi::CStr, mem};

use libc::utsname;

use crate::Version;

pub(crate) fn get_version() -> Version {
    let ver_str = unsafe {
        let mut buf: utsname = mem::zeroed();
        libc::uname(&mut buf);
        CStr::from_ptr(buf.release.as_ptr())
    };
    let ver_str = ver_str.to_str();
    let ver_str = match ver_str {
        Ok(s) => s,
        Err(_) => return Version::default(),
    };
    let ver_parts: Vec<&str> = ver_str.split('-').collect();
    let ver_str = match ver_parts.get(0) {
        Some(s) => s,
        None => return Version::default(),
    };
    let mut ver_numbers = ver_str.split('.').map(|s| s.parse().unwrap_or(0));
    let major = ver_numbers.next().unwrap_or(0);
    let minor = ver_numbers.next().unwrap_or(0);
    let patch = ver_numbers.next().unwrap_or(0);
    let build = ver_numbers.next().unwrap_or(0);
    Version::new(major, minor, patch, build)
}
