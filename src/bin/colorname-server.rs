#![feature(proc_macro_hygiene, decl_macro)]

use colorname::get_colors_matching;
use rocket::{self, get, routes};
use rocket_contrib::templates::Template;
use serde::Serialize;

#[derive(Serialize)]
struct Colors<'a> {
    colors: Vec<Color<'a>>,
}

#[derive(Serialize)]
struct Color<'a> {
    name: &'a str,
    hex: &'a str,
    fg: &'a str,
}

#[get("/colorname/<color>")]
fn get_colors(color: String) -> Template {
    let colors = get_colors_matching(&color)
        .iter()
        .map(|(name, hex)| Color {
            name,
            hex,
            fg: if is_dark(hex) { "lightgray" } else { "black" },
        })
        .collect::<Vec<_>>();

    Template::render("colorname", Colors { colors })
}

fn is_dark(hex: &str) -> bool {
    let (r, g, b) = parse_hex(hex);
    let r = r as f64 / 255.0;
    let g = g as f64 / 255.0;
    let b = b as f64 / 255.0;
    let brightness = (r * r * 0.241 + g * g * 0.691 + b * b * 0.068).sqrt();
    brightness < 0.33
}

fn parse_hex(hex: &str) -> (u8, u8, u8) {
    let r = u8::from_str_radix(&hex[1..=2], 16).unwrap();
    let g = u8::from_str_radix(&hex[3..=4], 16).unwrap();
    let b = u8::from_str_radix(&hex[5..=6], 16).unwrap();
    (r, g, b)
}
fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![get_colors])
        .launch();
}
