variable "region" {
  type        = string
  description = "Region for resources"
  default     = "eu-central-1"
}
variable "deploy_mode" {
  type        = string
  description = "Deploy to AWS or Localstack"
  default     = "localstack"
}
