use std::fs;

mod binary_reader;
mod formats;
mod string_reader;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = binary_reader::BinaryReader::new(fs::read("ram/Minecraft.Client.rpx")?);
    let rpx = formats::rpx::Rpx::parse(reader);

    Ok(())
}
