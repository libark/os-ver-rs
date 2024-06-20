#[cfg(target_os = "ios")]
pub mod ios;
#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "macos")]
pub mod macos;
#[allow(unused)]
#[cfg(target_os = "windows")]
pub mod windows;

use lazy_static::lazy_static;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub build: u32,
}

type VersionTuple = (u32, u32, u32, u32);

lazy_static! {
    pub static ref OS_VERSION: Version = {
        #[cfg(target_os = "ios")]
        let version = ios::get_version();
        #[cfg(target_os = "linux")]
        let version = linux::get_version();
        #[cfg(target_os = "macos")]
        let version = macos::get_version();
        #[cfg(target_os = "windows")]
        let version = windows::get_version();
        #[cfg(not(any(target_os = "ios", target_os = "linux", target_os = "macos", target_os = "windows")))]
        let version = Version::default();

        version
    };
}

impl Version {
    pub const fn new(major: u32, minor: u32, patch: u32, build: u32) -> Self {
        Self {
            major,
            minor,
            patch,
            build,
        }
    }

    pub const fn from((major, minor, patch, build): VersionTuple) -> Self {
        Self {
            major,
            minor,
            patch,
            build,
        }
    }
}

impl From<Version> for VersionTuple {
    fn from(version: Version) -> Self {
        (version.major, version.minor, version.patch, version.build)
    }
}

impl From<VersionTuple> for Version {
    fn from(tuple: VersionTuple) -> Self {
        Self::from(tuple)
    }
}

#[macro_export]
macro_rules! if_greater_than {
    (($major:expr) => $block:block $(else $else_block:block)?) => {
        if_greater_than!(($major, 0, 0, 0) => $block $(else {$else_block})?)
    };
    (($major:expr, $minor:expr) => $block:block $(else $else_block:block)?) => {
        if_greater_than!(($major, $minor, 0, 0) => $block $(else {$else_block})?)
    };
    (($major:expr, $minor:expr, $patch:expr) => $block:block $(else $else_block:block)?) => {
        if_greater_than!(($major, $minor, $patch, 0) => $block $(else {$else_block})?)
    };
    (($major:expr, $minor:expr, $patch:expr, $build:expr) => $block:block $(else $else_block:block)?) => {
        if_greater_than!(Version { major: $major, minor: $minor, patch: $patch, build: $build } => $block $(else {$else_block})?)
    };
    ($version:expr => $block:block $(else $else_block:block)?) => {
        if *OS_VERSION >= $version {
            $block
        } $(else {
            $else_block
        })?
    };
}
