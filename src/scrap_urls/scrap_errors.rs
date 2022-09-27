use scraper::Selector;
use snafu::prelude::*;


#[derive(Debug, Snafu)]
pub enum Errors {
    #[snafu(display("Element with this selector not found"))]
    NotFoundElement,

    #[snafu(display("Not correct selector"))]
    NotCorrectSelector,
}
