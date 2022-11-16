// Credits: Tim McNamara

use std::{collections::BinaryHeap, ops::Add};
use url::Url;

// Composition over Inheritance
#[derive(PartialEq, Eq)]
struct ShortestFirst<U>(U);

impl ShortestFirst<Url> {
    fn new(url: Url) -> Self {
        Self(url)
    }
}

impl PartialOrd for ShortestFirst<Url> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ShortestFirst<Url> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let left = (other.0).as_str().len();
        let right = (self.0).as_str().len();

        left.cmp(&right)
    }
}

impl From<Url> for ShortestFirst<Url> {
    fn from(url: Url) -> Self {
        Self(url)
    }
}

impl std::fmt::Display for ShortestFirst<Url> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

struct Miles(f64);
struct Kilometers(f64);

impl Add<Miles> for Kilometers {
    type Output = Kilometers;

    fn add(self, rhs: Miles) -> Self::Output {
        Kilometers(self.0 + rhs.0 * 1.609)
    }
}

impl From<Kilometers> for Miles {
    fn from(km: Kilometers) -> Self {
        Self(km.0 / 1.609)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url_strings = vec![
        "https://derstandard.at",
        "https://rust-training.eu",
        "https://fettblog.eu",
        "https://mastodon.social/@deadparrot",
        "https://rust-linz.at",
    ];

    let mut urls = BinaryHeap::new();

    for url in url_strings {
        urls.push(ShortestFirst::from(Url::parse(url)?));
    }

    while let Some(url) = urls.pop() {
        println!("{}", url)
    }

    Ok(())
}
