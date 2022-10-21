use ::store_dto::app;
use core::aws_lambda_http_post;
use core::model::inventory::Inventory;
use lambda_http::tower::BoxError;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    aws_lambda_http_post!(Inventory, app)
}
