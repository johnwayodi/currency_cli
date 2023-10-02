extern crate clap;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use clap::{App, Arg};
use serde_derive::Deserialize;
use std::io;

#[derive(Deserialize)]
struct ApiResponse {
    rates: std::collections::HashMap<String, f64>,
}

const BASE_URL: &str = "https://open.er-api.com/v6/latest";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("currency-cli")
        .arg(
            Arg::with_name("convert")
                .help("Convert between currencies in the format FROM-TO, e.g., KES-UGX")
                .short("c")
                .long("convert")
                .value_name("FROM-TO")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("AMOUNT")
                .help("The amount to convert")
                .required_if("convert", "")
                .index(1),
        )
        .get_matches();

    if let Some(convert) = matches.value_of("convert") {
        let currencies: Vec<&str> = convert.split('-').collect();
        if currencies.len() == 2 {
            let from = &currencies[0].to_uppercase();
            let to = &currencies[1].to_uppercase();
            let amount: f64 = matches
                .value_of("AMOUNT")
                .unwrap_or("1")
                .parse()
                .unwrap_or(1.0);
            convert_currencies(from, to, amount)?;
        } else {
            println!("Invalid format for conversion. Use: FROM-TO");
        }
    } else {
        // Default behavior
        println!("Enter country codes separated by commas (e.g., 'KES,UGX' for Kenyan Shilling and Ugandan Shilling):");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let country_codes: Vec<String> = input
            .trim()
            .split(',')
            .map(|s| s.trim().to_uppercase())
            .collect();

        let response: ApiResponse = reqwest::blocking::get(BASE_URL)?.json()?;

        for country_code in country_codes {
            match response.rates.get(&country_code) {
                Some(rate) => {
                    println!("1 USD = {} {}", rate, country_code);
                }
                None => {
                    println!("Couldn't find exchange rate for {}", country_code);
                }
            }
        }
    }

    Ok(())
}

fn convert_currencies(from: &str, to: &str, amount: f64) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/{}", BASE_URL, from);
    let response: ApiResponse = reqwest::blocking::get(&url)?.json()?;

    if let Some(rate) = response.rates.get(to) {
        let converted = amount * rate;
        println!("{} {} = {} {}", amount, from, converted, to);
    } else {
        println!("Error: Couldn't find exchange rate for {}", to);
    }

    Ok(())
}
