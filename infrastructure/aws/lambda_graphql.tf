module "graphql" {
  source  = "terraform-aws-modules/lambda/aws"
  version = "3.3.1"

  function_name                     = "${terraform.workspace}-graphql"
  description                       = "GraphQL"
  handler                           = "does.not.matter"
  runtime                           = "provided"
  publish                           = true
  cloudwatch_logs_retention_in_days = 1
  create_package                    = false
  local_existing_package            = "../../deploy/lambda_graphql.zip"
  memory_size                       = 128
  timeout                           = 3

  environment_variables = {
    STORE_PRODUCT_URL   = aws_lambda_function_url.store_product.function_url
    STORE_INVENTORY_URL = aws_lambda_function_url.store_inventory.function_url
    RETRIEVE_PRODUCT_URL   = aws_lambda_function_url.retrieve_product.function_url
    RETRIEVE_INVENTORY_URL = aws_lambda_function_url.retrieve_inventory.function_url
  }
}

resource "aws_lambda_function_url" "graphql" {
  function_name      = module.graphql.lambda_function_name
  authorization_type = "NONE"
  cors {
    allow_credentials = true
    allow_origins     = ["*"]
    allow_methods     = ["GET", "POST"]
    allow_headers     = ["date", "keep-alive"]
    expose_headers    = ["keep-alive", "date"]
    max_age           = 86400
  }
}

# Outputs
output "graphql_lambda" {
  value = module.graphql.lambda_function_arn
}

output "graphql_lambda_log_group" {
  value = module.graphql.lambda_cloudwatch_log_group_name
}

output "graphql_url" {
  value = aws_lambda_function_url.graphql.function_url
}
