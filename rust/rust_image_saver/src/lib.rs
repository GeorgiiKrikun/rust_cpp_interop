use std::slice;
use std::ffi::CStr;
use std::os::raw::c_char;
use image::ColorType;

#[unsafe(no_mangle)]
pub extern "C" fn save_image_rust(
    data: *const u8,
    width: i32,
    height: i32,
    channels: i32,
    path: *const c_char,
) -> i32 {
    if data.is_null() {
        return -1;
    }
    if channels != 3 {
        return -2;
    }

    // Safely convert the raw C pointer into a Rust mutable slice.
    // This is the idiomatic way to handle buffers from C.
    let pixel_count = (width * height) as usize;
    let buffer_size = pixel_count * (channels as usize);

    // This block is `unsafe` because we are trusting the C++ caller to provide valid inputs.
    let pixels = unsafe { slice::from_raw_parts(data, buffer_size) };
    let c_str_path = unsafe { CStr::from_ptr(path) };
    let str_path = c_str_path.to_str().unwrap();
    
    let str_path = unsafe {
        // Wrap the C string pointer in a CStr
        let c_str_path = CStr::from_ptr(path);
        // Convert to a Rust string slice, handling potential UTF-8 errors
        match c_str_path.to_str() {
            Ok(s) => s,
            Err(_) => {
                eprintln!("[Rust Error] The provided path contains invalid UTF-8.");
                return -4;
            }
        }
    };

    let color_type = ColorType::Rgb8;
    let buffer_size = (width as usize) * (height as usize) * (channels as usize);
    // Create a safe Rust slice that views the C/C++ owned memory
    let pixels = unsafe { slice::from_raw_parts(data, buffer_size) };

    // --- 3. Save the Image Using the `image` Crate ---
    println!("[Rust] Saving image ({}x{}) to: {}", width, height, str_path);

    match image::save_buffer(str_path, pixels, width as u32, height as u32, color_type) {
        Ok(_) => {
            println!("[Rust] Image saved successfully.");
            0 // Return 0 for success
        }
        Err(e) => {
            eprintln!("[Rust Error] Failed to save image: {}", e);
            -6 // Return a generic error code for save failure
        }
    }



}
