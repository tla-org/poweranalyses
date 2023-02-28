///
/// Logic to transfer strings between Javascript and WebAssembly.
/// Thanks to Richard L. Apodaca at https://depth-first.com.
///
use std::mem;
use std::ffi::CStr;
use std::os::raw::c_void;

#[no_mangle]
pub extern "C" fn alloc() -> *mut c_void {
    let mut buf = Vec::with_capacity(1024);
    let ptr = buf.as_mut_ptr();

    mem::forget(buf);

    ptr
}

#[no_mangle]
pub unsafe extern "C" fn dealloc(ptr: *mut c_void) {
    let _ = Vec::from_raw_parts(ptr, 0, 1024);
}

pub unsafe fn u8_to_string(ptr: *mut u8) -> String {
    let text = CStr::from_ptr(ptr as *const i8).to_str().unwrap();
    text.to_string()
}
