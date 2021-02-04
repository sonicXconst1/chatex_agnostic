use super::accountant;
use super::sniffer;
use super::trader;

pub struct ChatexMerchant<TConnector> {
    client: std::sync::Arc<chatex_sdk_rust::ChatexClient<TConnector>>,
}

impl<TConnector> ChatexMerchant<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    pub fn new(client: std::sync::Arc<chatex_sdk_rust::ChatexClient<TConnector>>) -> Self {
        ChatexMerchant { client }
    }
}

impl<TConnector> agnostic::merchant::Merchant for ChatexMerchant<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    type Accountant = accountant::ChatexAccountant<TConnector>;
    type Trader = trader::ChatexTrader<TConnector>;
    type Sniffer = sniffer::ChatexSniffer<TConnector>;

    fn accountant(&self) -> Self::Accountant {
        accountant::ChatexAccountant::new(self.client.clone())
    }

    fn trader(&self) -> Self::Trader {
        trader::ChatexTrader::new(std::sync::Arc::new(self.client.exchange()))
    }

    fn sniffer(&self) -> Self::Sniffer {
        sniffer::ChatexSniffer::new(self.client.clone())
    }
}
