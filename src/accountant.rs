pub struct ChatexAccountant {}

impl agnostic::market::Accountant for ChatexAccountant {
    fn ask(
        &self,
        coin: agnostic::coin::Coin,
    ) -> agnostic::market::Future<Result<agnostic::currency::Currency, String>> {
        todo!()
    }

    fn ask_both(
        &self,
        coins: agnostic::coin::CoinPair,
    ) -> agnostic::market::Future<
        Result<(agnostic::currency::Currency, agnostic::currency::Currency), String>,
    > {
        todo!()
    }

    fn calculate_volume(&self, coins: agnostic::coin::CoinPair, price: f64, volume: f64) -> f64 {
        todo!()
    }
}
