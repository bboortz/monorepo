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
    # relative_name = format("%s-%s-%s-%s-%s-%s", var.environment, var.stage, var.depl, var.location, "app", "traffmgnr")
    relative_name = format("%s-%s-%s", var.environment, var.stage, "app")
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

# resource "azurerm_traffic_manager_endpoint" "app-blue" {
#   name                = format("%s-%s-%s-%s-%s-%s", var.environment, var.stage, var.depl, "global", "appBlue", "traffmgnrendpoint")
#   resource_group_name = azurerm_resource_group.trafficmgnr.name
#   profile_name        = azurerm_traffic_manager_profile.app.name
#   type                = "azureEndpoints"
#   target_resource_id  = var.targetResourceIdList[0]
#   endpoint_status     = var.targetStatusList[0]
#   weight              = var.targetWeightList[0]
# }

resource "azurerm_traffic_manager_endpoint" "app-green" {
  name                = format("%s-%s-%s-%s-%s-%s", var.environment, var.stage, var.depl, "global", "appGreen", "traffmgnrendpoint")
  resource_group_name = azurerm_resource_group.trafficmgnr.name
  profile_name        = azurerm_traffic_manager_profile.app.name
  type                = "azureEndpoints"
  target_resource_id  = var.targetResourceIdList[1]
  endpoint_status     = var.targetStatusList[1]
  weight              = var.targetWeightList[1]
}

resource "azurerm_traffic_manager_endpoint" "appgw-blue" {
  name                = format("%s-%s-%s-%s-%s-%s", var.environment, var.stage, var.depl, var.location, "appgw-green", "traffmgnrendpoint")
  resource_group_name = azurerm_resource_group.trafficmgnr.name
  profile_name        = azurerm_traffic_manager_profile.app.name
  type                = "externalEndpoints"
  target              = "howto-dev-green-westeurope-appgw.westeurope.cloudapp.azure.com"
  endpoint_status     = var.targetStatusList[0]
  weight              = var.targetWeightList[0]
}

