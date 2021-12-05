output "resource_group_id" {
  value = azurerm_resource_group.app.id
}

# output "virtual_network_id" {
#   value = azurerm_virtual_network.app.id
# }

output "helloworld_id" {
  value = azurerm_app_service_slot.helloworld.id
}
