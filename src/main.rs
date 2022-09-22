mod telegram;
mod scrap_urls;

use reqwest;
use scraper::{Html, Selector};


fn main() {
    let url: &str = "https://minfin.com.ua/currency/usd/";
    let bot_token = "5671049707:AAEBwSEQJMF1_Hc5F-5X5NZEjQif2-4k25U";
    let chat_id = "292317891";
    let telegram_url_send_message = "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}";


    let component_rate_selector = Selector::parse("tbody>tr:nth-of-type(2)").unwrap();
    let black_market_sell = Selector::parse("td:nth-of-type(3)>span").unwrap();
    let black_market_buy = Selector::parse("td:nth-of-type(2)>span").unwrap();

    let response = reqwest::blocking::get(url);
    match response {
        Ok(response) => {
            let text_body = response.text().unwrap_or("".to_string());
            let document = Html::parse_document(text_body.as_str());
            let element = document
                .select(&component_rate_selector)
                .next();
            match element {
                Some(el) => {
                    let mut sell_e = el.select(&black_market_sell);
                    let sell_rate = match sell_e.next() {
                        Some(val) => {
                            match val.inner_html().split('\n').next() {
                                Some(el) => el.to_string(),
                                None => " ".to_string()
                            }
                        }
                        None => "none".to_string()
                    };

                    let buy_rate: String = match el.select(&black_market_buy).next(){
                        Some(val) => {
                            val.inner_html().split('\n').next().unwrap_or("none").to_string()
                        }
                        None => "none".to_string()
                    };

                    let message = format!("Sell {} Buy {}", sell_rate, buy_rate);
                    let response_send_message = reqwest::blocking::get(
                        format!("https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}", bot_token, chat_id, message)
                    );
                },
                None => {
                    println!("element not found")
                }
            }
        },
        Err(e) => println!("{:?}", e)
    }
}
