use lambda_http::{service_fn, tower::BoxError, Body, Request};

use core::error_and_panic;
use core::model::product::Product;

use log::{error, info};

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    store_product::initialise_logger()?;
    info!("Initialise Store Product");

    lambda_http::run(service_fn(move |event: Request| {
        let body = match event.body() {
            Body::Text(val) => val.as_ref(),
            Body::Binary(val) => std::str::from_utf8(val).unwrap(),
            Body::Empty => error_and_panic!("Invalid input, please use a string"),
        };
        let value: Product = match serde_json::from_str(body) {
            Ok(item) => item,
            Err(e) => error_and_panic!("Could not parse input to known type", e),
        };

        store_product::app(value.clone())
    }))
    .await?;

    Ok(())
}
