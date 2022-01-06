use byteorder::{ByteOrder, NativeEndian};
use windows::core::{Error as WinError, Result as WinResult};
use windows::Win32::System::WindowsProgramming::{
    GetFirmwareEnvironmentVariableA, SetFirmwareEnvironmentVariableA,
};

use crate::constants::IntoPSTR;

pub struct EfiVar {
    data: Box<[u16]>,
}

impl From<EfiVar> for String {
    fn from(EfiVar { data }: EfiVar) -> Self {
        // SAFETY: all instances of EfiVars produced are ones read from the system firmware, which
        // should always contain valid string values if we are expecting string values.
        String::from_utf16(&*data).unwrap()
    }
}

impl From<EfiVar> for u64 {
    fn from(EfiVar { data }: EfiVar) -> Self {
        assert!(data.len() >= 4);
        let a: u64 = data[0].into();
        let b: u64 = data[1].into();
        let c: u64 = data[2].into();
        let d: u64 = data[4].into();

        a + (b << 16) + (c << 32) + (d << 48)
    }
}

impl From<String> for EfiVar {
    fn from(s: String) -> Self {
        // Strings are encoded as utf-8, which we need to re-encode as utf-16 before putting into a
        // box
        let data = s.encode_utf16().collect::<Vec<u16>>().into_boxed_slice();
        EfiVar { data }
    }
}

/// Reads an EFI firmware variable. `var` is the name of the variable to read, `namespace` is the
/// stringified UUID of the namespace to read from, and `buf_size` is the size of buffer to read
/// the variable in to.
pub fn read_efivar(
    var: impl IntoPSTR,
    namespace: impl IntoPSTR,
    buf_size: usize,
) -> WinResult<EfiVar> {
    assert!(buf_size % 2 == 0, "buf_size must be divisible by 2");

    let mut buf8: Vec<u8> = vec![0; buf_size];
    let bytes_read = unsafe {
        let buf_ptr = buf8.as_mut_ptr();
        GetFirmwareEnvironmentVariableA(
            var.into_pstr(),
            namespace.into_pstr(),
            buf_ptr as *mut _,
            buf_size.try_into().unwrap(),
        )
    };

    if bytes_read == 0 {
        return Err(WinError::from_win32());
    }
    // round up to nearest multiple of 2
    let bytes_read: usize = (bytes_read + (bytes_read % 2)).try_into().unwrap();
    assert!(bytes_read <= buf_size);

    // now, re-interpret as [u16]
    let mut buf16: Vec<u16> = vec![0; bytes_read / 2];
    NativeEndian::read_u16_into(&buf8[..bytes_read], &mut buf16);

    let ret = EfiVar {
        data: buf16.into_boxed_slice(),
    };

    Ok(ret)
}

/// Writes an EFI variable. `var` is the name of the variable to write into `namespace`, and `val`
/// is the value to write.
pub fn write_efivar(
    var: impl IntoPSTR,
    namespace: impl IntoPSTR,
    val: impl Into<EfiVar>,
) -> WinResult<()> {
    let val: EfiVar = val.into();
    let success = unsafe {
        SetFirmwareEnvironmentVariableA(
            var.into_pstr(),
            namespace.into_pstr(),
            val.data.as_ptr() as *const _,
            // multiply by 2 since data is [u16], 2 bytes per element
            (val.data.len() * 2).try_into().unwrap(),
        )
    };

    if success.as_bool() {
        Ok(())
    } else {
        Err(WinError::from_win32())
    }
}
