pub struct ChatexSniffer {
}

impl agnostic::market::Sniffer for ChatexSniffer {
    fn all_the_best_orders(&self, coins: agnostic::coin::CoinPair, count: u32) -> Vec<agnostic::order::Order> {
        todo!()
    }

    fn the_best_order(&self, coins: agnostic::coin::CoinPair) -> Option<agnostic::order::Order> {
        todo!()
    }
}
