fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/blobstore.cc")
        .flag_if_supported("-std=c++14")
        .compile("cxx-test");

    // include this to update build on changes
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/blobstore.cc");
    println!("cargo:rerun-if-changed=include/blobstore.h");
}
