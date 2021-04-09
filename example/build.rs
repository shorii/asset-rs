use asset::builder::AssetBuilder;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let asset_os_file = env::var_os("ASSET_FILE").unwrap();
    if let Some(filename) = asset_os_file.to_str() {
        let init_code = AssetBuilder::build(filename);
        let out_dir = env::var_os("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join("asset.rs");
        fs::write(&dest_path, init_code.to_string()).unwrap();
        println!("cargo:rerun-if-changed=build.rs");
    } else {
        panic!("Failed to retrieve filename");
    }
}
