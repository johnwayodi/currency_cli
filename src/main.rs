extern crate reqwest;
extern crate serde;
extern crate serde_json;

use serde_derive::Deserialize;
use std::io;

#[derive(Deserialize)]
struct ApiResponse {
    rates: std::collections::HashMap<String, f64>,
}

const BASE_URL: &str = "https://open.er-api.com/v6/latest/USD";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter country codes separated by commas (e.g., 'KES,GBP,JPY,UGX'):");

    let mut country_codes = String::new();
    io::stdin().read_line(&mut country_codes)?;
    let country_codes: Vec<&str> = country_codes.trim().split(',').map(|s| s.trim()).collect();

    let response: ApiResponse = match reqwest::blocking::get(BASE_URL) {
        Ok(res) => res.json()?,
        Err(err) => {
            println!("Error fetching exchange rates: {}", err);
            return Ok(());
        }
    };

    for country_code in country_codes.iter() {
        let uppercase_code = country_code.to_uppercase();
        match response.rates.get(&uppercase_code) {
            Some(rate) => {
                println!("1 USD = {} {}", rate, uppercase_code);
            }
            None => {
                println!(
                    "Error: Couldn't find the exchange rate for {}",
                    uppercase_code
                );
            }
        }
    }

    Ok(())
}
