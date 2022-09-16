use juniper::FieldResult;
use juniper::{EmptySubscription, RootNode};
use std::env;

mod product;
use core::model::product::{Price as PriceModel, Product as ProductModel};
use product::{NewProduct, Price, Product};

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn view_product(_id: String) -> FieldResult<Product> {
        Ok(Product {
            key: "1234".to_owned(),
            name: "Cardboard Box".to_owned(),
            description: Some("A cardboard box".to_owned()),
            price: None,
        })
    }
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    async fn create_product(new_product: NewProduct) -> FieldResult<Product> {
        let store_product_url =
            env::var("STORE_PRODUCT_URL").expect("STORE_PRODUCT_URL must be set");
        let product = ProductModel {
            key: new_product.key.to_string(),
            name: new_product.name,
            description: new_product.description,
            price: Some({
                PriceModel {
                    amount: 1.0,
                    currency_code: "GBP".to_string(),
                }
            }),
        };

        let response = reqwest::Client::new()
            .post(store_product_url)
            .json(&product)
            .send()
            .await
            .unwrap()
            .json::<ProductModel>()
            .await
            .unwrap();

        Ok(Product {
            key: response.key,
            name: response.name,
            description: response.description,
            price: response.price.map(|price| Price {
                amount: price.amount,
                currency_code: price.currency_code,
            }),
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
