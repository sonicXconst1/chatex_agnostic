pub struct ChatexTrader {}

impl agnostic::market::Trader for ChatexTrader {
    fn create_order(
        &self,
        order: agnostic::order::Order,
    ) -> agnostic::market::Future<Result<(), String>> {
        todo!()
    }

    fn update_order(
        &self,
        id: &str,
        new_order: agnostic::order::Order,
    ) -> agnostic::market::Future<Result<(), String>> {
        todo!()
    }

    fn delete_order(&self, id: &str) -> agnostic::market::Future<Result<(), String>> {
        todo!()
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
