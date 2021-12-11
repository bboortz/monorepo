terraform {
  required_providers {
    random = {
      source  = "hashicorp/random"
      version = ">= 3.1.0"
    }
    time = {
      source  = "hashicorp/time"
      version = ">= 0.7.2"
    }
    hcloud = {
      source  = "hetznercloud/hcloud"
      version = ">= 1.32.1"
    }
  }

  required_version = ">= 1.0.11"

}


# Define Hetzner provider
provider "hcloud" {
  token = var.hcloud_token
}

locals {
  hub_network_cidr  = "10.2.0.0/16"
  hub_subnet_cidr   = "10.2.1.0/24"
  spoke_subnet_cidr = "10.2.2.0/24"
  # spoke_subnet_cidr= "10.3.1.0/24"
}


module "vpn" {
  source             = "./modules/vpn"
  hcloud_ssh_key_pub = var.hcloud_ssh_key_pub
  domain             = var.domain
  network_zone       = var.network_zone
  network_cidr       = local.hub_network_cidr
  subnet_cidr        = local.hub_subnet_cidr
  jumpy_priv_ip      = var.jumpy_priv_ip
  vpn_priv_ip        = var.vpn_priv_ip
}


module "spoke" {
  source             = "./modules/spoke"
  hcloud_ssh_key_pub = var.hcloud_ssh_key_pub
  stage              = var.stage
  domain             = format("%s.%s", var.stage, var.domain)
  network_zone       = var.network_zone
  # network_cidr       = "10.3.0.0/16"
  subnet_cidr = local.spoke_subnet_cidr
  # subnet_cidr        = "10.3.1.0/24"
  nextcloud_priv_ip = var.nextcloud_priv_ip
  hub_network       = module.vpn.network
  hub_network_cidr  = local.hub_subnet_cidr
  spoke_gw_ip       = "10.3.1.1"
}

