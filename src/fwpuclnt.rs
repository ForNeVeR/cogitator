#![allow(dead_code, non_camel_case_types, non_snake_case)]

use winapi::*;

pub const FWPM_SESSION_FLAG_DYNAMIC: UINT32 = 0x00000001;

#[repr(C)]
pub struct FWPM_DISPLAY_DATA0 {
    pub name: *mut wchar_t,
    pub description: *mut wchar_t,
}

#[repr(C)]
pub struct FWPM_SESSION0 {
    pub sessionKey: GUID,
    pub displayData: FWPM_DISPLAY_DATA0,
    pub flags: UINT32,
    pub txnWaitTimeoutInMSec: UINT32,
    pub processId: DWORD,
    pub sid: PSID,
    pub username: *const wchar_t,
    pub kernelMode: BOOL,
}

#[repr(C)]
pub struct FWPM_PROVIDER0 {
    pub providerKey: GUID,
    pub displayData: FWPM_DISPLAY_DATA0,
    pub flags: UINT32,
    pub providerData: FWP_BYTE_BLOB,
    pub serviceName: *mut wchar_t,
}

#[repr(C)]
pub struct FWP_BYTE_BLOB {
    pub size: UINT32,
    pub data: *mut UINT8,
}

extern "system" {
    pub fn FwpmEngineOpen0(
        serverName: *const wchar_t, authnService: UINT32,
        authIdentity: *mut SEC_WINNT_AUTH_IDENTITY_W, session: *const FWPM_SESSION0,
        engineHandle: *mut HANDLE,
    ) -> DWORD;

    pub fn FwpmTransactionBegin0(engineHandle: HANDLE, flags: UINT32) -> DWORD;
    pub fn FwpmTransactionCommit0(engineHandle: HANDLE) -> DWORD;
    pub fn FwpmTransactionAbort0(engineHandle: HANDLE) -> DWORD;
}
