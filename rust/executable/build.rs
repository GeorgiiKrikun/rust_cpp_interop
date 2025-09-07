use std::path::Path;

fn main() {
    // This is the path to the directory where the C++ library was built.
    // For this example, we assume the cpp_project is a sibling directory.
    // In a real project, you might use an environment variable.
    let cpp_path = "../../cpp/";
    let lib_path = "../../cpp/build/";

    // 1. Tell rustc to look for shared libraries in the specified directory.
    println!("cargo:rustc-link-search=native={}", lib_path);

    // 2. Tell rustc to link against the `image_saver` shared library.
    //    - On Linux/macOS, this corresponds to `libimage_saver.so` or `libimage_saver.dylib`.
    //    - On Windows, this corresponds to `image_saver.lib` (the import library).
    println!("cargo:rustc-link-lib=dylib=cv_im_saver");
    
    // Rerun this script if the library file changes.
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
