use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ptr;

use winapi::*;

use fwpuclnt::*;

const EMPTY_GUID: GUID = GUID {
    Data1: 0,
    Data2: 0,
    Data3: 0,
    Data4: [0, 0, 0, 0, 0, 0, 0, 0],
};

fn to_wchar(str : &str) -> Vec<u16> {
    OsStr::new(str).encode_wide().chain(once(0)).collect()
}

#[derive(Debug)]
pub struct Engine(pub HANDLE);

pub fn open_session() -> Engine {
    let display_data = FWPM_DISPLAY_DATA0 {
        name: to_wchar("cogitator session").as_mut_ptr(),
        description: ptr::null_mut()
    };
    let session = FWPM_SESSION0 {
        sessionKey: EMPTY_GUID,
        displayData: display_data,
        flags: 0,
        txnWaitTimeoutInMSec: INFINITE,
        processId: 0,
        sid: ptr::null_mut(),
        username: ptr::null(),
        kernelMode: FALSE
    };
    let mut engine: HANDLE = ptr::null_mut();
    let result = unsafe {
        FwpmEngineOpen0(
            ptr::null(),
            RPC_C_AUTHN_DEFAULT,
            ptr::null_mut(),
            &session,
            &mut engine)
    };
    if result != ERROR_SUCCESS {
        panic!("FwpmEngineOpen0 failure: {}", result);
    }

    Engine(engine)
}
