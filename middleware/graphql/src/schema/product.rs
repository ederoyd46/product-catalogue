use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject)]
#[graphql(description = "A price structure")]
pub struct Price {
    pub currency_code: String,
    pub amount: f64,
}

#[derive(GraphQLObject)]
#[graphql(description = "A basic product representation")]
pub struct Product {
    pub key: String,
    pub name: String,
    pub price: Option<Price>,
    pub description: Option<String>,
}

#[derive(GraphQLInputObject, Debug)]
#[graphql(description = "A new price structure")]
pub struct NewPrice {
    pub currency_code: String,
    pub amount: f64,
}

#[derive(GraphQLInputObject, Debug)]
#[graphql(description = "A new product representation")]
pub struct NewProduct {
    pub key: String,
    pub name: String,
    pub price: Option<NewPrice>,
    pub description: Option<String>,
}
