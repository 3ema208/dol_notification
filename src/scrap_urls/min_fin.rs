use super::scrap_errors::Errors as ScrapErr;
use super::scrapper_trait::Scrapper;
use mockall::predicate::*;
use mockall::*;
use reqwest::{Request, Response};
use scraper::{Html, Selector};
use std::error::Error;
use std::fmt::{format, Display};
use std::io::Read;

const URL: &str = "https://minfin.com.ua/currency/usd/";
const SELECTOR_BLACK_MARKET_SELL: &str = "tbody>tr:nth-of-type(2)>td:nth-of-type(3)>span";
const SELECTOR_BLACK_MARKET_BUY: &str = "tbody>tr:nth-of-type(2)>td:nth-of-type(2)>span";

// Parse Minfin website with rate dollar
#[derive(Debug)]
struct MinFinScrapper {
    url: &'static str,
    buy_rate_dollar_bm: Selector,
    sell_rate_dollar_bm: Selector,
}

#[automock]
trait GetDocument {
    fn get_document() -> Result<Html, Box<dyn Error>>;
}

impl MinFinScrapper {
    fn new() -> Self {
        let buy = Selector::parse(SELECTOR_BLACK_MARKET_BUY).unwrap();
        let sell = Selector::parse(SELECTOR_BLACK_MARKET_SELL).unwrap();
        MinFinScrapper {
            url: URL,
            buy_rate_dollar_bm: buy,
            sell_rate_dollar_bm: sell,
        }
    }
}

impl GetDocument for MinFinScrapper {
    fn get_document() -> Result<Html, Box<dyn Error>> {
        let response = reqwest::blocking::get(URL)?;
        let data = response.text()?;
        Ok(Html::parse_document(data.as_str()))
    }
}

impl Scrapper for MinFinScrapper {
    fn parse(&self) -> Result<String, Box<dyn Error>> {
        let document = MinFinScrapper::get_document()?;
        let val_buy = {
            document
                .select(&self.buy_rate_dollar_bm)
                .next()
                .ok_or(ScrapErr::NotFoundElement)?
                .text()
                .next()
                .ok_or(ScrapErr::NotCorrectSelector)?
                .trim()
        };
        let val_sell = {
            document
                .select(&self.sell_rate_dollar_bm)
                .next()
                .ok_or(ScrapErr::NotFoundElement)?
                .text()
                .next()
                .ok_or(ScrapErr::NotCorrectSelector)?
                .trim()
        };
        Ok(format!("Black market {} {}", val_buy, val_sell))
    }
}

#[cfg(test)]
mod test {
    use super::{Error, Html, MinFinScrapper, MockGetDocument, ScrapErr, Scrapper};

    #[test]
    fn it_work() {
        let min = MinFinScrapper::new();
        let mock = MockGetDocument::get_document_context();
        mock.expect().returning(|| Ok(Html::parse_fragment("body")));
        let r = min.parse();
        println!("{:?}", r)
    }
}
