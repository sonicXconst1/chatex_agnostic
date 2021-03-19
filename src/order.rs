use crate::converter;
use chatex_sdk_rust::coin::CoinPair;
use agnostic::trading_pair::{TradingPairConverter, TradingPair};
use std::str::FromStr;

pub struct Order {
    pub id: Option<u32>,
    pub pair: CoinPair,
    pub rate: f64,
    pub amount: f64,
}

impl Order {
    pub fn from_trade(
        trading_pair: &TradingPair,
        trade: chatex_sdk_rust::models::Trade,
    ) -> Order {
        Self::from(trade.id, &trading_pair, &trade.order.rate, &trade.amount)
    }

    pub fn from_raw(
        trading_pair: &agnostic::trading_pair::TradingPair,
        order: &chatex_sdk_rust::models::Order
    ) -> Order {
        Self::from(order.id, &trading_pair, &order.rate, &order.amount)
    }

    fn from(
        id: u32,
        trading_pair: &TradingPair,
        price: &str,
        amount: &str,
    ) -> Order {
        let price = f64::from_str(&price).unwrap();
        let amount = f64::from_str(&amount).unwrap();
        let converter = converter::TradingPairConverter::default();
        let pair = converter.to_pair(trading_pair.clone());
        let order_price = price.into();
        let price = agnostic::price::convert_to_base_coin_price(
            trading_pair.target.clone(),
            trading_pair.side.clone(),
            &order_price);
        let amount = agnostic::price::convert_to_base_coin_amount(
            trading_pair.target.clone(),
            trading_pair.side.clone(),
            &order_price,
            amount);
        Order {
            id: Some(id),
            pair,
            rate: price,
            amount
        }
    }
}

impl From<agnostic::order::Order> for Order {
    fn from(order: agnostic::order::Order) -> Order {
        let converter = converter::TradingPairConverter::default();
        let pair = converter.to_pair(order.trading_pair.clone());
        let order_price = order.price.into();
        let price = agnostic::price::convert_to_base_coin_price(
            order.trading_pair.target.clone(),
            order.trading_pair.side.clone(),
            &order_price);
        let amount = agnostic::price::convert_to_base_coin_amount(
            order.trading_pair.target.clone(),
            order.trading_pair.side.clone(),
            &order_price,
            order.amount);
        Order {
            id: None,
            pair,
            rate: price,
            amount
        }
    }
}
