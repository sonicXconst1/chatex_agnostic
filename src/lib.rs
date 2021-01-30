pub mod accountant;
pub mod sniffer;
pub mod trader;
pub mod merchant;

use agnostic::coin::Coin;

#[derive(Clone, Copy, Debug)]
pub struct CoinConverter {
}

impl agnostic::coin::CoinConverter for CoinConverter {
    type Coin = chatex_sdk_rust::coin::Coin;

    fn to_string(&self, coin: agnostic::coin::Coin) -> String {
        match coin {
            Coin::TON => chatex_sdk_rust::coin::Coin::TON.to_string(),
            Coin::USDT => chatex_sdk_rust::coin::Coin::USDT.to_string(),
            Coin::BTC => chatex_sdk_rust::coin::Coin::BTC.to_string(),
        }
    }

    fn to_coin(&self, coin: agnostic::coin::Coin) -> Self::Coin {
        match coin {
            Coin::TON => chatex_sdk_rust::coin::Coin::TON,
            Coin::USDT => chatex_sdk_rust::coin::Coin::USDT,
            Coin::BTC => chatex_sdk_rust::coin::Coin::BTC,
        }
    }
}

