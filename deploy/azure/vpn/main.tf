# Configure the Azure provider
terraform {
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 2.84"
    }
    random = {
      source  = "hashicorp/random"
      version = "~> 3.1.0"
    }
    time = {
      source  = "hashicorp/time"
      version = "~> 0.7.2"
    }
  }

  required_version = ">= 1.0.7"

  #   backend "azurerm" {
  #     resource_group_name  = "howto-dev-central-westus2-tfstate-rg"
  #     storage_account_name = "howtodevtfstateh8m81"
  #     container_name       = "howto-dev-central-westus2-tfstate-sc"
  #     key                  = "howto-dev-terraform.tfstate"
  #   }
  # 
}

provider "azurerm" {
  features {}
}

module "tfstate" {
  source           = "../modules/tfstate"
  team             = var.team
  environment      = var.environment
  stage            = var.stage
  depl             = "central"
  location         = var.locationA
  configManagement = var.configManagement
}

#   module "packer" {
#     source           = "./modules/packer"
#     team             = var.team
#     environment      = var.environment
#     stage            = var.stage
#     depl             = var.deplBlue
#     location         = var.locationA
#     configManagement = var.configManagement
#     cidr             = "10.1.0.0/16"
#     appgwCidr        = "10.1.1.0/24"
#     frontendCidr     = "10.1.2.0/24"
#   }

# module "blueLocAApp" {
#   source           = "./modules/app"
#   team             = var.team
#   environment      = var.environment
#   stage            = var.stage
#   depl             = var.deplBlue
#   location         = var.locationA
#   configManagement = var.configManagement
#   cidr             = "10.1.0.0/16"
#   appgwCidr        = "10.1.1.0/24"
#   frontendCidr     = "10.1.2.0/24"
# }
# 
# module "greenLocBApp" {
#   source           = "./modules/app"
#   team             = var.team
#   environment      = var.environment
#   stage            = var.stage
#   depl             = var.deplGreen
#   location         = var.locationB
#   configManagement = var.configManagement
#   cidr             = "10.3.0.0/16"
#   appgwCidr        = "10.3.1.0/24"
#   frontendCidr     = "10.3.2.0/24"
# }
# 
# locals {
#   trafficMgnrTargetList = [module.blueLocAApp.helloworld_id, module.greenLocBApp.helloworld_id]
#   trafficMgnrStatusList = [var.deplBlueStatus, var.deplGreenStatus]
#   trafficMgnrWeightList = [var.deplBlueWeight, var.deplGreenWeight]
# }
# 
# module "trafficmanager" {
#   source               = "./modules/trafficmanager"
#   depends_on           = [module.blueLocAApp, module.greenLocBApp]
#   team                 = var.team
#   environment          = var.environment
#   stage                = var.stage
#   depl                 = var.deplGreen
#   location             = var.locationB
#   configManagement     = var.configManagement
#   targetResourceIdList = local.trafficMgnrTargetList
#   targetStatusList     = local.trafficMgnrStatusList
#   targetWeightList     = local.trafficMgnrWeightList
# }
# 
# 
# module "appgateway" {
#   source               = "./modules/appgateway"
#   depends_on           = [module.blueLocAApp, module.greenLocBApp]
#   team                 = var.team
#   environment          = var.environment
#   stage                = var.stage
#   depl                 = var.deplGreen
#   location             = var.locationB
#   configManagement     = var.configManagement
#   targetResourceIdList = local.trafficMgnrTargetList
#   targetStatusList     = local.trafficMgnrStatusList
#   targetWeightList     = local.trafficMgnrWeightList
#   cidr                 = "10.6.0.0/16"
#   appgwCidr            = "10.6.1.0/24"
#   frontendCidr         = "10.6.2.0/24"
# }
