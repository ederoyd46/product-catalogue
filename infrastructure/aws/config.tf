provider "aws" {
  region = "eu-central-1"
  endpoints {
    s3             = "http://localhost:4566"
    lambda         = "http://localhost:4566"
    dynamodb       = "http://localhost:4566"
    cloudwatchlogs = "http://localhost:4566"
    sts            = "http://localhost:4566"
    config         = "http://localhost:4566"
    eventbridge    = "http://localhost:4566"
    iam            = "http://localhost:4566"
    apigateway     = "http://localhost:4566"
    secretsmanager = "http://localhost:4566"
  }

  # dynamic "endpoints" {
  #   for_each = [for n in [var.deploy_mode] : n if n == "localstack"]
  #   content {
  #     sts            = "http://localhost:4566"
  #     config         = "http://localhost:4566"
  #     dynamodb       = "http://localhost:4566"
  #     lambda         = "http://localhost:4566"
  #     eventbridge    = "http://localhost:4566"
  #     s3             = "http://localhost:4566"
  #     cloudwatchlogs = "http://localhost:4566"
  #     iam            = "http://localhost:4566"
  #     apigateway     = "http://localhost:4566"
  #     secretsmanager = "http://localhost:4566"
  #   }
  # }
}

terraform {
  backend "s3" {
    key      = "product-catalogue/terraform"
    encrypt  = false
    bucket   = "ederoyd"
    region   = "eu-central-1"
    endpoint = "http://localhost.localstack.cloud:4566"
  }

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "4.19.0"
    }
  }
}
