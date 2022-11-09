use core::model::product::Price as PriceModel;
use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject, Debug, Clone)]
#[graphql(description = "A price structure")]
pub struct Price {
    pub currency_code: String,
    pub amount: f64,
}

impl From<PriceModel> for Price {
    fn from(item: PriceModel) -> Self {
        Self {
            amount: item.amount,
            currency_code: item.currency_code,
        }
    }
}

impl From<Price> for PriceModel {
    fn from(item: Price) -> Self {
        Self {
            amount: item.amount,
            currency_code: item.currency_code,
        }
    }
}

impl From<NewPrice> for Price {
    fn from(item: NewPrice) -> Self {
        Self {
            amount: item.amount,
            currency_code: item.currency_code,
        }
    }
}

#[derive(GraphQLInputObject, Debug)]
#[graphql(description = "A new price structure")]
pub struct NewPrice {
    pub currency_code: String,
    pub amount: f64,
}
