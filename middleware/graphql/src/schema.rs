use juniper::FieldResult;
use juniper::{EmptySubscription, RootNode};
use std::env;

mod product;
mod price;
use core::model::product::Product as ProductModel;
use product::{NewProduct, Product};

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    async fn view_product(key: String) -> FieldResult<Product> {
        let retrieve_product_url =
            env::var("RETRIEVE_PRODUCT_URL").expect("RETRIEVE_PRODUCT_URL must be set");
        let response = reqwest::Client::new()
            .get(format!("{}/{}", retrieve_product_url, key))
            .send()
            .await;

        match response {
            Ok(response) => Ok(Product::from(
                response
                    .json::<ProductModel>()
                    .await
                    .expect("Failed to parse response"),
            )),
            Err(error) => Err(juniper::FieldError::new(
                "Error retrieving product",
                juniper::Value::scalar(error.to_string()),
            )),
        }
    }
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    async fn create_product(new_product: NewProduct) -> FieldResult<Product> {
        let store_product_url =
            env::var("STORE_PRODUCT_URL").expect("STORE_PRODUCT_URL must be set");

        let product = Product::from(new_product);
        let product_model: ProductModel = product.try_into().expect("count not be converted");

        let response = reqwest::Client::new()
            .post(store_product_url)
            .json(&product_model)
            .send()
            .await;

        match response {
            Ok(response) => Ok(Product::from(
                response
                    .json::<ProductModel>()
                    .await
                    .expect("Failed to parse response"),
            )),
            Err(error) => Err(juniper::FieldError::new(
                "Error retrieving product",
                juniper::Value::scalar(error.to_string()),
            )),
        }
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
