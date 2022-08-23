use lambda_http::{service_fn, tower::BoxError, Body, Request};

use ::store_dto::app;
use core::model::inventory::Inventory;
use core::{aws_lambda_http, error_and_panic};

use log::{error, info, LevelFilter};

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    aws_lambda_http!(Inventory, app)
}
