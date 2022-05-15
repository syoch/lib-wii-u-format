use std::fs;

mod binary_reader;
mod rpx;
mod string_reader;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = binary_reader::BinaryReader::new(fs::read("ram/Minecraft.Client.rpx")?);
    let rpx = rpx::Rpx::parse(reader);

    for section in rpx.section_headers.iter() {
        println!("{}", section);
    }
    Ok(())
}
