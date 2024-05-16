#[cfg(target_os = "macos")]
use os_ver::macos;
#[cfg(target_os = "windows")]
use os_ver::windows;
use os_ver::{if_greater_than, Version, OS_VERSION};

#[cfg(target_os = "macos")]
fn print_os_info() {
    println!("macOS version: {:?}", *OS_VERSION);
    if *OS_VERSION >= macos::SONOMA {
        println!("macOS Sonoma");
    } else if *OS_VERSION >= macos::VENTURA {
        println!("macOS Ventura");
    } else if *OS_VERSION >= macos::MONTEREY {
        println!("macOS Monterey");
    } else if *OS_VERSION >= macos::BIG_SUR {
        println!("macOS Big Sur");
    } else if *OS_VERSION >= macos::CATALINA {
        println!("macOS Catalina");
    } else if *OS_VERSION >= macos::MOJAVE {
        println!("macOS Mojave");
    } else if *OS_VERSION >= macos::HIGH_SIERRA {
        println!("macOS High Sierra");
    } else if *OS_VERSION >= macos::SIERRA {
        println!("macOS Sierra");
    } else if *OS_VERSION >= macos::EL_CAPITAN {
        println!("macOS El Capitan");
    } else if *OS_VERSION >= macos::YOSEMITE {
        println!("macOS Yosemite");
    } else if *OS_VERSION >= macos::MAVERICKS {
        println!("macOS Mavericks");
    } else if *OS_VERSION >= macos::MOUNTAIN_LION {
        println!("macOS Mountain Lion");
    } else if *OS_VERSION >= macos::LION {
        println!("macOS Lion");
    } else if *OS_VERSION >= macos::SNOW_LEOPARD {
        println!("macOS Snow Leopard");
    } else if *OS_VERSION >= macos::LEOPARD {
        println!("macOS Leopard");
    } else if *OS_VERSION >= macos::TIGER {
        println!("macOS Tiger");
    } else {
        println!("macOS Panther or older");
    }
}

#[cfg(target_os = "windows")]
fn print_os_info() {
    println!("Windows version: {:?}", *OS_VERSION);
    if *OS_VERSION >= windows::WIN11 {
        println!("Windows 11");
    } else if *OS_VERSION >= windows::WIN10 {
        println!("Windows 10");
    } else if *OS_VERSION >= Version::new(10, 0, 0, 10586) {
        println!("Windows 10 TH2");
    } else if *OS_VERSION >= windows::WINBLUE {
        println!("Windows 8.1");
    } else if *OS_VERSION >= windows::WIN8 {
        println!("Windows 8");
    } else if *OS_VERSION >= windows::WIN7 {
        println!("Windows 7");
    } else if *OS_VERSION >= windows::WIN7_SP1 {
        println!("Windows 7 SP1");
    } else if *OS_VERSION >= windows::VISTA {
        println!("Windows Vista");
    } else if *OS_VERSION >= windows::WINXP_SP3 {
        println!("Windows XP SP3");
    } else if *OS_VERSION >= windows::WINXP_SP2 {
        println!("Windows XP SP2");
    } else if *OS_VERSION >= windows::WINXP_SP1 {
        println!("Windows XP SP1");
    } else if *OS_VERSION >= windows::WINXP {
        println!("Windows XP");
    } else {
        println!("Windows 2000 or older");
    }
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn print_os_info() {
    println!("OS version: {:?}", *OS_VERSION);
}

fn main() {
    print_os_info();

    if_greater_than!((16) => {
        println!("OS version 16 or later");
    } else {
        println!("OS versions earlier than 16");
    });

    if_greater_than!((10, 10) => {
        println!("OS version 10.10 or later");
    } else {
        println!("OS versions earlier than 10.10");
    });

    if_greater_than!((10, 0, 0, 1) => {
        println!("OS version 10.0.0.1 or later");
    });

    #[cfg(target_os = "windows")]
    if_greater_than!(windows::WIN11 => {
        println!("Windows 11 or later");
    } else {
        println!("earlier than Windows 11");
    });

    #[cfg(target_os = "macos")]
    if_greater_than!(macos::SONOMA => {
        println!("macOS Sonoma or later");
    } else {
        println!("earlier than macOS Sonoma");
    });
}
