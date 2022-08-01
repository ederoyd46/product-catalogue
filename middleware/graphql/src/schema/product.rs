use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject)]
#[graphql(description = "A price structure")]
pub struct Price {
    pub currency_code: String,
    pub amount: f64,
    pub precision: Option<i32>,
}

#[derive(GraphQLObject)]
#[graphql(description = "A basic product representation")]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub price: Option<Price>,
}

#[derive(GraphQLInputObject, Debug)]
#[graphql(description = "A new price structure")]
pub struct NewPrice {
    pub currency_code: String,
    pub amount: f64,
    // pub precision: Option<i32>,
}

#[derive(GraphQLInputObject, Debug)]
#[graphql(description = "A new product representation")]
pub struct NewProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: NewPrice,
}
