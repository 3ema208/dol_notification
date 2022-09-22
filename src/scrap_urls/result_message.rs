
trait GetBlackRate {
    fn get_black_rate(&self) -> String;
}

trait GetBankRate {
    fn get_bank_rate(&self) -> String;
}


#[derive(Default, Debug)]
pub struct ResultMessage {
    title: String,
    sell_rate: f32,
    buy_rate: f32,
}
