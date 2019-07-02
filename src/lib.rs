use std::cmp::Ordering;
use strsim::{jaro, levenshtein, jaro_winkler};

pub fn get_colors_matching(query: &str) -> Vec<(&'static str, &'static str)> {
    let xkcd_colors: &str = include_str!("./rgb.txt");
    let mut colors: Vec<(&str, &str)> = xkcd_colors
        .lines()
        .skip(1)
        .map(|line| line.split('\t'))
        .map(|mut line| (line.next().unwrap(), line.next().unwrap()))
        .filter(|(name, _)| similarity_score(query, name, jaro_winkler) > 0.9 || jaro(query, name) > 0.9)
        .collect();
    colors
        .sort_by_cached_key(|(name, _)| (levenshtein(query, name)) as u32);
    colors
}

fn similarity_score<T>(query: &str, colorname: &str, cmp_fn: T) -> f64
where
    T: Fn(&str, &str) -> f64,
{
    query
        .split(" ")
        .map(|querypart| {
            colorname
                .split(" ")
                .map(|namepart| cmp_fn(querypart, namepart))
                .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                .unwrap_or(0.0)
        })
        .product::<f64>()
}
