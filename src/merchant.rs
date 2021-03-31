use super::accountant;
use super::sniffer;
use super::trader;

pub struct ChatexMerchant<TConnector> {
    accountant: std::sync::Arc<accountant::ChatexAccountant<TConnector>>,
    sniffer: std::sync::Arc<sniffer::ChatexSniffer<TConnector>>,
    trader: std::sync::Arc<trader::ChatexTrader<TConnector>>,
}

impl<TConnector> ChatexMerchant<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    pub fn new(client: std::sync::Arc<chatex_sdk_rust::ChatexClient<TConnector>>) -> Self {
        let accountant = std::sync::Arc::new(
            accountant::ChatexAccountant::new(client.clone()));
        let sniffer = std::sync::Arc::new(
            sniffer::ChatexSniffer::new(client.clone()));
        let trader = std::sync::Arc::new(
            trader::ChatexTrader::new(std::sync::Arc::new(client.exchange())));
        ChatexMerchant { 
            accountant,
            sniffer,
            trader,
        }
    }
}

impl<TConnector> agnostic::merchant::Merchant for ChatexMerchant<TConnector>
where
    TConnector: hyper::client::connect::Connect + Send + Sync + Clone + 'static,
{
    const ID: u8 = 1;

    fn accountant(&self) -> std::sync::Arc<dyn agnostic::market::Accountant> {
        self.accountant.clone()
    }

    fn trader(&self) -> std::sync::Arc<dyn agnostic::market::Trader> {
        self.trader.clone()
    }

    fn sniffer(&self) -> std::sync::Arc<dyn agnostic::market::Sniffer> {
        self.sniffer.clone()
    }
}
