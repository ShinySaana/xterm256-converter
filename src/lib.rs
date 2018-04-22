extern crate image;

mod colors;

use image::{ImageBuffer, ImageError, Rgb, Rgba};
use colors::XTERM_COLORS;
use std::u16;

fn diff(color_a: Rgba<u8>, color_b: Rgb<u8>) -> u16 {
    let mut sum: u16 = 0;
    for i in 0..3 {
        sum += (color_a[i] as i16 - color_b[i] as i16).abs() as u16;
    }

    sum
}

fn find_color_code(color: Rgba<u8>) -> u8 {
    let mut lowest_color_diff = u16::MAX;
    let mut closest_color = 0;

    for color_term in XTERM_COLORS.iter() {
        let color_diff = diff(color, color_term.to_rgb());
        if color_diff < lowest_color_diff {
            lowest_color_diff = color_diff;
            closest_color = color_term.id();
        }
    }

    closest_color
}

fn encode(start: &[u8], color_code: &u8) -> Vec<u8> {
    let string = color_code.to_string();
    let string = string.as_bytes();

    let mut encoding: Vec<u8> = Vec::new();
    encoding.extend_from_slice(start);
    encoding.extend_from_slice(string);
    encoding.extend_from_slice(b"m");

    encoding
}

fn encode_foreground(color_code: Option<u8>) -> Vec<u8> {
    match color_code {
        Some(value) => encode(b"\x1b[38;5;", &value),
        None => b"\x1b[39m".to_vec(),
    }
}

fn encode_background(color_code: Option<u8>) -> Vec<u8> {
    match color_code {
        Some(value) => encode(b"\x1b[48;5;", &value),
        None => b"\x1b[49m".to_vec(),
    }
}

fn unicode_converter_row<'a>(
    img: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    y: u32,
    result: &'a mut Vec<u8>,
) -> &'a mut Vec<u8> {
    let mut last_bg: Option<u8> = None;
    let mut last_fg: Option<u8> = None;

    for x in 0..img.dimensions().0 {
        let mut color_code_bg: Option<u8>;
        let mut color_code_fg: Option<u8>;

        if y == img.dimensions().1 - 1 {
            color_code_bg = None;
            color_code_fg = if img[(x, y)][3] < 128 {
                None
            } else {
                Some(find_color_code(img[(x, y)]))
            };
        } else {
            color_code_bg = if img[(x, y)][3] < 128 {
                None
            } else {
                Some(find_color_code(img[(x, y)]))
            };
            color_code_fg = if img[(x, y + 1)][3] < 128 {
                None
            } else {
                Some(find_color_code(img[(x, y + 1)]))
            };
        }

        let mut upper_block_flag = false;

        if color_code_fg == None && color_code_bg != None {
            color_code_fg = color_code_bg;
            color_code_bg = None;
            upper_block_flag = true;
        } else if color_code_fg == last_bg && color_code_bg == last_fg {
            let mut transition = color_code_bg;
            color_code_bg = color_code_fg;
            color_code_fg = transition;
            upper_block_flag = true;
        }

        if color_code_fg != last_fg {
            result.extend_from_slice(&encode_foreground(color_code_fg));
            last_fg = color_code_fg;
        }
        if color_code_bg != last_bg {
            result.extend_from_slice(&encode_background(color_code_bg));
            last_bg = color_code_bg;
        }

        if color_code_bg == color_code_fg {
            result.extend_from_slice(b" ");
        } else if upper_block_flag {
            result.extend_from_slice(String::from("\u{2580}").as_bytes());
        } else if y == img.dimensions().1 - 1 {
            result.extend_from_slice(String::from("\u{2580}").as_bytes());
        } else {
            result.extend_from_slice(String::from("\u{2584}").as_bytes())
        }
    }
    result.extend_from_slice(b"\x1b[0m\n");
    result
}
pub fn convert_to_unicode_from_image_buffer(img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();

    for y in 0..img.dimensions().1 {
        if y % 2 == 0 {
            result = unicode_converter_row(&img, y, &mut result).to_vec();
        }
    }

    result
}

pub fn convert_to_unicode_from_file(filename: &str) -> Result<Vec<u8>, ImageError> {
    let img = image::open(filename);

    match img {
        Ok(value) => Ok(convert_to_unicode_from_image_buffer(&value.to_rgba())),
        Err(err) => Err(err),
    }
}
