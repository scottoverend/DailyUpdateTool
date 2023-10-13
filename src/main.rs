extern crate reqwest;
extern crate scraper;

use scraper::{Html, Selector};
mod silver;

pub fn main() {
    gold();
    println!("");
    silver::silver_function();
    println!("");
}

fn gold() {
    let url = "https://finance.yahoo.com/";
    let response = reqwest::blocking::get(url)
        .expect("Failed to fetch URL")
        .text()
        .expect("Failed to get response text");
    let document = Html::parse_document(&response);
    let selector = Selector::parse("li[aria-label='Gold']").expect("Failed to parse selector");

    if let Some(element) = document.select(&selector).next() {
        let data = element.text().collect::<String>();
        println!("Gold Data: {}", data);
    } else {
        println!("Gold Element not found");
    }
}