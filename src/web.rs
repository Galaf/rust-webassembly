use std::ffi::CString;
use std::mem;
use std::os::raw::{c_char, c_void};

mod mandelbrot;

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, capacity: usize) {
    unsafe {
        let _buf = Vec::from_raw_parts(ptr, 0, capacity);
    }
}

#[no_mangle]
pub extern "C" fn fill(pointer: *mut u8, max_width: usize, max_height: usize) -> *mut c_char {
    let canvas = mandelbrot::draw(max_width, max_height);
    let s = format!("{?}", canvas);

    return CString::new(s).unwrap().into_raw();
}
