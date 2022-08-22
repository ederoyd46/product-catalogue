use lambda_http::{service_fn, tower::BoxError, Body, Request};

use core::model::product::Product;
use core::{error_and_panic, lambda_http_run};
use store_product::app;

use log::{error, info};

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    store_product::initialise_logger()?;
    info!("Initialise Store Product");
    lambda_http_run!(Product, app);
    info!("Initialise Store Completed");
    Ok(())
}
