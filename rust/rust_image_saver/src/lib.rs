use std::slice;
use std::ffi::CStr;
use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn save_image_rust(
    data: *const u8,
    width: i32,
    height: i32,
    channels: i32,
    path: *const c_char,
) {
    if data.is_null() {
        return;
    }
    if channels != 3 {
        return;
    }

    // Safely convert the raw C pointer into a Rust mutable slice.
    // This is the idiomatic way to handle buffers from C.
    let pixel_count = (width * height) as usize;
    let buffer_size = pixel_count * (channels as usize);

    // This block is `unsafe` because we are trusting the C++ caller to provide valid inputs.
    let pixels = unsafe { slice::from_raw_parts(data, buffer_size) };
    let c_str_path = unsafe { CStr::from_ptr(path) };
    let str_path = c_str_path.to_str().unwrap();
    
    println!("[Rust] Saving image size {}x{} to: {}", width, height, str_path);

}
