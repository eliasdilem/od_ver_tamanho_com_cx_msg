use std::io::Error;
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;

fn win32_string(msg: &str) -> Vec<u16> {
    OsStr::new(msg).encode_wide().chain(once(0)).collect()
}

pub fn cx_msg(rotulo: &str, resposta: &str) -> Result<i32, Error> {
    use winapi::um::winuser::{MessageBoxW, MB_ICONASTERISK, MB_OK};
    use std::ptr::null_mut;
    let rotulo = win32_string(rotulo);
    let resposta = win32_string(resposta);
    let ret = unsafe {
        MessageBoxW(
            null_mut(),
            resposta.as_ptr(),
            rotulo.as_ptr(),
            MB_OK | MB_ICONASTERISK,
        )
    };
    if ret == 0 {
        Err(Error::last_os_error())
    } else {
        Ok(ret)
    }
}
