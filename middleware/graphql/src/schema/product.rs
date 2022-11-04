use core::model::product::Product as ProductModel;
use juniper::{GraphQLInputObject, GraphQLObject};

use super::price::{NewPrice, Price};

#[derive(GraphQLObject, Debug, Clone)]
#[graphql(description = "A basic product representation")]
pub struct Product {
    pub key: String,
    pub name: String,
    pub price: Option<Price>,
    pub description: Option<String>,
}

impl From<ProductModel> for Product {
    fn from(item: ProductModel) -> Self {
        Product {
            key: item.key,
            name: item.name,
            description: item.description,
            price: item.price.map(Price::from),
        }
    }
}

impl From<Product> for ProductModel {
    fn from(item: Product) -> Self {
        ProductModel {
            key: item.key,
            name: item.name,
            description: item.description,
            price: item.price.map(From::from),
        }
    }
}

impl From<NewProduct> for Product {
    fn from(item: NewProduct) -> Self {
        Product {
            key: item.key,
            name: item.name,
            description: item.description,
            price: item.price.map(Price::from),
        }
    }
}

#[derive(GraphQLInputObject, Debug)]
#[graphql(description = "A new product representation")]
pub struct NewProduct {
    pub key: String,
    pub name: String,
    pub price: Option<NewPrice>,
    pub description: Option<String>,
}
