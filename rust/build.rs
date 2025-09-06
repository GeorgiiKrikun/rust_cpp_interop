fn main() {
    // This is the path to the directory where the C++ library was built.
    // For this example, we assume the cpp_project is a sibling directory.
    // In a real project, you might use an environment variable.
    let lib_path = "../cpp/build/";

    // 1. Tell rustc to look for shared libraries in the specified directory.
    println!("cargo:rustc-link-search=native={}", lib_path);

    // 2. Tell rustc to link against the `image_saver` shared library.
    //    - On Linux/macOS, this corresponds to `libimage_saver.so` or `libimage_saver.dylib`.
    //    - On Windows, this corresponds to `image_saver.lib` (the import library).
    println!("cargo:rustc-link-lib=dylib=cv_im_saver");
    
    // Rerun this script if the library file changes.
    println!("cargo:rerun-if-changed={}/cv_im_saver.so", lib_path);
}
