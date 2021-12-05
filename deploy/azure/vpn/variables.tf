variable "configManagement" {
  default = "terraform"
}

variable "team" {
  default = "benni"
}

variable "environment" {
  default = "vpn"
}

variable "stage" {
  default = "dev"
}

variable "deplBlue" {
  default = "blue"
}

variable "deplBlueStatus" {
  default = "Enabled"
}

variable "deplBlueWeight" {
  default = 100
}

variable "deplGreen" {
  default = "green"
}

variable "deplGreenStatus" {
  default = "Disabled"
}

variable "deplGreenWeight" {
  default = 100
}

variable "locationA" {
  default = "westus2"
}

variable "locationB" {
  default = "westeurope"
}
