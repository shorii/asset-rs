include!(concat!(env!("OUT_DIR"), "/asset.rs"));

fn main() -> Result<(), std::io::Error> {
    let asset = init_asset();
    println!("{:?}", asset);
    Ok(())
}
