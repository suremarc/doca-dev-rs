extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=/opt/mellanox/doca/lib");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=doca");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    let mut builder = bindgen::Builder::default();
    // for whitelisted_type in ["NtNetStreamRx_t", "NtNetBuf_t", "NtErrorCodes_e"] {
    //     builder = builder.allowlist_type(whitelisted_type);
    // }

    // for whitelisted_func in ["NT_Init", "NT_Done", "NT_ExplainError", "NT_NetRx.*"] {
    //     builder = builder.allowlist_function(whitelisted_func);
    // }

    // for whitelisted_var in [
    //     "NT_ERRBUF_SIZE",
    //     "NTAPI.*",
    //     "NT_NET_GET_SEGMENT_TIMESTAMP_TYPE",
    //     "NT_TIMESTAMP.*",
    // ] {
    //     builder = builder.allowlist_var(whitelisted_var);
    // }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = builder
        .clang_arg("-I/opt/mellanox/doca/include")
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}