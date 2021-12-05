resource "random_string" "resource_code" {
  length  = 5
  special = false
  upper   = false
}

resource "azurerm_resource_group" "tfstate" {
  name     = format("%s-%s-%s-%s-%s-%s", var.environment, var.stage, var.depl, var.location, "tfstate", "rg")
  location = var.location

  tags = {
    Team             = var.team
    Environment      = var.environment
    Stage            = var.stage
    Deployment       = var.depl
    Location         = var.location
    ResourceType     = "rg"
    Name             = "tfstate"
    ConfigManagement = var.configManagement
  }
}

resource "azurerm_storage_account" "tfstate" {
  name                     = format("%s%s%s", var.environment, var.stage, "tfstate${random_string.resource_code.result}")
  resource_group_name      = azurerm_resource_group.tfstate.name
  location                 = var.location
  account_tier             = "Standard"
  account_replication_type = "LRS"
  allow_blob_public_access = true

  tags = {
    Team             = var.team
    Environment      = var.environment
    Stage            = var.stage
    Deployment       = var.depl
    Location         = var.location
    ResourceType     = "sa"
    Name             = "tfstate"
    ConfigManagement = var.configManagement
  }
}

resource "azurerm_storage_container" "tfstate" {
  name                  = format("%s-%s-%s-%s-%s-%s", var.environment, var.stage, var.depl, var.location, "tfstate", "sc")
  storage_account_name  = azurerm_storage_account.tfstate.name
  container_access_type = "blob"
}
