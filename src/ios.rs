use objc2::{
    class,
    encode::{Encoding, RefEncode},
    msg_send_id,
    rc::Id,
    runtime::NSObject,
    Message,
};
use objc2_foundation::NSString;

use crate::Version;

#[link(name = "UIKit", kind = "framework")]
extern "C" {}

struct UIDevice;

unsafe impl RefEncode for UIDevice {
    const ENCODING_REF: Encoding = NSObject::ENCODING_REF;
}

unsafe impl Message for UIDevice {}

impl UIDevice {
    fn current_device() -> Id<UIDevice> {
        unsafe { msg_send_id![class!(UIDevice), currentDevice] }
    }

    fn system_version(&self) -> Id<NSString> {
        unsafe { msg_send_id![self, systemVersion] }
    }
}

pub(crate) fn get_version() -> Version {
    let ver_str = UIDevice::current_device().system_version().to_string();
    let mut ver_numbers = ver_str.split('.').map(|s| s.parse().unwrap_or(0));
    let major = ver_numbers.next().unwrap_or(0);
    let minor = ver_numbers.next().unwrap_or(0);
    let patch = ver_numbers.next().unwrap_or(0);
    Version::new(major, minor, patch, 0)
}
