//! Build script to include the manifest and the icon into the executable on
//! Windows.

fn main() {
    println!("cargo:rerun-if-changed=manifest.rc");
    println!("cargo:rerun-if-changed=../assets/icon.ico");
    embed_resource::compile("manifest.rc", embed_resource::NONE);
}
