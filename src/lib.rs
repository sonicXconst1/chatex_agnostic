pub mod accountant;
pub mod sniffer;
pub mod trader;
pub mod merchant;

pub struct CoinConverter {
}

impl agnostic::coin::CoinConverter for CoinConverter {
    fn to_string(&self, coin: agnostic::coin::Coin) -> String {
        use agnostic::coin::Coin;
        match coin {
            Coin::TON => chatex_sdk_rust::coin::Coin::TON.to_string(),
            Coin::USDT => chatex_sdk_rust::coin::Coin::USDT.to_string(),
            Coin::BTC => chatex_sdk_rust::coin::Coin::BTC.to_string(),
        }
    }
}

