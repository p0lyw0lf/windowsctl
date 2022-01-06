use windows::core::{Error as WinError, Result as WinResult};
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE, LUID, PSTR},
    Security::{
        AdjustTokenPrivileges, ImpersonateSelf, LookupPrivilegeValueA, SecurityImpersonation,
        LUID_AND_ATTRIBUTES, SE_PRIVILEGE_ENABLED, TOKEN_ADJUST_PRIVILEGES, TOKEN_PRIVILEGES,
        TOKEN_QUERY,
    },
    System::Threading::{GetCurrentThread, OpenThreadToken},
};

use crate::constants::IntoPSTR;

fn get_system_environment_luid() -> WinResult<LUID> {
    let mut out = LUID::default();

    let success = unsafe {
        LookupPrivilegeValueA(
            PSTR(std::ptr::null_mut()),
            crate::constants::SE_SYSTEM_ENVIRONMENT_NAME.into_pstr(),
            &mut out as *mut LUID,
        )
    };

    if success.as_bool() {
        Ok(out)
    } else {
        return Err(WinError::from_win32());
    }
}

fn get_privilege_token_handle() -> WinResult<HANDLE> {
    let mut out = HANDLE::default();

    let success = unsafe {
        // This is required for the current thread to actually have a
        // access token
        ImpersonateSelf(SecurityImpersonation)
    };
    if !success.as_bool() {
        return Err(WinError::from_win32());
    }

    let success = unsafe {
        OpenThreadToken(
            GetCurrentThread(),
            TOKEN_QUERY | TOKEN_ADJUST_PRIVILEGES,
            true,
            &mut out as *mut HANDLE,
        )
    };

    if success.as_bool() {
        Ok(out)
    } else {
        Err(WinError::from_win32())
    }
}

/// When run as the administrator, elevates privileges of the current thread to
/// "SE_SYSTEM_ENVIRONMENT_NAME". Must be rust before any functions in `crate::efi_editor`.
pub fn elevate_privileges() -> WinResult<()> {
    let privilege_luid = get_system_environment_luid()?;
    let token_handle = get_privilege_token_handle()?;

    let privileges = TOKEN_PRIVILEGES {
        PrivilegeCount: 1,
        Privileges: [LUID_AND_ATTRIBUTES {
            Luid: privilege_luid,
            Attributes: SE_PRIVILEGE_ENABLED,
        }],
    };

    let success = unsafe {
        AdjustTokenPrivileges(
            token_handle,
            false,
            &privileges as *const TOKEN_PRIVILEGES,
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    }
    .as_bool();

    let success = unsafe { CloseHandle(token_handle) }.as_bool() && success;
    if success {
        Ok(())
    } else {
        Err(WinError::from_win32())
    }
}
