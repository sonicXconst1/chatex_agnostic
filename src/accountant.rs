use agnostic::coin::CoinConverter;
use chatex_sdk_rust::models;

pub struct ChatexAccountant<TConnector> {
    client: std::sync::Arc<chatex_sdk_rust::ChatexClient<TConnector>>,
}

impl<TConnector> agnostic::market::Accountant for ChatexAccountant<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    fn ask(
        &self,
        coin: agnostic::coin::Coin,
    ) -> agnostic::market::Future<Result<agnostic::currency::Currency, String>> {
        let profile = self.client.profile();
        let converter = crate::CoinConverter::default();
        let future = async move {
            let coin_as_string = converter.to_string(coin);
            match profile.get_balance_summary().await {
                Ok(balance) => balance
                    .into_iter()
                    .find(|currency| currency.coin == coin_as_string)
                    .map_or_else(
                        || Err("Failed to find currency".to_owned()),
                        |currency| {
                            let currency = models::typed::Currency::from(currency);
                            let coin = converter.from_coin(currency.coin);
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
        coins: agnostic::coin::CoinPair,
    ) -> agnostic::market::Future<
        Result<(agnostic::currency::Currency, agnostic::currency::Currency), String>,
    > {
        let profile = self.client.profile();
        let converter = crate::CoinConverter::default();
        let future = async move {
            let sell_coin_as_string = converter.to_string(coins.sell);
            let buy_coin_as_string = converter.to_string(coins.buy);
            match profile.get_balance_summary().await {
                Ok(balance) => {
                    let currencies: Vec<agnostic::currency::Currency> = balance
                        .into_iter()
                        .filter_map(|currency| {
                            if currency.coin != sell_coin_as_string
                                && currency.coin != buy_coin_as_string
                            {
                                None
                            } else {
                                let currency = models::typed::Currency::from(currency);
                                let coin = converter.from_coin(currency.coin);
                                Some(agnostic::currency::Currency {
                                    coin,
                                    amount: currency.amount,
                                    held: currency.held,
                                })
                            }
                        })
                        .collect();
                    if currencies.len() == 2 {
                        let left = currencies.get(0).unwrap().clone();
                        let right = currencies.get(1).unwrap().clone();
                        Ok((left, right))
                    } else {
                        Err("Invalid currencies. Found more then 2 currencies.".to_owned())
                    }
                }
                Err(error) => Err(format!("{}", error)),
            }
        };
        Box::pin(future)
    }

    fn calculate_volume(&self, _coins: agnostic::coin::CoinPair, price: f64, amount: f64) -> f64 {
        price * amount
    }
}
