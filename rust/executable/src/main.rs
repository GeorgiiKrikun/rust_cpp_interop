use std::ffi::CString;
use std::fs;
use std::os::raw::{c_char, c_int};
use std::path::Path;
use clap::Parser;
use std::error::Error;

#[path="generated/bindings.rs"]
mod bindings;

use bindings::UserProfile;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String,
}

unsafe extern "C" {
    unsafe fn save_image_from_rust(
        data: *const u8,
        width: c_int,
        height: c_int,
        channels: c_int,
        output_path: *const c_char,
    ) -> c_int;
    unsafe fn print_user_from_rust(user: *const UserProfile);
}

fn check_input(p: &Path) {
    if !p.is_file() {
        panic!("Input path is not a file: {:?}", p);
    }
    
    if !p.exists() {
        panic!("Input file does not exist: {:?}", p);
    }
}

    

// The main function is IDENTICAL to the previous example.
fn main() -> Result<(), Box<dyn Error> > {
    let args = Args::parse();
    let input_img_pth = Path::new(&args.input);
    check_input(input_img_pth);

    let in_fname = input_img_pth.
        file_name().
        ok_or("Can't get filename")?.
        to_str().
        ok_or("Can't convert filename to string")?;

    let out_img_str : &str = &args.output;
    let out_img_pth = Path::new(out_img_str);
    let out_img_dir = out_img_pth.parent().ok_or("Can't get parent dir")?;

    if !out_img_dir.exists() {
        fs::create_dir_all(out_img_dir)?;
    }

    println!("[Rust] Loading image: {:?}", input_img_pth);
    let img = image::open(input_img_pth).unwrap();
    let rgb_img = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();
    let raw_data = rgb_img.as_raw();
    println!("Loaded img W={} H={}", width, height);
    let c_out_path = CString::new(out_img_str);
    let result = unsafe {
        save_image_from_rust(
            raw_data.as_ptr(),
            width as c_int,
            height as c_int,
            3, // RGB8 has 3 channels
            c_out_path?.as_ptr(),
        )
    };
    if result == 0 {
        println!("[Rust] Call to C++ succeeded for");
    } else {
        println!("[Rust] Call to C++ failed for");
    }

    println!("Try to print user profile from C++:");
    let up = UserProfile {
        user_id: 42,
        score: 99.5,
        is_active: true,
        name: CString::new(in_fname)?.into_raw(), // Convert Rust string to C string
    };

    unsafe {
        print_user_from_rust(&up);
    }
    // println!("---");

    Ok(())
}
