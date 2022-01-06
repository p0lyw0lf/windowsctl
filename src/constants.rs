use windows::Win32::Foundation::PSTR;

/// See documentation at https://github.com/systemd/systemd/blob/66a0e222937e145a1722640fab413608f565cf33/docs/BOOT_LOADER_INTERFACE.md
pub static SYSTEMD_LOADER_VENDOR_GUID: &'static str = "{4a67b082-0a4c-41cf-b6c7-440b29bb8c4f}";
pub static DEFAULT_VAR_NAME: &'static str = "LoaderEntryDefault";
pub static ONESHOT_VAR_NAME: &'static str = "LoaderEntryOneShot";
// pub static FEATURES_VAR_NAME: &'static str = "LoaderFeatures";

/// See https://docs.microsoft.com/en-us/windows/win32/secauthz/privilege-constants for the
/// definition of this value, and https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getfirmwareenvironmentvariablew#remarks for why we need this privilege
pub static SE_SYSTEM_ENVIRONMENT_NAME: &'static str ="SeSystemEnvironmentPrivilege";

pub trait IntoPSTR {
    /// This function is unsafe since the caller must guarantee that the resulting PSTR is managed
    /// properly (pointer never becomes invalid, memory managed properly).
    unsafe fn into_pstr(self) -> PSTR;
}

impl IntoPSTR for &'static str {
    /// SAFETY: the caller must ensure that functions consuming this PSTR never attempt to modify
    /// it.
    unsafe fn into_pstr(self) -> PSTR {
        PSTR(self.as_ptr() as *mut u8)
    }
}
