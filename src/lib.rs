pub mod accountant;
pub mod sniffer;
pub mod trader;
pub mod merchant;

use chatex_sdk_rust::coin;
use agnostic::trading_pair;
use agnostic::trading_pair::Side;
use agnostic::trading_pair::Coins;
use agnostic::trading_pair::Coin;
use agnostic::trading_pair::TradingPair;

#[derive(Default, Clone, Copy, Debug)]
pub struct TradingPairConverter {
}

impl trading_pair::TradingPairConverter for TradingPairConverter {
    type Pair = coin::CoinPair;
    type Coin = coin::Coin;

    fn to_string(&self, trading_pair: TradingPair ) -> String {
        self.to_pair(trading_pair).into()
    }

    fn to_pair(&self, trading_pair: TradingPair) -> Self::Pair {
        let direct_pair = match trading_pair.coins {
            Coins::TonUsdt => coin::CoinPair::new(coin::Coin::TON, coin::Coin::USDT),
        };
        match trading_pair.side {
            Side::Buy => direct_pair.reversed(),
            Side::Sell => direct_pair,
        }
    }

    fn from_agnostic_coin(&self, coin: Coin) -> Self::Coin {
        match coin {
            Coin::TON => Self::Coin::TON,
            Coin::USDT => Self::Coin::USDT,
        }
    }

    fn to_agnostic_coin(&self, coin: Self::Coin) -> Option<Coin> {
        match coin {
            Self::Coin::USDT => Some(Coin::USDT),
            Self::Coin::TON => Some(Coin::TON),
            _other => None,
        }
    }
}
