use lambda_http::tower::BoxError;

use ::store_dto::app;
use core::aws_lambda_http_post;
use core::model::product::Product;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    aws_lambda_http_post!(Product, app)
}
