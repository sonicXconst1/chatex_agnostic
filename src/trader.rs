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
                let order_amount = f64::from_str(&order.amount).unwrap();
                let rate = converted_order.rate;
                let amount = converted_order.amount;
                order_rate == rate && order_amount == amount
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
