use std::env;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

fn main() {
    let asset_os_file = env::var_os("ASSET_FILE").unwrap();
    if let Some(filename) = asset_os_file.to_str() {
        let out_dir = env::var_os("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join("asset.rs");
        let file = File::open(filename).unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = vec![];
        reader.read_to_end(&mut buffer).unwrap();
        let byte_array = buffer
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>();
        let byte_array_length = buffer.len();
        let init_code = format!(
            "
use hex_literal::hex;
use asset::Asset;
pub fn init_asset() -> Asset<{}> {{
    let bytes = hex!({:x?});
    Asset::new(\"{:}\", bytes)
}}
",
            byte_array_length, byte_array, filename,
        );

        fs::write(&dest_path, init_code).unwrap();
        println!("cargo:rerun-if-changed=build.rs");
    } else {
        panic!("Failed to retrieve filename");
    }
}
