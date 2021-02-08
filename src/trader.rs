use agnostic::trading_pair::TradingPairConverter;
use crate::convert_price;

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
            let converter = crate::TradingPairConverter::default();
            let coins = converter.to_pair(order.trading_pair.clone());
            let price = convert_price(order.trading_pair.side.clone(), order.price);
            match client.create_order(coins, order.amount, price).await {
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
            let price = convert_price(new_order.trading_pair.side.clone(), new_order.price);
            let new_order = chatex_sdk_rust::models::UpdateOrder {
                amount: format!("{}", new_order.amount),
                rate: format!("{}", price),
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

    fn create_trade_by_id(&self, order_id: &str) -> agnostic::market::Future<Result<(), String>> {
        todo!()
    }

    fn create_trade_from_order(
        &self,
        order: agnostic::order::Order,
    ) -> agnostic::market::Future<Result<(), String>> {
        todo!()
    }
}
