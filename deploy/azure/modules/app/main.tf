resource "azurerm_resource_group" "app" {
  name     = format("%s-%s-%s-%s-%s-%s", var.environment, var.stage, var.depl, var.location, "app", "rg")
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

resource "azurerm_app_service_plan" "appsrvplan" {
  name                = format("%s-%s-%s-%s-%s-%s", var.environment, var.stage, var.depl, var.location, "app", "appsrvplan")
  location            = azurerm_resource_group.app.location
  resource_group_name = azurerm_resource_group.app.name
  kind                = "Linux"
  reserved            = true


  sku {
    tier = "Standard"
    size = "S1"
  }

  tags = {
    Team             = var.team
    Environment      = var.environment
    Stage            = var.stage
    Deployment       = var.depl
    Location         = var.location
    Name             = "app"
    ResourceType     = "appsrvplan"
    ConfigManagement = var.configManagement
  }
}

resource "azurerm_app_service" "helloworld" {
  name                = format("%s-%s-%s-%s-%s-%s", var.environment, var.stage, var.depl, var.location, "helloworld", "appsrv")
  location            = azurerm_resource_group.app.location
  resource_group_name = azurerm_resource_group.app.name
  app_service_plan_id = azurerm_app_service_plan.appsrvplan.id
  https_only          = true

  site_config {
    app_command_line = ""
    linux_fx_version = "DOCKER|appsvcsample/python-helloworld:latest"
  }

  app_settings = {
    "WEBSITES_ENABLE_APP_SERVICE_STORAGE" = "false"
    "DOCKER_REGISTRY_SERVER_URL"          = "https://index.docker.io"
  }

  tags = {
    Team             = var.team
    Environment      = var.environment
    Stage            = var.stage
    Deployment       = var.depl
    Location         = var.location
    Name             = "helloworld"
    ResourceType     = "appsrv"
    ConfigManagement = var.configManagement
  }
}

resource "azurerm_app_service_custom_hostname_binding" "helloworld" {
  resource_group_name = azurerm_resource_group.app.name
  app_service_name    = azurerm_app_service.helloworld.name
  hostname            = format("%s-%s-%s-%s.%s", "hello", var.stage, var.depl, var.location, "srv.benni.tech")
}

resource "azurerm_app_service_custom_hostname_binding" "hello" {
  resource_group_name = azurerm_resource_group.app.name
  app_service_name    = azurerm_app_service.helloworld.name
  hostname            = format("%s-%s.%s", "hello", var.stage, "srv.benni.tech")
}

resource "azurerm_app_service_slot" "helloworld" {
  name                = format("%s-%s-%s-%s-%s-%s", var.environment, var.stage, var.depl, var.location, "helloworld", "appsrvslot")
  location            = azurerm_resource_group.app.location
  resource_group_name = azurerm_resource_group.app.name
  app_service_plan_id = azurerm_app_service_plan.appsrvplan.id
  app_service_name    = azurerm_app_service.helloworld.name

  tags = {
    Team             = var.team
    Environment      = var.environment
    Stage            = var.stage
    Deployment       = var.depl
    Location         = var.location
    Name             = "helloworld"
    ResourceType     = "appsrvslot"
    ConfigManagement = var.configManagement
  }
}
