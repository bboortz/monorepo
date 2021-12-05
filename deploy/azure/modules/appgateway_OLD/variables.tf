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

variable "targetResourceId" {
  type    = string
  default = "123"
}
