use std::path::Path;

fn main() {
    let cpp_path = "../../cpp/";
    let lib_path = "../../cpp/build/";

    println!("cargo:rustc-link-search=native={}", lib_path);

    println!("cargo:rustc-link-lib=dylib=cv_im_saver");
    
    println!("cargo:rerun-if-changed={}/cv_im_saver.so", lib_path);

    let header_path = Path::new(cpp_path).
        join("include").
        join("user_profile.h");

    // 2. Generate Rust bindings for the C header
    let bindings = bindgen::Builder::default()
        .header(header_path.to_str().expect("Invalid header path"))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_arg("-x")
        .clang_arg("c++")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = Path::new("src/generated/bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
