use winapi::*;

#[repr(C)]
pub enum WINDIVERT_LAYER {
    WINDIVERT_LAYER_NETWORK = 0,
    WINDIVERT_LAYER_NETWORK_FORWARD = 1
}

#[link(name="WinDivert")]
extern "C" {
    pub fn WinDivertOpen(
        filter: *const c_char, layer: WINDIVERT_LAYER, priority: i16, flags: u64) -> HANDLE;
}
