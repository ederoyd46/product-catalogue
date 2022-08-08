module "store_product" {
  source  = "terraform-aws-modules/lambda/aws"
  version = "3.3.1"

  function_name                     = "${terraform.workspace}-store-product"
  description                       = "Store Product"
  handler                           = "does.not.matter"
  runtime                           = "provided"
  publish                           = true
  cloudwatch_logs_retention_in_days = 1
  create_package                    = false
  local_existing_package            = "../../deploy/lambda_store_product.zip"
  memory_size                       = 128
  timeout                           = 3


  environment_variables = {
    DATABASE = aws_dynamodb_table.data_store.name
  }

  attach_policy_statements = true
  policy_statements = {
    service_config = {
      effect    = "Allow",
      actions   = ["dynamodb:PutItem"]
      resources = [aws_dynamodb_table.data_store.arn]
    },
  }

}
resource "aws_lambda_function_url" "store_product" {
  function_name = module.store_product.lambda_function_name
  # qualifier          = "db"
  authorization_type = "NONE"
  # authorization_type = "AWS_IAM"

  cors {
    allow_credentials = true
    allow_origins     = ["*"]
    allow_methods     = ["POST"]
    allow_headers     = ["date", "keep-alive"]
    expose_headers    = ["keep-alive", "date"]
    max_age           = 86400
  }
}

# Outputs
output "store_product_lambda" {
  value = module.store_product.lambda_function_arn
}

output "store_product_lambda_log_group" {
  value = module.store_product.lambda_cloudwatch_log_group_name
}

output "store_product_url" {
  value = aws_lambda_function_url.store_product.function_url
}
