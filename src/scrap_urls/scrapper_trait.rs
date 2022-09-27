use std::error::Error;


pub trait Scrapper {
    fn parse(&self) -> Result<String, Box<dyn Error>>;
}
