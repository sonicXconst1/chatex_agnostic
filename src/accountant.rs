use agnostic::trading_pair::Coin;
use agnostic::trading_pair::TradingPair;
use agnostic::trading_pair::TradingPairConverter;
use chatex_sdk_rust::models;

pub struct ChatexAccountant<TConnector> {
    client: std::sync::Arc<chatex_sdk_rust::ChatexClient<TConnector>>,
    price_epsilon: f64,
}

impl<TConnector> ChatexAccountant<TConnector> 
where 
    TConnector: hyper::client::connect::Connect + Sync + Send + Clone + 'static
{
    pub fn new(
        client: std::sync::Arc<chatex_sdk_rust::ChatexClient<TConnector>>,
    ) -> ChatexAccountant<TConnector> {
        ChatexAccountant {
            client,
            price_epsilon: 0.0001,
        }
    }
}

impl<TConnector> agnostic::market::Accountant for ChatexAccountant<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    fn ask(
        &self,
        coin: Coin,
    ) -> agnostic::market::Future<Result<agnostic::currency::Currency, String>> {
        let profile = self.client.profile();
        let converter = crate::converter::TradingPairConverter::default();
        let coin_as_string = String::from(converter.from_agnostic_coin(coin.clone()));
        let future = async move {
            match profile.get_balance_summary().await {
                Ok(balance) => balance
                    .into_iter()
                    .find(|currency| currency.coin == coin_as_string)
                    .map_or_else(
                        || Err("Failed to find currency".to_owned()),
                        |currency| {
                            let currency = models::typed::Currency::from(currency);
                            let coin = converter.to_agnostic_coin(currency.coin)
                                .expect("String representation is the same");
                            Ok(agnostic::currency::Currency {
                                coin,
                                amount: currency.amount,
                                held: currency.held,
                            })
                        },
                    ),
                Err(error) => Err(format!("{}", error)),
            }
        };
        Box::pin(future)
    }

    fn ask_both(
        &self,
        left: Coin,
        right: Coin,
    ) -> agnostic::market::Future<
        Result<(agnostic::currency::Currency, agnostic::currency::Currency), String>,
    > {
        let profile = self.client.profile();
        let converter = crate::converter::TradingPairConverter::default();
        let future = async move {
            let left_coin_as_string = String::from(converter.from_agnostic_coin(left.clone()));
            let right_coin_as_string = String::from(converter.from_agnostic_coin(right));
            match profile.get_balance_summary().await {
                Ok(balance) => {
                    let currencies: Vec<agnostic::currency::Currency> = balance
                        .into_iter()
                        .filter_map(|currency| {
                            if currency.coin != left_coin_as_string
                                && currency.coin != right_coin_as_string
                            {
                                None
                            } else {
                                let currency = models::typed::Currency::from(currency);
                                let coin = converter.to_agnostic_coin(currency.coin)
                                    .expect("String representation is the same!");
                                Some(agnostic::currency::Currency {
                                    coin,
                                    amount: currency.amount,
                                    held: currency.held,
                                })
                            }
                        })
                        .collect();
                    if currencies.len() == 2 {
                        let left_currency = currencies.get(0).unwrap().clone();
                        let right_currency = currencies.get(1).unwrap().clone();
                        if left_currency.coin == left {
                            Ok((left_currency, right_currency))
                        } else {
                            Ok((right_currency, left_currency))
                        }
                    } else {
                        Err("Invalid currencies. Found more then 2 currencies.".to_owned())
                    }
                }
                Err(error) => Err(format!("{}", error)),
            }
        };
        Box::pin(future)
    }

    fn calculate_volume(&self, _trading_pair: TradingPair, price: f64, amount: f64) -> f64 {
        price * amount
    }

    fn nearest_price(&self, _trading_pair: TradingPair, price: f64) -> f64 {
        price - self.price_epsilon
    }
}
