extern crate image;
extern crate xterm256_converter;

use xterm256_converter::*;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let converted = convert_to_unicode_from_file("examples/ferris.png").unwrap();
    let mut file = File::create("ferris.colseq").unwrap();
    file.write_all(&converted).unwrap();
    println!("Now try \"cat ferris.colseq\"");
}
