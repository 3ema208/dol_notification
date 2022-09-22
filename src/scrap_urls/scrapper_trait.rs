use std::error::Error;
use crate::scrap_urls::result_message::ResultMessage;


pub trait Scrapper {
    fn parse(&self) -> Result<ResultMessage, Box<dyn Error>>;
}