provider "aws" {
  region = "eu-central-1"

  endpoints {
    sts            = "http://localhost:4566"
    config         = "http://localhost:4566"
    dynamodb       = "http://localhost:4566"
    lambda         = "http://localhost:4566"
    eventbridge    = "http://localhost:4566"
    s3             = "http://localhost:4566"
    cloudwatchlogs = "http://localhost:4566"
    iam            = "http://localhost:4566"
    apigateway     = "http://localhost:4566"
    secretsmanager = "http://localhost:4566"
  }
}

terraform {
  # backend "s3" {
  #   key          = "product-catalogue/terraform"
  #   encrypt      = true
  #   bucket       = "ederoyd"
  #   region       = "eu-central-1"
  # }

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "4.19.0"
    }
  }
}
