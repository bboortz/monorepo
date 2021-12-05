locals {
  backend_address_pool_name            = "${azurerm_virtual_network.appgw.name}-beap"
  frontend_port_name                   = "${azurerm_virtual_network.appgw.name}-feport"
  https_frontend_port_name             = "${azurerm_virtual_network.appgw.name}-feport-https"
  frontend_ip_configuration_name       = "${azurerm_virtual_network.appgw.name}-feip"
  https_frontend_ip_configuration_name = "${azurerm_virtual_network.appgw.name}-feip-https"
  http_setting_name                    = "${azurerm_virtual_network.appgw.name}-be-htst"
  listener_name                        = "${azurerm_virtual_network.appgw.name}-httplstn"
  https_listener_name                  = "${azurerm_virtual_network.appgw.name}-httpslstn"
  request_routing_rule_name            = "${azurerm_virtual_network.appgw.name}-rqrt"
  https_request_routing_rule_name      = "${azurerm_virtual_network.appgw.name}-rqrt-https"
  redirect_configuration_name          = "${azurerm_virtual_network.appgw.name}-rdrcfg"
}

resource "azurerm_resource_group" "appgw" {
  name     = format("%s-%s-%s-%s-%s-%s", var.environment, var.stage, var.depl, var.location, "appgw", "rg")
  location = var.location

  tags = {
    Team             = var.team
    Environment      = var.environment
    Stage            = var.stage
    Deployment       = var.depl
    Location         = var.location
    Name             = "appgw"
    ResourceType     = "rg"
    ConfigManagement = var.configManagement
  }
}

# Create a virtual network
resource "azurerm_virtual_network" "appgw" {
  name                = format("%s-%s-%s-%s-%s-%s", var.environment, var.stage, var.depl, var.location, "appgw", "vnet")
  location            = azurerm_resource_group.appgw.location
  resource_group_name = azurerm_resource_group.appgw.name
  address_space       = [var.cidr]

  tags = {
    Team             = var.team
    Environment      = var.environment
    Stage            = var.stage
    Deployment       = var.depl
    Location         = var.location
    Name             = "appgw"
    ResourceType     = "vnet"
    ConfigManagement = var.configManagement
  }
}

resource "azurerm_subnet" "appgw" {
  name                 = format("%s-%s-%s-%s-%s-%s", var.environment, var.stage, var.depl, var.location, "appgw", "subnet")
  resource_group_name  = azurerm_resource_group.appgw.name
  virtual_network_name = azurerm_virtual_network.appgw.name
  address_prefixes     = [var.appgwCidr]
  depends_on           = [azurerm_virtual_network.appgw]
}

resource "azurerm_public_ip" "appgw" {
  name                = "example-pip"
  resource_group_name = azurerm_resource_group.appgw.name
  location            = azurerm_resource_group.appgw.location
  # allocation_method   = "Dynamic"
  allocation_method = "Static"
  sku               = "Standard"
  domain_name_label = format("%s-%s-%s-%s-%s", var.environment, var.stage, var.depl, var.location, "appgw")
}

resource "azurerm_application_gateway" "network" {
  name                = "example-appgateway"
  location            = azurerm_resource_group.appgw.location
  resource_group_name = azurerm_resource_group.appgw.name

  sku {
    name = "Standard_v2"
    tier = "Standard_v2"
    # name     = "Standard_Small"
    # tier     = "Standard"
    capacity = 2
  }

  identity {
    type         = "UserAssigned"
    identity_ids = [azurerm_user_assigned_identity.agw.id]
  }

  gateway_ip_configuration {
    name      = "my-gateway-ip-configuration"
    subnet_id = azurerm_subnet.appgw.id
  }

  frontend_port {
    name = local.frontend_port_name
    port = 80
  }

  frontend_port {
    name = local.https_frontend_port_name
    port = 443
  }

  frontend_ip_configuration {
    name                 = local.frontend_ip_configuration_name
    public_ip_address_id = azurerm_public_ip.appgw.id
  }

  backend_address_pool {
    name  = local.backend_address_pool_name
    fqdns = ["howto-dev-blue-westus2-helloworld-appsrv.azurewebsites.net"]
    # fqdns = ["${var.targetResourceIdList[0].name}.azurewebsites.net"]
  }

  backend_http_settings {
    name                  = local.http_setting_name
    cookie_based_affinity = "Disabled"
    path                  = "/"
    port                  = 80
    protocol              = "Http"
    request_timeout       = 3
    probe_name            = "probe"
  }

  http_listener {
    name                           = local.listener_name
    frontend_ip_configuration_name = local.frontend_ip_configuration_name
    frontend_port_name             = local.frontend_port_name
    protocol                       = "Http"
    # host_name                      = "hello.srv.benni.tech"
    host_names = [
      "hello-dev.srv.benni.tech",
      "howto-dev-app.trafficmanager.net"
    ]
  }

  http_listener {
    name                           = local.https_listener_name
    frontend_ip_configuration_name = local.frontend_ip_configuration_name
    frontend_port_name             = local.https_frontend_port_name
    protocol                       = "Https"
    ssl_certificate_name           = azurerm_key_vault_certificate.example.name
    host_names = [
      "hello-dev.srv.benni.tech",
      "howto-dev-app.trafficmanager.net"
    ]
  }

  probe {
    name     = "probe"
    protocol = "http"
    path     = "/"
    # host                = "${var.targetResourceIdList[0].name}.azurewebsites.net"
    host                = "howto-dev-blue-westus2-helloworld-appsrv.azurewebsites.net"
    interval            = "15"
    timeout             = "3"
    unhealthy_threshold = "3"
  }

  request_routing_rule {
    name                       = local.request_routing_rule_name
    rule_type                  = "Basic"
    http_listener_name         = local.listener_name
    backend_address_pool_name  = local.backend_address_pool_name
    backend_http_settings_name = local.http_setting_name
  }

  request_routing_rule {
    name                       = local.https_request_routing_rule_name
    rule_type                  = "Basic"
    http_listener_name         = local.https_listener_name
    backend_address_pool_name  = local.backend_address_pool_name
    backend_http_settings_name = local.http_setting_name
  }

  ssl_certificate {
    name                = azurerm_key_vault_certificate.example.name
    key_vault_secret_id = azurerm_key_vault_certificate.example.secret_id
  }
}
