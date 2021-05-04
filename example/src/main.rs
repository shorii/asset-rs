use asset::init;
use std::str;

include!(concat!(env!("OUT_DIR"), "/asset.rs"));

fn byte_to_string(buf: &[u8]) -> String {
    match str::from_utf8(buf) {
        Ok(v) => String::from(v),
        _ => panic!("invalid byte"),
    }
}

fn main() -> Result<(), std::io::Error> {
    let asset = init!();
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
    let file = asset.find("assets/test3.txt");
    println!("{:?}", file);

    let file = asset.find("assets/data/test1.txt");
    println!("{:?}", file);
    Ok(())
}
