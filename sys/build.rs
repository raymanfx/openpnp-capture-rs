extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    if env::var("CARGO_FEATURE_NATIVE").is_ok() {
        // Tell cargo to tell rustc to link the system openpnp-capture shared
        // library and its dependencies
        println!("cargo:rustc-link-lib=openpnp-capture");
        println!("cargo:rustc-link-lib=turbojpeg");
    } else if env::var("CARGO_FEATURE_VENDOR").is_ok() {
        // Compile the included library distribution
        let out = cmake::build("vendor");

        // Tell cargo to link the static library
        println!(
            "cargo:rustc-link-search=native={}",
            out.join("lib").display()
        );
        println!(
            "cargo:rustc-link-search=native={}",
            out.join("lib64").display()
        );
        println!("cargo:rustc-link-lib=static=openpnp-capture");
    }

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .clang_arg("--include-directory=vendor/include")
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
