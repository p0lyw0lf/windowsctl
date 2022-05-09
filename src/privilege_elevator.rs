//! A module to help your thread acquire SYSTEM privileges

use windows::core::{Error as WinError, Result as WinResult, PCSTR};
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE, LUID},
    Security::{
        AdjustTokenPrivileges, ImpersonateSelf, LookupPrivilegeValueA, SecurityImpersonation,
        LUID_AND_ATTRIBUTES, SE_PRIVILEGE_ENABLED, TOKEN_ADJUST_PRIVILEGES, TOKEN_PRIVILEGES,
        TOKEN_QUERY,
    },
    System::Threading::{GetCurrentThread, OpenThreadToken},
};

use crate::traits::{ToPCSTR, IntoWinResult};

fn get_system_environment_luid() -> WinResult<LUID> {
    let mut out = LUID::default();

    unsafe {
        LookupPrivilegeValueA(
            PCSTR(std::ptr::null()),
            crate::constants::SE_SYSTEM_ENVIRONMENT_NAME.to_pcstr(),
            &mut out as *mut LUID,
        )
    }.into_win_result()?;

    Ok(out)
}

fn get_privilege_token_handle() -> WinResult<HANDLE> {
    unsafe {
        // This is required for the current thread to actually have a
        // access token
        ImpersonateSelf(SecurityImpersonation)
    }
    .into_win_result()?;

    let mut out = HANDLE::default();
    unsafe {
        OpenThreadToken(
            GetCurrentThread(),
            TOKEN_QUERY | TOKEN_ADJUST_PRIVILEGES,
            true,
            &mut out as *mut HANDLE,
        )
    }
    .into_win_result()?;

    Ok(out)
}

/// When run as the administrator, elevates privileges of the current thread to
/// "SE_SYSTEM_ENVIRONMENT_NAME".
pub fn elevate_thread_to_system() -> WinResult<()> {
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

    let err = if success { None } else { Some(WinError::from_win32()) };

    let success = unsafe { CloseHandle(token_handle) }.as_bool() && success;
    if success {
        Ok(())
    } else {
        // Use the previous error, from the AdjustTokenPrivileges call
        match err {
            Some(e) => Err(e),
            None => Err(WinError::from_win32()),
        }
    }
}
