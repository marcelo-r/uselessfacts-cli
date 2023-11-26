use std::env;

use serde::{Deserialize, Serialize};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fact_type = env::args().nth(1).unwrap_or(String::from(""));
    let result = match fact_type.as_str() {
        "t" | "today" => get_daily_fact(),
        "r" | "random" => get_random_fact(),
        _ => get_help(),
    };
    match result {
        Ok(fact) => print_fact(fact),
        Err(e) => return Err(e),
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Fact {
    id: String,
    text: String,
    source: String,
    source_url: String,
    language: String,
    permalink: String,
}

const FACT_BASE_URL: &str = "https://uselessfacts.jsph.pl";
const FACT_RANDOM: &str = "/api/v2/facts/random?language=en";
const FACT_DAILY: &str = "/api/v2/facts/today?language=en";

fn get_fact(url: &str) -> Result<Fact, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(url)?.json::<Fact>()?;
    Ok(resp)
}

fn get_random_fact() -> Result<Fact, Box<dyn std::error::Error>> {
    let url = format!("{}{}", FACT_BASE_URL, FACT_RANDOM);
    get_fact(&url)
}

fn get_daily_fact() -> Result<Fact, Box<dyn std::error::Error>> {
    let url = format!("{}{}", FACT_BASE_URL, FACT_DAILY);
    get_fact(&url)
}

fn get_help() -> Result<Fact, Box<dyn std::error::Error>> {
    let mut fact = Fact::default();
    fact.text = String::from("Usage: fact [t]oday | [r]andom");
    Ok(fact)
}

fn print_fact(fact: Fact) {
    println!("Fact: {}", fact.text);
    println!("Source: {}", fact.source_url);
}
