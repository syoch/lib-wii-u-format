use std::fs;

mod binary_reader;
mod ghs_demangle;
mod rpx;
mod string_reader;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let funcitons = fs::read("/shared/Resources/minecraft_function_list.txt")?;

    let functions = String::from_utf8(funcitons)?;

    for line in functions.lines() {
        println!("{:?}", ghs_demangle::demangle(line));
    }

    Ok(())
}
