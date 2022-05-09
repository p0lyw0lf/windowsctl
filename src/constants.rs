//! This module contains constants for efi variables and Windows system privileges.

use std::ffi::{CStr, CString};

fn str_to_cstr(s: &'static str) -> &'static CStr {
    let cstring = CString::new(s).expect("&'static str should have valid UTF-8");
    let boxed_cstr = cstring.into_boxed_c_str();
    Box::<CStr>::leak(boxed_cstr)
}

lazy_static::lazy_static! {
    /// See documentation at https://github.com/systemd/systemd/blob/66a0e222937e145a1722640fab413608f565cf33/docs/BOOT_LOADER_INTERFACE.md
    pub static ref SYSTEMD_LOADER_VENDOR_GUID: &'static CStr = str_to_cstr("{4a67b082-0a4c-41cf-b6c7-440b29bb8c4f}");
    pub static ref DEFAULT_VAR_NAME: &'static CStr = str_to_cstr("LoaderEntryDefault");
    pub static ref ONESHOT_VAR_NAME: &'static CStr = str_to_cstr("LoaderEntryOneShot");
    // pub static ref FEATURES_VAR_NAME: &'static CStr = str_to_cstr("LoaderFeatures");

    /// See https://docs.microsoft.com/en-us/windows/win32/secauthz/privilege-constants for the
    /// definition of this value, and <https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getfirmwareenvironmentvariablew#remarks>
    /// for why we need this privilege
    pub static ref SE_SYSTEM_ENVIRONMENT_NAME: &'static CStr = str_to_cstr("SeSystemEnvironmentPrivilege");
    pub static ref RUNAS: &'static CStr = str_to_cstr("runas");
}
