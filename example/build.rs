use builder::Builder;
use std::env;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

fn main() {
    let asset_os_file = env::var_os("ASSET_FILE").unwrap();
    if let Some(filename) = asset_os_file.to_str() {
        let init_code = Builder::build(filename);
        let out_dir = env::var_os("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join("asset.rs");
        fs::write(&dest_path, init_code).unwrap();
        println!("cargo:rerun-if-changed=build.rs");
    } else {
        panic!("Failed to retrieve filename");
    }
}
