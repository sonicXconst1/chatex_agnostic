use agnostic::trading_pair::Target;
use agnostic::trading_pair::Side;

#[derive(Copy, Clone, Debug)]
pub struct Price {
    value: f64
}

impl From<f64> for Price {
    fn from(value: f64) -> Price {
        Price { value }
    }
}

impl Price {
    pub fn direct(&self) -> f64 {
        self.value
    }

    pub fn reversed(&self) -> f64 {
        1.0 / self.value
    }
}

pub fn convert_price(target: Target, side: Side, price: &Price) -> f64 {
    match side {
        Side::Sell => price.direct(),
        Side::Buy => price.reversed(),
    }
}

pub fn convert_amount(target: Target, side: Side, price: &Price, amount: f64) -> f64 {
    match side {
        Side::Sell => price.direct() * amount,
        Side::Buy => price.reversed() * amount,
    }
}
