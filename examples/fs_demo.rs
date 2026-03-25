#![no_std]
#![no_main]

use core::ffi::CStr;
use kos::fs::{read_file, write_file};

const FILE_PATH:  &CStr = c"/tmp/0/test.txt";
const WRITE_DATA: &[u8] = b"Hello from Rust on KolibriOS";

#[no_mangle]
fn kol_main() {
    let _ = write_file(FILE_PATH, WRITE_DATA);

    let mut buf = [0u8; 64];
    let _ = read_file(FILE_PATH, &mut buf);
}
