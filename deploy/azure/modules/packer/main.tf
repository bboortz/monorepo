resource "azurerm_resource_group" "packer" {
  name     = format("%s-%s-%s-%s-%s-%s", var.environment, var.stage, var.depl, var.location, "packer", "rg")
  location = var.location

  tags = {
    Team             = var.team
    Environment      = var.environment
    Stage            = var.stage
    Deployment       = var.depl
    Location         = var.location
    Name             = "app"
    ResourceType     = "rg"
    ConfigManagement = var.configManagement
  }
}
