use crate::{convert_price, convert_amount};
use agnostic::trading_pair::TradingPair;
use agnostic::trading_pair::TradingPairConverter;
use std::str::FromStr;

pub struct ChatexSniffer<TConnector> {
    client: std::sync::Arc<chatex_sdk_rust::ChatexClient<TConnector>>,
}

impl<TConnector> ChatexSniffer<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    pub fn new(client: std::sync::Arc<chatex_sdk_rust::ChatexClient<TConnector>>) -> Self {
        ChatexSniffer { client }
    }
}

impl<TConnector> agnostic::market::Sniffer for ChatexSniffer<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Clone + Sync + 'static,
{
    fn all_the_best_orders(
        &self,
        trading_pair: TradingPair,
        count: u32,
    ) -> agnostic::market::Future<Result<Vec<agnostic::order::Order>, String>> {
        let exchange = self.client.exchange();
        let future = async move {
            let converter = crate::TradingPairConverter::default();
            let pair = converter.to_pair(trading_pair.clone());
            match exchange.get_all_orders(pair, None, Some(count)).await {
                Ok(orders) => Ok(orders
                    .into_iter()
                    .map(|order| {
                        let price = f64::from_str(&order.rate).unwrap();
                        log::debug!("Initial price: {}", price);
                        let amount = f64::from_str(&order.amount).unwrap();
                        let price = convert_price(trading_pair.side.clone(), price);
                        log::debug!("Converted price: {}", price);
                        let amount = convert_amount(trading_pair.side.clone(), price, amount);
                        agnostic::order::Order {
                            trading_pair: trading_pair.clone(),
                            price,
                            amount,
                        }
                    })
                    .collect()),
                Err(error) => Err(format!("{}", error)),
            }
        };
        Box::pin(future)
    }

    fn the_best_order(
        &self,
        trading_pair: TradingPair,
    ) -> agnostic::market::Future<Result<agnostic::order::Order, String>> {
        let exchange = self.client.exchange();
        let future = async move {
            let converter = crate::TradingPairConverter::default();
            let pair = converter.to_pair(trading_pair.clone());
            match exchange.get_all_orders(pair, None, Some(1)).await {
                Ok(orders) => {
                    let order = match orders.get(0) {
                        Some(order) => order,
                        None => return Err("0 orders from chatex API".to_owned()),
                    };
                    let price = f64::from_str(&order.rate).unwrap();
                    let amount = f64::from_str(&order.amount).unwrap();
                    let price = convert_price(trading_pair.side.clone(), price);
                    let amount = convert_amount(trading_pair.side.clone(), price, amount);
                    Ok(agnostic::order::Order {
                        trading_pair,
                        price,
                        amount,
                    })
                }
                Err(error) => Err(format!("{}", error)),
            }
        };
        Box::pin(future)
    }

    fn get_my_orders(
        &self,
        trading_pair: TradingPair,
    ) -> agnostic::market::Future<Result<Vec<agnostic::order::OrderWithId>, String>> {
        let exchange = self.client.exchange();
        let future = async move {
            let converter = crate::TradingPairConverter::default();
            let pair = converter.to_pair(trading_pair.clone());
            match exchange.get_my_orders(Some(pair), None, None, None).await {
                Ok(orders) => Ok(orders
                    .into_iter()
                    .map(|order| {
                        let price = f64::from_str(&order.rate).unwrap();
                        let amount = f64::from_str(&order.amount).unwrap();
                        let price = convert_price(trading_pair.side.clone(), price);
                        let amount = convert_amount(trading_pair.side.clone(), price, amount);
                        agnostic::order::OrderWithId {
                            id: format!("{}", order.id),
                            trading_pair: trading_pair.clone(),
                            amount,
                            price,
                        }
                    })
                    .collect()),
                Err(error) => Err(error.to_string()),
            }
        };
        Box::pin(future)
    }
}
