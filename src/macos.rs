use objc2::{sel, ClassType};
use objc2_foundation::NSProcessInfo;

use crate::Version;

pub const SEQUOIA: Version = Version::new(15, 0, 0, 0);
pub const SONOMA: Version = Version::new(14, 0, 0, 0);
pub const VENTURA: Version = Version::new(13, 0, 0, 0);
pub const MONTEREY: Version = Version::new(12, 0, 0, 0);
pub const BIG_SUR: Version = Version::new(11, 0, 0, 0);
pub const CATALINA: Version = Version::new(10, 15, 0, 0);
pub const MOJAVE: Version = Version::new(10, 14, 0, 0);
pub const HIGH_SIERRA: Version = Version::new(10, 13, 0, 0);
pub const SIERRA: Version = Version::new(10, 12, 0, 0);
pub const EL_CAPITAN: Version = Version::new(10, 11, 0, 0);
pub const YOSEMITE: Version = Version::new(10, 10, 0, 0);
pub const MAVERICKS: Version = Version::new(10, 9, 0, 0);
pub const MOUNTAIN_LION: Version = Version::new(10, 8, 0, 0);
pub const LION: Version = Version::new(10, 7, 0, 0);
pub const SNOW_LEOPARD: Version = Version::new(10, 6, 0, 0);
pub const LEOPARD: Version = Version::new(10, 5, 0, 0);
pub const TIGER: Version = Version::new(10, 4, 0, 0);

pub(crate) fn get_version() -> Version {
    let process_info = NSProcessInfo::processInfo();
    if NSProcessInfo::class().responds_to(sel!(operatingSystemVersion)) {
        let version = process_info.operatingSystemVersion();
        Version::new(
            version.majorVersion as u32,
            version.minorVersion as u32,
            version.patchVersion as u32,
            0,
        )
    } else {
        let ver_str = unsafe { process_info.operatingSystemVersionString() };
        let ver_str = ver_str.to_string();
        let ver_parts: Vec<&str> = ver_str.split_whitespace().collect();
        let ver_str = match ver_parts.get(1) {
            Some(s) => s,
            None => return Version::default(),
        };
        let mut ver_numbers = ver_str.split('.').map(|s| s.parse().unwrap_or(0));
        let major = ver_numbers.next().unwrap_or(0);
        let minor = ver_numbers.next().unwrap_or(0);
        let patch = ver_numbers.next().unwrap_or(0);
        Version::new(major, minor, patch, 0)
    }
}
