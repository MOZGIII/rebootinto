//! Build script to include the manifest into the executable on Windows.

fn main() {
    embed_resource::compile("manifest.rc");
}
