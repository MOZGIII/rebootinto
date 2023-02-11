//! Build script to include the manifest into the executable on Windows.

fn main() {
    println!("cargo:rerun-if-changed=manifest.rc");
    embed_resource::compile("manifest.rc", embed_resource::NONE);
}
