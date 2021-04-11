use std::str;

include!(concat!(env!("OUT_DIR"), "/asset.rs"));

fn byte_to_string(buf: &[u8]) -> String {
    match str::from_utf8(buf) {
        Ok(v) => String::from(v),
        _ => panic!("invalid byte"),
    }
}

fn main() -> Result<(), std::io::Error> {
    let asset = init_asset();
    println!("dirname: {:?}", asset.name());
    for dir in asset.dirs() {
        println!("\tdirname: {:?}", dir.name());
        for file in dir.files() {
            println!("\t\tfilename: {:?}", file.name());
            println!("\t\tcontent : {:?}", byte_to_string(&*file.content()));
        }
    }
    for file in asset.files() {
        println!("\tfilename: {:?}", file.name());
        println!("\tcontent : {:?}", byte_to_string(&*file.content()));
    }
    Ok(())
}
