data "azurerm_client_config" "current" {}



#########

resource "random_string" "vault_id" {
  length  = 5
  special = false
  upper   = false
}


resource "azurerm_key_vault" "example" {
  name = format("%s%s", "examplekeyvault", random_string.vault_id.result)

  location                   = azurerm_resource_group.appgw.location
  resource_group_name        = azurerm_resource_group.appgw.name
  tenant_id                  = data.azurerm_client_config.current.tenant_id
  sku_name                   = "standard"
  soft_delete_retention_days = 7

  access_policy {
    tenant_id               = data.azurerm_client_config.current.tenant_id
    object_id               = azurerm_user_assigned_identity.agw.principal_id
    certificate_permissions = ["Get"]
    key_permissions         = []
    secret_permissions      = []
    storage_permissions     = []
    # object_id               = "ae4aca61-47f2-4177-927e-3f3b8f020fe9"
  }

  access_policy {
    tenant_id = data.azurerm_client_config.current.tenant_id
    object_id = data.azurerm_client_config.current.object_id

    certificate_permissions = [
      "create",
      "delete",
      "deleteissuers",
      "get",
      "getissuers",
      "import",
      "list",
      "listissuers",
      "managecontacts",
      "manageissuers",
      "purge",
      "setissuers",
      "update",
    ]

    key_permissions = [
      "backup",
      "create",
      "decrypt",
      "delete",
      "encrypt",
      "get",
      "import",
      "list",
      "purge",
      "recover",
      "restore",
      "sign",
      "unwrapKey",
      "update",
      "verify",
      "wrapKey",
    ]

    secret_permissions = [
      "backup",
      "delete",
      "get",
      "list",
      "purge",
      "recover",
      "restore",
      "set",
    ]
  }
}

resource "azurerm_key_vault_certificate" "example" {
  name         = "generated-cert"
  key_vault_id = azurerm_key_vault.example.id

  certificate_policy {
    issuer_parameters {
      name = "Self"
    }

    key_properties {
      exportable = true
      key_size   = 2048
      key_type   = "RSA"
      reuse_key  = true
    }

    lifetime_action {
      action {
        action_type = "AutoRenew"
      }

      trigger {
        days_before_expiry = 30
      }
    }

    secret_properties {
      content_type = "application/x-pkcs12"
    }

    x509_certificate_properties {
      # Server Authentication = 1.3.6.1.5.5.7.3.1
      # Client Authentication = 1.3.6.1.5.5.7.3.2
      extended_key_usage = ["1.3.6.1.5.5.7.3.1"]

      key_usage = [
        "cRLSign",
        "dataEncipherment",
        "digitalSignature",
        "keyAgreement",
        "keyCertSign",
        "keyEncipherment",
      ]

      subject_alternative_names {
        dns_names = ["internal.contoso.com", "domain.hello.world"]
      }

      subject            = "CN=hello-world"
      validity_in_months = 12
    }
  }
}


#########

resource "azurerm_user_assigned_identity" "agw" {
  name                = format("%s-%s-%s-%s-%s-%s", var.environment, var.stage, var.depl, var.location, "appgw", "identity")
  location            = azurerm_resource_group.appgw.location
  resource_group_name = azurerm_resource_group.appgw.name
  tags                = azurerm_resource_group.appgw.tags
}

#resource "azurerm_key_vault_access_policy" "builder" {
#  key_vault_id = azurerm_key_vault.example.id
#  tenant_id    = data.azurerm_client_config.current.tenant_id
#  object_id    = data.azurerm_client_config.current.object_id
# 
#   certificate_permissions = [
#    "create",
#    "get",
#    "list"
#  ]
#}

#resource "azurerm_key_vault_access_policy" "agw" {
#  key_vault_id = azurerm_key_vault.example.id
#  tenant_id    = data.azurerm_client_config.current.tenant_id
#  object_id    = azurerm_user_assigned_identity.agw.principal_id
#
#  secret_permissions = [
#    "get"
#  ]
#}

#########


#########

# 
# resource "azurerm_key_vault" "agw" {
#   name                = format("%s-%s-%s", var.environment, var.stage, var.location)
#   location            = azurerm_resource_group.appgw.location
#   resource_group_name = azurerm_resource_group.appgw.name
#   tenant_id           = data.azurerm_client_config.current.tenant_id
#   # soft_delete_enabled        = true #The EnableSoftDelete feature must be used for TLS termination to function properly. If you're configuring Key Vault soft-delete through the Portal, the retention period must be kept at 90 days, the default value. Application Gateway doesn't support a different retention period yet.
#   soft_delete_retention_days = 90
#   purge_protection_enabled   = false
#   sku_name                   = "standard"
# 
#   network_acls {
#     default_action = "Allow"
#     bypass         = "AzureServices"
#   }
# 
#   #   access_policy {
#   #     tenant_id               = data.azurerm_client_config.current.tenant_id
#   #     object_id               = azurerm_user_assigned_identity.agw.principal_id
#   #     certificate_permissions = ["Get"]
#   #     key_permissions         = []
#   #     secret_permissions      = []
#   #     storage_permissions     = []
#   #     # object_id               = "ae4aca61-47f2-4177-927e-3f3b8f020fe9"
#   #   }
#   #   access_policy {
#   #     tenant_id               = data.azurerm_client_config.current.tenant_id
#   #     object_id               = "71b2ed80-37a2-4173-b32f-433297685855"
#   #     certificate_permissions = ["Get"]
#   #     key_permissions         = []
#   #     secret_permissions      = []
#   #     storage_permissions     = []
#   #   }
# 
#   tags = azurerm_resource_group.appgw.tags
# }
# 
# resource "azurerm_key_vault_access_policy" "builder" {
#   key_vault_id = azurerm_key_vault.agw.id
#   tenant_id    = data.azurerm_client_config.current.tenant_id
#   object_id    = data.azurerm_client_config.current.object_id
# 
#   certificate_permissions = [
#     "create",
#     "get",
#     "list"
#   ]
# }
# 
# resource "azurerm_key_vault_access_policy" "agw" {
#   key_vault_id = azurerm_key_vault.agw.id
#   tenant_id    = data.azurerm_client_config.current.tenant_id
#   object_id    = azurerm_user_assigned_identity.agw.principal_id
# 
#   secret_permissions = [
#     "get"
#   ]
# }
# 
# resource "azurerm_key_vault_certificate" "mysite1" {
#   name         = format("%s-%s-%s-%s-%s-%s", var.environment, var.stage, var.depl, var.location, "appgw", "vaultCert")
#   key_vault_id = azurerm_key_vault.agw.id
# 
#   certificate_policy {
#     issuer_parameters {
#       name = "Self"
#     }
# 
#     key_properties {
#       exportable = true
#       key_size   = 2048
#       key_type   = "RSA"
#       reuse_key  = true
#     }
# 
#     lifetime_action {
#       action {
#         action_type = "AutoRenew"
#       }
# 
#       trigger {
#         days_before_expiry = 30
#       }
#     }
# 
#     secret_properties {
#       content_type = "application/x-pkcs12"
#     }
# 
#     x509_certificate_properties {
#       # Server Authentication = 1.3.6.1.5.5.7.3.1
#       # Client Authentication = 1.3.6.1.5.5.7.3.2
#       extended_key_usage = ["1.3.6.1.5.5.7.3.1"]
# 
#       key_usage = [
#         "cRLSign",
#         "dataEncipherment",
#         "digitalSignature",
#         "keyAgreement",
#         "keyCertSign",
#         "keyEncipherment",
#       ]
# 
#       subject_alternative_names {
#         dns_names = ["mysite1.com"]
#       }
# 
#       subject            = "CN=mysite1.com"
#       validity_in_months = 12
#     }
#   }
# }
# 
# resource "time_sleep" "wait_60_seconds" {
#   depends_on = [azurerm_key_vault_certificate.mysite1]
# 
#   create_duration = "60s"
# }
