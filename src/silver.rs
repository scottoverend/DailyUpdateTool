extern crate reqwest;
extern crate scraper;

use scraper::{Html, Selector};

pub fn silver_function() {
    let url = "https://finance.yahoo.com/";
    let response = reqwest::blocking::get(url)
        .expect("Failed to fetch URL")
        .text()
        .expect("Failed to get response text");
    let document = Html::parse_document(&response);
    let selector = Selector::parse("li[aria-label='S&P Futures']").expect("Failed to parse selector");

    if let Some(element) = document.select(&selector).next() {
        let data = element.text().collect::<String>();
        println!("S&P Futures: {}", data);
    } else {
        println!("Silver Element not found");
    }
}