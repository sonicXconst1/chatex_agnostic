pub struct ChatexTrader {
}

impl agnostic::market::Trader for ChatexTrader {
    fn create_order(&self, order: agnostic::order::Order) -> bool {
        todo!()
    }

    fn update_order(&self, id: &str, new_order: agnostic::order::Order) -> bool {
        todo!()
    }

    fn delete_order(&self, id: &str) -> bool {
        todo!()
    }

    fn create_trade_by_id(&self, order_id: &str) -> bool {
        todo!()
    }

    fn create_trade_from_order(&self, order: agnostic::order::Order) -> bool {
        todo!()
    }
}
