use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(&["src/types.proto"], &["src/"])?;

    cxx_build::bridge("src/fs_event_lib.rs")
        .file("cpp/src/fs_events.cc")
        .std("c++17")
        // .flag("-mmacosx-version-min=11.0")
        .compile("net-experiments");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=cpp/hello.cc");
    println!("cargo:rerun-if-changed=cpp/hello.h");

    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rustc-link-lib=framework=CoreServices");

    Ok(())
}