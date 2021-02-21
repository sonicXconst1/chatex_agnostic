use agnostic::trading_pair::TradingPair;
use agnostic::trading_pair::TradingPairConverter;

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
            let converter = crate::converter::TradingPairConverter::default();
            let pair = converter.to_pair(trading_pair.clone());
            log::debug!("Pair: {:#?}", String::from(pair.clone()));
            match exchange.get_all_orders(pair, None, Some(count)).await {
                Ok(orders) => Ok(orders
                    .into_iter()
                    .map(|order| {
                        let order = crate::order::Order::from_raw(
                            &trading_pair,
                            &order);
                        agnostic::order::Order {
                            trading_pair: trading_pair.clone(),
                            price: order.rate,
                            amount: order.amount,
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
            let converter = crate::converter::TradingPairConverter::default();
            let pair = converter.to_pair(trading_pair.clone());
            match exchange.get_all_orders(pair, None, Some(1)).await {
                Ok(orders) => {
                    let order = match orders.get(0) {
                        Some(order) => order,
                        None => return Err("0 orders from chatex API".to_owned()),
                    };
                    let order = crate::order::Order::from_raw(
                        &trading_pair,
                        order);
                    Ok(agnostic::order::Order {
                        trading_pair,
                        price: order.rate,
                        amount: order.amount,
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
            let converter = crate::converter::TradingPairConverter::default();
            let pair = converter.to_pair(trading_pair.clone());
            match exchange.get_my_orders(Some(pair), None, None, None).await {
                Ok(orders) => Ok(orders
                    .into_iter()
                    .map(|order| {
                        let order = crate::order::Order::from_raw(
                            &trading_pair,
                            &order);
                        agnostic::order::OrderWithId {
                            id: format!("{}", order.id.unwrap()),
                            trading_pair: trading_pair.clone(),
                            amount: order.amount,
                            price: order.rate,
                        }
                    })
                    .collect()),
                Err(error) => Err(error.to_string()),
            }
        };
        Box::pin(future)
    }
}

#[cfg(test)]
mod test {
    use crate::test;
    use crate::test::TestCase;
    use crate::converter;
    use agnostic::trading_pair::TradingPairConverter;
    use agnostic::market::Sniffer;

    fn create_sniffer(
        client: std::sync::Arc<chatex_sdk_rust::ChatexClient<test::Connector>>,
    ) -> super::ChatexSniffer<test::Connector> {
        super::ChatexSniffer::new(client)
    }

    #[test]
    fn all_the_best_orders() {
        let trading_pair = agnostic::trading_pair::TradingPair {
            coins: agnostic::trading_pair::Coins::TonUsdt,
            side: agnostic::trading_pair::Side::Buy,
            target: agnostic::trading_pair::Target::Market,
        };
        let input_price = 2.0;
        let input_amount = 2.0;
        test_trading_pair(trading_pair, input_price, 2.0, input_amount, 2.0);
        let trading_pair = agnostic::trading_pair::TradingPair {
            coins: agnostic::trading_pair::Coins::TonUsdt,
            side: agnostic::trading_pair::Side::Sell,
            target: agnostic::trading_pair::Target::Market,
        };
        test_trading_pair(trading_pair, input_price, 0.5, input_amount, 4.0);
        let trading_pair = agnostic::trading_pair::TradingPair {
            coins: agnostic::trading_pair::Coins::TonUsdt,
            side: agnostic::trading_pair::Side::Buy,
            target: agnostic::trading_pair::Target::Limit,
        };
        test_trading_pair(trading_pair, input_price, 0.5, input_amount, 4.0);
        let trading_pair = agnostic::trading_pair::TradingPair {
            coins: agnostic::trading_pair::Coins::TonUsdt,
            side: agnostic::trading_pair::Side::Sell,
            target: agnostic::trading_pair::Target::Limit,
        };
        test_trading_pair(trading_pair, input_price, 2.0, input_amount, 2.0);
    }

    #[test]
    fn get_my_orders() {
        let trading_pair = agnostic::trading_pair::TradingPair {
            coins: agnostic::trading_pair::Coins::TonUsdt,
            side: agnostic::trading_pair::Side::Buy,
            target: agnostic::trading_pair::Target::Market,
        };
        let input_price = 2.0;
        let input_amount = 2.0;
        let test_case = TestCase::default();
        let converter = converter::TradingPairConverter::default();
        let auth_mock = test_case.mock_access_token();
        let mock = test_case.server.mock(|when, then| {
            when.method(httpmock::Method::GET);
            let body: Vec<chatex_sdk_rust::models::Order> = vec![
                chatex_sdk_rust::models::typed::Order::new(
                    converter.to_pair(trading_pair.clone()),
                    input_price,
                    input_amount,
                ).into(),
            ];
            let body = serde_json::to_string(&body).expect(test::SERDE_ERROR);
            then.status(200)
                .header("Content-Type", "application/json")
                .body(body);
        });
        let sniffer = create_sniffer(test_case.client.clone());
        let my_orders = sniffer.get_my_orders(trading_pair);
        let my_orders = tokio_test::block_on(my_orders);
        assert!(
            my_orders.is_ok(),
            format!("Failed to get my orders: {:#?}", my_orders.err())
        );
        let my_orders = my_orders.unwrap();
        assert_eq!(my_orders.len(), 1, "Invalid amount of orders");
        auth_mock.assert();
        mock.assert();
    }

    fn test_trading_pair(
        trading_pair: agnostic::trading_pair::TradingPair,
        input_price: f64,
        expected_price: f64,
        input_amount: f64,
        expected_amount: f64,
    ) {
        let test_case = TestCase::default();
        let converter = converter::TradingPairConverter::default();
        let auth_mock = test_case.mock_access_token();
        let mock = test_case.server.mock(|when, then| {
            when.method(httpmock::Method::GET);
            let body: Vec<chatex_sdk_rust::models::Order> = vec![
                chatex_sdk_rust::models::typed::Order::new(
                    converter.to_pair(trading_pair.clone()),
                    input_price,
                    input_amount,
                ).into(),
            ];
            let body = serde_json::to_string(&body).expect(test::SERDE_ERROR);
            then.status(200)
                .header("Content-Type", "application/json")
                .body(body);
        });
        let sniffer = create_sniffer(test_case.client.clone());
        let orders = tokio_test::block_on(sniffer.all_the_best_orders(trading_pair, 10));
        assert!(
            orders.is_ok(),
            format!("Failed to get orders from server: {:#?}", orders.err()));
        let orders = orders.unwrap();
        assert_eq!(orders.len(), 1);
        let first_order = orders.get(0);
        assert!(first_order.is_some(), "Failed to get the first order");
        let first_order = first_order.unwrap();
        assert_eq!(first_order.price, expected_price, "Invlaid price");
        assert_eq!(first_order.amount, expected_amount, "Invalid amount");
        mock.assert();
        auth_mock.assert();
    }
}
