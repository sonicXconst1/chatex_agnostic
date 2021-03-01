use crate::order::Order;
use std::str::FromStr;

pub struct ChatexTrader<TConnector> {
    client: std::sync::Arc<chatex_sdk_rust::ExchangeClient<TConnector>>,
}

impl<TConnector> ChatexTrader<TConnector>
where
    TConnector: hyper::client::connect::Connect + Sync + Send + Clone + 'static,
{
    pub fn new(
        client: std::sync::Arc<chatex_sdk_rust::ExchangeClient<TConnector>>,
    ) -> ChatexTrader<TConnector> {
        ChatexTrader { client }
    }
}

impl<TConnector> agnostic::market::Trader for ChatexTrader<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    fn create_order(
        &self,
        order: agnostic::order::Order,
    ) -> agnostic::market::Future<Result<(), String>> {
        let client = self.client.clone();
        let future = async move {
            let converted_order: Order = order.into();
            match client
                .create_order(
                    converted_order.pair,
                    converted_order.amount,
                    converted_order.amount,
                )
                .await
            {
                Ok(order) => {
                    log::debug!("Order created: {:#?}", order);
                    Ok(())
                }
                Err(error) => Err(format!("{}", error)),
            }
        };
        Box::pin(future)
    }

    fn delete_and_create(
        &self,
        id: &str,
        new_order: agnostic::order::Order,
    ) -> agnostic::market::Future<Result<String, String>> {
        let client = self.client.clone();
        let id = id.to_owned();
        let future = async move {
            let converted_order: Order = new_order.into();
            let new_order = chatex_sdk_rust::models::UpdateOrder {
                amount: format!("{}", converted_order.rate),
                rate: format!("{}", converted_order.amount),
            };
            match client.update_order_by_id(&id, &new_order).await {
                Ok(order) => {
                    log::debug!("Order updated: {:#?}", order);
                    Ok(format!("{}", order.id))
                }
                Err(error) => Err(format!("{}", error)),
            }
        };
        Box::pin(future)
    }

    fn delete_order(&self, id: &str) -> agnostic::market::Future<Result<(), String>> {
        let client = self.client.clone();
        let id = id.to_owned();
        let future = async move {
            match client.delete_order_by_id(&id).await {
                Ok(order) => {
                    log::debug!("Order deleted: {:#?}", order);
                    Ok(())
                }
                Err(error) => Err(format!("{}", error)),
            }
        };
        Box::pin(future)
    }

    fn create_trade_from_order(
        &self,
        order: agnostic::order::Order,
    ) -> agnostic::market::Future<Result<(), String>> {
        let client = self.client.clone();
        let future = async move {
            let converted_order: Order = order.clone().into();
            let orders = match client
                .get_all_orders(converted_order.pair.clone(), None, Some(30))
                .await
            {
                Ok(orders) => orders,
                Err(error) => return Err(format!("{:#?}", error)),
            };
            if let Some(order) = orders.iter().find(|order| {
                let order_rate = f64::from_str(&order.rate).unwrap();
                let rate = converted_order.rate;
                order_rate == rate
            }) {
                let trade = chatex_sdk_rust::models::CreateTradeRequest {
                    amount: converted_order.amount.to_string(),
                    rate: converted_order.rate.to_string(),
                };
                match client
                    .create_trade_for_order(&order.id.to_string(), &trade)
                    .await
                {
                    Ok(trade) => {
                        log::debug!("Trade success: {:#?}", trade);
                        Ok(())
                    }
                    Err(error) => Err(error.to_string()),
                }
            } else {
                Err(format!("Failed to find the order: {:#?}", order))
            }
        };
        Box::pin(future)
    }
}

#[cfg(test)]
mod test {
    use crate::test::TestCase;
    use crate::test::SERDE_ERROR;
    use crate::test::Connector;
    use agnostic::trading_pair::{TradingPair, Coins, Target, Side};
    use agnostic::market::Trader;
    use super::*;

    fn create_trader(
        client: std::sync::Arc<chatex_sdk_rust::ChatexClient<Connector>>
    ) -> std::sync::Arc<ChatexTrader<Connector>> {
        std::sync::Arc::new(ChatexTrader::new(std::sync::Arc::new(client.exchange())))
    }

    #[test]
    fn create_trade_from_order() {
        let test_case = TestCase::default();
        let auth_mock = test_case.mock_access_token();
        let orders_mock = test_case.server.mock(|when, then| {
            when.method(httpmock::Method::GET);
            let order = chatex_sdk_rust::models::typed::Order::new(
                chatex_sdk_rust::coin::CoinPair::new(
                    chatex_sdk_rust::coin::Coin::USDT,
                    chatex_sdk_rust::coin::Coin::TON),
                0.5,
                4.0);
            let body: chatex_sdk_rust::models::Order = order.clone().into();
            let body = vec![body];
            let body = serde_json::to_string(&body).expect(SERDE_ERROR);
            then
                .status(200)
                .header("Content-Type", "application/json")
                .body(body);
        });
        let trade_mock = test_case.server.mock(|when, then| {
            let order = chatex_sdk_rust::models::typed::Order::new(
                chatex_sdk_rust::coin::CoinPair::new(
                    chatex_sdk_rust::coin::Coin::TON,
                    chatex_sdk_rust::coin::Coin::USDT),
                0.5,
                4.0);
            let trade = chatex_sdk_rust::models::CreateTradeRequest {
                amount: format!("{}", order.amount),
                rate: format!("{}", order.rate),
            };
            let body = serde_json::to_string(&trade).expect(SERDE_ERROR);
            when
                .body(body);
            let trade: chatex_sdk_rust::models::Trade = order.into();
            let trade = serde_json::to_string(&trade).expect(SERDE_ERROR);
            then.status(201)
                .header("Content-Type", "application/json")
                .body(trade);
        });
        let trader = create_trader(test_case.client.clone());
        let order = agnostic::order::Order {
            trading_pair: TradingPair {
                coins: Coins::TonUsdt,
                target: Target::Market,
                side: Side::Sell,
            },
            amount: 2.0,
            price: 2.0,
        };
        let trade_result = trader.create_trade_from_order(order);
        let trade_result = tokio_test::block_on(trade_result);
        assert!(
            trade_result.is_ok(),
            format!("Failed to create the trade: {:#?}", trade_result.err()));
        auth_mock.assert_hits(2);
        orders_mock.assert();
        trade_mock.assert()
    }
}
