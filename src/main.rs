use crate::{color_hex::_ColorElement, colors::Color};
use std::{env::args, io::stdin};

mod color_hex;
mod colors;
mod traits;

struct _ArgsFinal {
    arg: Vec<String>,
}

impl _ArgsFinal {
    fn get_final_args(&self) -> Vec<String> {
        let args = self.arg.to_vec();
        args
    }
}

pub fn main() {
    let args_v: Vec<String> = args().collect();

    let arg = _ArgsFinal { arg: args_v };

    let first = &arg.arg[1];

    if first == "-C" {
        for _ in 0..arg.get_final_args()[2].len() {
            let col_r = rand::random_range(1..254 + 1);
            let col_g = rand::random_range(1..254 + 1);
            let col_b = rand::random_range(1..254 + 1);

            let col = colors::Color {
                0: col_r,
                1: col_g,
                2: col_b,
            };

            let color = Color(col.get_first(), col.get_second(), col.get_third());

            let hex_r = color_hex::_HexBased {
                r: color.0,
                g: color.1,
                b: color.2,
            };

            let color_r = colors::Color::get_first(&color);
            let color_g = colors::Color::get_second(&color);
            let color_b = colors::Color::get_third(&color);

            println!("R: {}, G: {}, B: {}", color_r, color_g, color_b);
            println!(
                "#{}{}{}",
                hex_r.get_hex_r().to_string(),
                hex_r.get_hex_g().to_string(),
                hex_r.get_hex_b().to_string()
            )
        }
    } else {
        for _ in 0..2 {
            let col_r = rand::random_range(1..254 + 1);
            let col_g = rand::random_range(1..254 + 1);
            let col_b = rand::random_range(1..254 + 1);

            let col = colors::Color {
                0: col_r,
                1: col_g,
                2: col_b,
            };

            let color = Color(col.get_first(), col.get_second(), col.get_third());

            let hex_r = color_hex::_HexBased {
                r: color.0,
                g: color.1,
                b: color.2,
            };

            let color_r = colors::Color::get_first(&color);
            let color_g = colors::Color::get_second(&color);
            let color_b = colors::Color::get_third(&color);

            println!("R: {}, G: {}, B: {}", color_r, color_g, color_b);
            println!(
                "#{}{}{}",
                hex_r.get_hex_r().to_string(),
                hex_r.get_hex_g().to_string(),
                hex_r.get_hex_b().to_string()
            )
        }
    }
    stdin().read_line(&mut String::new()).unwrap();
}
