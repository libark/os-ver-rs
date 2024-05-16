use windows_version;

use crate::Version;

pub const WINXP: Version = Version::new(5, 1, 0, 0);
pub const WINXP_SP1: Version = Version::new(5, 1, 1, 0);
pub const WINXP_SP2: Version = Version::new(5, 1, 2, 0);
pub const WINXP_SP3: Version = Version::new(5, 1, 3, 0);
pub const VISTA: Version = Version::new(6, 0, 0, 0);
pub const WIN7: Version = Version::new(6, 1, 0, 0);
pub const WIN7_SP1: Version = Version::new(6, 1, 1, 0);
pub const WIN8: Version = Version::new(6, 2, 0, 0);
pub const WINBLUE: Version = Version::new(6, 3, 0, 0);
pub const WIN10: Version = Version::new(10, 0, 0, 0);
pub const WIN11: Version = Version::new(10, 0, 0, 22000);

pub(crate) fn get_version() -> Version {
    let os_version = windows_version::OsVersion::current();
    Version::new(os_version.major, os_version.minor, os_version.pack, os_version.build)
}
