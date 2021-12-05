resource "azurerm_resource_group" "trafficmgnr" {
  name     = format("%s-%s-%s-%s-%s-%s", var.environment, var.stage, var.depl, var.location, "trafficmgnr", "rg")
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

resource "azurerm_traffic_manager_profile" "app" {
  name                   = format("%s-%s-%s-%s-%s-%s", var.environment, var.stage, var.depl, var.location, "app", "traffmgnr")
  resource_group_name    = azurerm_resource_group.trafficmgnr.name
  traffic_routing_method = "Weighted"

  dns_config {
    relative_name = format("%s-%s-%s-%s-%s-%s", var.environment, var.stage, var.depl, var.location, "app", "traffmgnr")
    ttl           = 60
  }

  monitor_config {
    protocol                     = "TCP"
    port                         = 80
    interval_in_seconds          = 30
    timeout_in_seconds           = 9
    tolerated_number_of_failures = 3
  }

  tags = {
    Team             = var.team
    Environment      = var.environment
    Stage            = var.stage
    Deployment       = var.depl
    Location         = var.location
    Name             = "app"
    ResourceType     = "traffmgnr"
    ConfigManagement = var.configManagement
  }
}

resource "azurerm_traffic_manager_endpoint" "app" {
  name                = format("%s-%s-%s-%s-%s-%s", var.environment, var.stage, var.depl, "global", "app", "traffmgnrendpoint")
  resource_group_name = azurerm_resource_group.trafficmgnr.name
  profile_name        = azurerm_traffic_manager_profile.app.name
  target_resource_id  = var.targetResourceId
  type                = "azureEndpoints"
  weight              = 100
}

