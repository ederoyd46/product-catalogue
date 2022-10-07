module "retrieve_product" {
  source  = "terraform-aws-modules/lambda/aws"
  version = "3.3.1"

  function_name                     = "${terraform.workspace}-retrieve-product"
  description                       = "Retrieve Product"
  handler                           = "does.not.matter"
  runtime                           = "provided"
  publish                           = true
  cloudwatch_logs_retention_in_days = 1
  create_package                    = false
  local_existing_package            = "../../deploy/lambda_retrieve_product.zip"
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
resource "aws_lambda_function_url" "retrieve_product" {
  function_name = module.retrieve_product.lambda_function_name
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
output "retrieve_product_lambda" {
  value = module.retrieve_product.lambda_function_arn
}

output "retrieve_product_lambda_log_group" {
  value = module.retrieve_product.lambda_cloudwatch_log_group_name
}

output "retrieve_product_url" {
  value = aws_lambda_function_url.retrieve_product.function_url
}
