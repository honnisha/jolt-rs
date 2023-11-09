fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("joltc/blobstore.cc")
        .compile("jolt-rs");
}
