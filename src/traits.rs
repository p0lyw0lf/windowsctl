//! This module contains useful traits for working with strings passed to Windows API calls

use std::ffi::CStr;
use windows::core::{Error as WinError, Result as WinResult};
use windows::Win32::Foundation::{BOOL, E_ACCESSDENIED, E_FAIL, E_INVALIDARG, E_ABORT, E_OUTOFMEMORY, E_UNEXPECTED};
use windows::core::PCSTR;

/// This trait is unsafe since the implementor must guarantee that the resulting PCSTR is managed
/// properly (pointer is never invalid for the duration where it can be dereferenced by Windows
/// API calls).
pub unsafe trait ToPCSTR {
    fn to_pcstr(&self) -> PCSTR;
}

unsafe impl ToPCSTR for PCSTR {
    //! SAFETY: it's already a PCSTR
    fn to_pcstr(&self) -> PCSTR {
        *self
    }
}

/*
/// This trait is unsafe since the implementor must guarantee that the resulting PSTR is managed
/// properly (pointer is never invalid for the duration where it can be dereferenced by Windows
/// API calls).
pub unsafe trait ToPSTR {
    fn to_pstr(&mut self) -> PSTR;
}
*/

unsafe impl ToPCSTR for CStr {
    //! SAFETY: So long as the CStr lives long enough, so will the slice gotten from it
    fn to_pcstr(&self) -> PCSTR {
        PCSTR(self.to_bytes_with_nul().as_ptr())
    }
}

pub trait IntoWinResult {
    type Output;
    fn into_win_result(self) -> WinResult<Self::Output>;
}

impl IntoWinResult for BOOL {
    type Output = ();
    fn into_win_result(self) -> WinResult<Self::Output> {
        if self.as_bool() {
            Ok(())
        } else {
            Err(WinError::from_win32())
        }
    }
}

impl<T> IntoWinResult for std::io::Result<T> {
    type Output = T;
    fn into_win_result(self) -> WinResult<Self::Output> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => {
                use std::io::ErrorKind::*;
                let hresult = match e.kind() {
                    PermissionDenied => E_ACCESSDENIED,
                    InvalidInput => E_INVALIDARG,
                    Interrupted => E_ABORT,
                    OutOfMemory => E_OUTOFMEMORY,
                    UnexpectedEof => E_UNEXPECTED,
                    _ => E_FAIL,
                };
                Err(WinError::new(hresult, format!("{:?}", e).into()))
            }
        }
    }
}
