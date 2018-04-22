extern crate image;
extern crate xterm256_converter;

use xterm256_converter::*;

fn main() {
    let converted = convert_to_unicode_from_file("examples/ferris.png").unwrap();
    print!("{}", String::from_utf8(converted).unwrap());
}
