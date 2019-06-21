use clap::{App, Arg};
use crossterm_style::{Color, Colored};
use strsim::jaro;

fn main() {
    let matches = App::new("colorname")
        .about("show colors by name")
        .arg(Arg::with_name("COLORNAME").required(true).multiple(true))
        .get_matches();
    let color: String = matches
        .values_of("COLORNAME")
        .unwrap()
        .collect::<Vec<&str>>()
        .join(" ");

    let colors = get_colors_matching(&color);
    print_colors(colors);
}

fn parse_hex(hex: &str) -> Color {
    let r = u8::from_str_radix(&hex[1..=2], 16).unwrap();
    let g = u8::from_str_radix(&hex[3..=4], 16).unwrap();
    let b = u8::from_str_radix(&hex[5..=6], 16).unwrap();
    Color::Rgb { r, g, b }
}

fn get_colors_matching(colorname: &str) -> Vec<(&'static str, &'static str)> {
    let xkcd_colors: &str = include_str!("./rgb.txt");
    let mut colors: Vec<(&str, &str)> = xkcd_colors
        .lines()
        .skip(1)
        .map(|line| line.split('\t'))
        .map(|mut line| (line.next().unwrap(), line.next().unwrap()))
        .filter(|(name, _)| jaro(name, &colorname) > 0.75)
        .collect();

    colors.sort_by_cached_key(|(name, _)| (jaro(name, &colorname) * 1_000_000.0) as u32);
    colors.reverse();
    colors.truncate(30);
    colors
}

fn print_colors(colors: Vec<(&str, &str)>) {
    let max_length = colors
        .iter()
        .map(|(name, _)| name)
        .max_by_key(|name| name.len())
        .map_or(0, |name| name.len());
    for (colorname, hex) in colors {
        println!(
            "{}{:width$} ({}) is {}this color \u{25a0}",
            Colored::Fg(Color::Reset),
            colorname,
            hex,
            Colored::Fg(parse_hex(hex)),
            width = max_length
        );
    }
}
