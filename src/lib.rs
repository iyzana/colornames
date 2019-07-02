use std::cmp::Ordering;
use strsim::{jaro, jaro_winkler, levenshtein};

pub fn get_colors_matching(query: &str) -> Vec<(&'static str, &'static str)> {
    let xkcd_colors: &str = include_str!("./rgb.txt");
    let mut colors: Vec<(&str, &str)> = xkcd_colors
        .lines()
        .skip(1)
        .map(|line| line.split('\t'))
        .map(|mut line| (line.next().unwrap(), line.next().unwrap()))
        .filter(|(name, _)| similarity_score(query, name) > 0.9 || jaro(query, name) > 0.9)
        .collect();
    colors.sort_by_cached_key(|(name, _)| (levenshtein(query, name)) as u32);
    colors
}

fn similarity_score(query: &str, colorname: &str) -> f64 {
    query
        .split(" ")
        .map(|querypart| {
            colorname
                .split(" ")
                .map(|namepart| jaro_winkler(querypart, namepart))
                .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                .unwrap_or(0.0)
        })
        .product::<f64>()
}
