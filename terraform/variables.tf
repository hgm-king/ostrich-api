variable "environment_tag" {
  description = "Environment"
  default     = "dev"
}

variable "auth_ecs_name" {
  description = "Name primitive to use for all resources created"
  type        = string
  default     = "AuthEcsService"
}

variable "user_ecs_name" {
  description = "Name primitive to use for all resources created"
  type        = string
  default     = "UserEcsService"
}

variable "email_job_ecs_name" {
  description = "Name primitive to use for all resources created"
  type        = string
  default     = "EmailJobEcsService"
}

variable "auth_repository_name" {
  description = "Value of the Name tag for the Auth ECR Repository"
  type        = string
  default     = "ostrich_auth_repository"
}

variable "user_repository_name" {
  description = "Value of the Name tag for the User ECR Repository"
  type        = string
  default     = "ostrich_user_repository"
}

variable "email_job_repository_name" {
  description = "Value of the Name tag for the User ECR Repository"
  type        = string
  default     = "ostrich_email_job_repository"
}

variable "cidr_vpc" {
  description = "CIDR block for the VPC"
  default     = "10.0.0.0/24"
}

variable "cidr_subnet1" {
  description = "CIDR block for the subnet"
  default     = "10.0.0.0/25"
}

variable "cidr_subnet2" {
  description = "CIDR block for the subnet"
  default     = "10.0.0.128/25"
}

# Fargate
variable "fargate_cpu" {
  description = "CPU size for fargate"
  default     = 1024
  type        = number
}
variable "fargate_mem" {
  description = "Memory to use for fargate"
  default     = 2048
  type        = number
}

# Builder container
variable "container_cpu" {
  description = "CPU to use for container, must be equal or less than fargate"
  default     = 1024
  type        = number
}
variable "container_mem" {
  description = "Memory to use for container, must be equal or less than fargate"
  default     = 2048
  type        = number
}

variable "image_tag" {
  description = "Tag to use when pulling ECR image"
  default     = "latest"
  type        = string
}