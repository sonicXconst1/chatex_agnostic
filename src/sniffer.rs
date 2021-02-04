use std::str::FromStr;
use agnostic::coin::CoinConverter;

pub struct ChatexSniffer<TConnector> {
    client: std::sync::Arc<chatex_sdk_rust::ChatexClient<TConnector>>,
}

impl<TConnector> ChatexSniffer<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    pub fn new(client: std::sync::Arc<chatex_sdk_rust::ChatexClient<TConnector>>) -> Self {
        ChatexSniffer {
            client,
        }
    }
}

impl<TConnector> agnostic::market::Sniffer for ChatexSniffer<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Clone + Sync + 'static,
{
    fn all_the_best_orders(
        &self,
        coins: agnostic::coin::CoinPair,
        count: u32,
    ) -> agnostic::market::Future<Result<Vec<agnostic::order::Order>, String>> {
        let exchange = self.client.exchange();
        let future = async move {
            let converter = crate::CoinConverter { };
            let pair = coins.clone();
            let pair = chatex_sdk_rust::coin::CoinPair::new(
                converter.to_coin(pair.sell),
                converter.to_coin(pair.buy),
            );
            match exchange.get_all_orders(pair, None, Some(count)).await {
                Ok(orders) => Ok(orders
                    .into_iter()
                    .map(|order| agnostic::order::Order {
                        coins: coins.clone(),
                        price: f64::from_str(&order.rate).unwrap(),
                        amount: f64::from_str(&order.amount).unwrap(),
                    })
                    .collect()),
                Err(error) => Err(format!("{}", error)),
            }
        };
        Box::pin(future)
    }

    fn the_best_order(
        &self,
        coins: agnostic::coin::CoinPair,
    ) -> agnostic::market::Future<Result<agnostic::order::Order, String>> {
        let exchange = self.client.exchange();
        let future = async move {
            let converter = crate::CoinConverter::default();
            let pair = coins.clone();
            let pair = chatex_sdk_rust::coin::CoinPair::new(
                converter.to_coin(pair.sell),
                converter.to_coin(pair.buy),
            );
            match exchange.get_all_orders(pair, None, Some(1)).await {
                Ok(orders) => {
                    let order = match orders.get(0) {
                        Some(order) => order,
                        None => return Err("0 orders from chatex API".to_owned()),
                    };
                    Ok(agnostic::order::Order {
                        coins: coins.clone(),
                        price: f64::from_str(&order.rate).unwrap(),
                        amount: f64::from_str(&order.amount).unwrap(),
                    })
                }
                Err(error) => Err(format!("{}", error)),
            }
        };
        Box::pin(future)
    }

    fn get_my_orders(
        &self,
        coins: agnostic::coin::CoinPair,
    ) -> agnostic::market::Future<Result<Vec<agnostic::order::OrderWithId>, String>> {
        let exchange = self.client.exchange();
        let future = async move {
            let converter = crate::CoinConverter::default();
            let pair = coins.clone();
            let pair = chatex_sdk_rust::coin::CoinPair::new(
                converter.to_coin(pair.sell),
                converter.to_coin(pair.buy),
            );
            match exchange.get_my_orders(Some(pair), None, None, None).await {
                Ok(orders) => Ok(orders.into_iter()
                    .map(|order| agnostic::order::OrderWithId {
                        id: format!("{}", order.id),
                        coins: coins.clone(),
                        amount: f64::from_str(&order.amount).unwrap(),
                        price: f64::from_str(&order.rate).unwrap(),
                    })
                    .collect()),
                Err(error) => Err(error.to_string())
            }
        };
        Box::pin(future)
    }
}