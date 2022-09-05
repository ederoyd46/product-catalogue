provider "aws" {
  region = "eu-central-1"

  access_key                  = "fake"
  secret_key                  = "fake"
  skip_credentials_validation = true
  skip_metadata_api_check     = true
  skip_requesting_account_id  = true

  endpoints {
    config         = "http://localhost:4566"
    dynamodb       = "http://localhost:4566"
    lambda         = "http://localhost:4566"
    eventbridge    = "http://localhost:4566"
    s3             = "http://localhost:4566"
    cloudwatchlogs = "http://localhost:4566"
    iam            = "http://localhost:4566"
    apigateway     = "http://localhost:4566"
  }
}

terraform {
  backend "s3" {
    key     = "product-catalogue/terraform"
    encrypt = true
    bucket  = "ederoyd"
    region  = "eu-central-1"
  }

  # required_version = "= 1.1.3"

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "4.19.0"
    }
  }
}
