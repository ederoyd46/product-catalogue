use lambda_http::tower::BoxError;

use ::retrieve_dto::app;
use core::aws_lambda_http_get;
use core::model::inventory_search::InventorySearch;
use core::model::DataQueryObject;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    aws_lambda_http_get!(InventorySearch, app)
}
