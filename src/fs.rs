use core::ffi::CStr;
use crate::sys::{fs_read, fs_write, FsInfo};

const FS_OP_READ:  u32 = 0;
const FS_OP_WRITE: u32 = 3;

/// Read a file at `path` into `buf`. Returns number of bytes read, or Err(error_code).
pub fn read_file(path: &CStr, buf: &mut [u8]) -> Result<u32, u32> {
    let mut info = FsInfo {
        operation:  FS_OP_READ,
        offset:     0,
        offset_hi:  0,
        byte_count: buf.len() as u32,
        buffer:     buf.as_mut_ptr(),
        reserved:   0,
        path:       path.as_ptr() as *const u8,
    };
    let result = unsafe { fs_read(&mut info as *mut FsInfo as *const FsInfo) };
    if result == 0 { Ok(info.byte_count) } else { Err(result) }
}

/// Write `buf` to a file at `path`. Returns number of bytes written, or Err(error_code).
pub fn write_file(path: &CStr, buf: &[u8]) -> Result<u32, u32> {
    let mut info = FsInfo {
        operation:  FS_OP_WRITE,
        offset:     0,
        offset_hi:  0,
        byte_count: buf.len() as u32,
        buffer:     buf.as_ptr() as *mut u8,
        reserved:   0,
        path:       path.as_ptr() as *const u8,
    };
    let result = unsafe { fs_write(&mut info as *mut FsInfo as *const FsInfo) };
    if result == 0 { Ok(info.byte_count) } else { Err(result) }
}
