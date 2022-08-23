use lambda_http::{service_fn, tower::BoxError, Body, Request};

use core::model::product::Product;
use core::{aws_lambda_http, error_and_panic};
use store_product::app;

use log::{error, info, LevelFilter};

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    aws_lambda_http!(Product, app)
}
