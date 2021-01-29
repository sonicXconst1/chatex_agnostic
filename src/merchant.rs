use super::sniffer;
use super::accountant;
use super::trader;

pub struct ChatexMerchant {
}

impl agnostic::merchant::Merchant for ChatexMerchant {
    type Accountant = ChatexAccountant;
    type Trader = ChatexTrader;
    type Sniffer = ChatexSniffer;

    fn accountant(&self) -> Self::Accountant {
        todo!()
    }

    fn trader(&self) -> Self::Trader {
        todo!()
    }

    fn sniffer(&self) -> Self::Sniffer {
        todo!()
    }
}
