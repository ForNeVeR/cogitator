extern crate kernel32;

use std::ffi::CString;

use winapi::*;

#[repr(C)]
pub enum WINDIVERT_LAYER {
    WINDIVERT_LAYER_NETWORK = 0,
    WINDIVERT_LAYER_NETWORK_FORWARD = 1
}

pub const WINDIVERT_FLAG_SNIFF: u64 = 1;
pub const WINDIVERT_FLAG_DROP: u64 = 2;
pub const WINDIVERT_FLAG_NO_CHECKSUM: u64 = 1024;

#[link(name="WinDivert")]
extern "C" {
    pub fn WinDivertOpen(
        filter: *const c_char, layer: WINDIVERT_LAYER, priority: i16, flags: u64) -> HANDLE;
}

pub fn open(filter: &str) -> HANDLE {
    let handle = unsafe {
        WinDivertOpen(
            CString::new(filter).unwrap().as_ptr(),
            WINDIVERT_LAYER::WINDIVERT_LAYER_NETWORK,
            0,
            WINDIVERT_FLAG_SNIFF)
    };

    if handle == INVALID_HANDLE_VALUE {
        unsafe {
            panic!(
                "WinDivertOpen returned INVALID_HANDLE_VALUE; error {}",
                kernel32::GetLastError());
        }
    }

    handle
}
