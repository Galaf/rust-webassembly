use std::mem;
use std::os::raw::c_void;
use std::slice;

mod mandelbrot;

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buffer = Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);
    return pointer as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(pointer: *mut c_void, capacity: usize) {
    unsafe {
        let _buffer = Vec::from_raw_parts(pointer, 0, capacity);
    }
}

#[no_mangle]
pub extern "C" fn fill(pointer: *mut u8, max_width: usize, max_height: usize) {
    let byte_size = max_width * max_height * 4;
    let s1 = unsafe { slice::from_raw_parts_mut(pointer, byte_size) };

    let canvas = mandelbrot::draw(max_width, max_height);

    for i in 0..byte_size {
        let x = i / 4 % max_width;
        let y = 1 / 4 / max_width;

        s1[i] = canvas[x][y] as u8;
    }
}
