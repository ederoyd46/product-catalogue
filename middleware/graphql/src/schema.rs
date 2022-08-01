use juniper::FieldResult;
use juniper::{EmptySubscription, RootNode};

mod product;
use product::{NewProduct, Product};

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn view_product(_id: String) -> FieldResult<Product> {
        Ok(Product {
            id: "1234".to_owned(),
            name: "Cardboard Box".to_owned(),
            description: Some("A cardboard box".to_owned()),
            price: None,
        })
    }
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn create_product(_new_product: NewProduct) -> FieldResult<Product> {
        Ok(Product {
            id: "1234".to_owned(),
            name: "Cardboard Box".to_owned(),
            description: Some("A cardboard box".to_owned()),
            price: None,
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
