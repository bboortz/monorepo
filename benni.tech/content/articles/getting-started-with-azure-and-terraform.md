+++
title = "Getting Started with Azure and Terraform"
date = "2021-09-26T16:11:02+02:00"
author = ""
authorTwitter = "" #do not include @
cover = ""
tags = []
keywords = ["", ""]
description = ""
showFullContent = true
+++

Today I thought lets try [Azure](https://azure.com) as a cloud provider because I have never used Azure. 
I have used a couple of different other cloud provider but for whatever reason never Azure. 
First I had a look into how to provison resources on Azure in an automatd manner. 
Their own provisioning tool is [Azure Resource Manager](https://azure.microsoft.com/en-us/features/resource-manager/) which is a DSL that must be written in json.
Well, this is for me as an vim user. 
So that I have decided to do my first tries using a tool I am familier with. 
So that I have tried [Terraform](https://terraform.io) which is a nother language but from my perspective by far simpler to write using a simple text editor.


# Prerequesites

* You must have an Azure accunt.
* This how-to is written for linux. For other operating systems commands and behaviour might be different.

# Preperation

*Installing Terraform*

```
curl -O -L https://releases.hashicorp.com/terraform/1.0.7/terraform_1.0.7_linux_amd64.zip
terraform_1.0.7_linux_amd64.zip
terraform -version
```

*Installing Azure CLI*

```
curl -L https://aka.ms/InstallAzureCli | bash
```


# Getting Started

*Login via Azure CLI*

```
az login
az account show
```

*Write main.tf*

I had a look into the examples from [terraform-prodiver-azurerm](https://github.com/hashicorp/terraform-provider-azurerm/tree/main/examples) and compiled this simple `main.tf` which will only provision an Azure Resource Group.

```
# Configure the Azure provider
terraform {
  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "~> 2.78"
    }
  }

  required_version = ">= 1.0.7"
}

provider "azurerm" {
  features {}
}

resource "azurerm_resource_group" "rg" {
  name     = "test-rg"
  location = "westus2"
}
```


*Initialize Terraform*

```
terraform init
```

*Format Configuraton*

```
terraform fmt
```

*Validate Configuraton*

```
terraform validate
```

*Plan Configuration*

```
terraform plan
```

*Apply Configuration*

```
terraform apply
```



That's it for a getting started. 
