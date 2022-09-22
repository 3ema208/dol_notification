use std::error::Error;

use reqwest;
use scraper::{Selector, Html, ElementRef};

use super::result_message::{ResultMessage};
use super::scrap_errors::Errors as ScrapErr;
use super::scrapper_trait::{Scrapper};


const URL: &str = "https://minfin.com.ua/currency/usd/";
const SELECTOR_BLOCK: &str = "tbody>tr:nth-of-type(2)";
const SELECTOR_BLACK_MARKET_SELL: &str = "td:nth-of-type(3)>span";
const SELECTOR_BLACK_MARKET_BUY: &str = "td:nth-of-type(2)>span";


// Parse Minfin website with rate dollar
#[derive(Debug)]
struct MinFinScrapper {
    block: Selector,
    buy_rate: Selector,
    sell_rate: Selector
}


impl MinFinScrapper {
    fn new() -> Self {
        MinFinScrapper{
            block: Selector::parse(SELECTOR_BLOCK).unwrap(),
            buy_rate: Selector::parse(SELECTOR_BLACK_MARKET_BUY).unwrap(),
            sell_rate: Selector::parse(SELECTOR_BLACK_MARKET_SELL).unwrap()
        }
    }
}

impl Scrapper for MinFinScrapper {
    fn parse(&self) -> Result<ResultMessage, Box<dyn Error>> {
        let response = reqwest::blocking::get(URL)?;
        let source_text = response.text()?;
        let document = Html::parse_document(source_text.as_str());
        match document.select(&self.block).next() {
            Some(el) => {
                let buy_rate = match el.select(&self.buy_rate).next() {
                    Some(el) => {
                        let source_value = el.inner_html();
                        source_value.split('\n').next().un
                    },
                    None => return Err(
                        Box::new(ScrapErr::NotFoundElement { selector: self.buy_rate.clone() } )
                    )
                };
                let sell_rate =match el.select(&self.sell_rate).next() {
                    Some(el) => {

                    },
                    None => return Err(
                        Box::new(ScrapErr::NotFoundElement { selector: self.buy_rate.clone() } )
                    )
                };
            },
            None => return Err(
                Box::new(
                    ScrapErr::NotFoundElement{
                        selector: self.block.clone()
                    }
                )
            )
        };
        Ok(ResultMessage::default())
    }
}
