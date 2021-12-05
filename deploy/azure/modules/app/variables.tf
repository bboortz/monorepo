variable "configManagement" {
  type    = string
  default = "terraform"
}

variable "team" {
  type    = string
  default = "blueteam"
}

variable "environment" {
  type    = string
  default = "gettingstarted"
}

variable "stage" {
  type    = string
  default = "dev"
}

variable "depl" {
  type    = string
  default = "blue"
}

variable "location" {
  type    = string
  default = "westus2"
}

variable "cidr" {
  type    = string
  default = "10.1.0.0/16"
}

variable "appgwCidr" {
  type    = string
  default = "10.1.1.0/24"
}

variable "frontendCidr" {
  type    = string
  default = "10.1.2.0/24"
}
