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
    match (target, side) {
        (Target::Market, Side::Buy) => price.direct(),
        (Target::Market, Side::Sell) => price.reversed(),
        (Target::Limit, Side::Buy) => price.reversed(),
        (Target::Limit, Side::Sell) => price.direct(),
    }
}

pub fn convert_amount(target: Target, side: Side, price: &Price, amount: f64) -> f64 {
    let price = convert_price(target, side, &price);
    price * amount
}
