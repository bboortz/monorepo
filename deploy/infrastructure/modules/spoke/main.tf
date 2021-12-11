terraform {
  required_providers {
    hcloud = {
      source  = "hetznercloud/hcloud"
      version = ">= 1.32.1"
    }
  }

  required_version = ">= 1.0.11"

}

resource "hcloud_firewall" "public-nextcloud-fw" {
  name = "public-nextcloud-fw"
  rule {
    direction = "in"
    protocol  = "tcp"
    port      = "65535"
    source_ips = [
      "1.1.1.1/32",
    ]
  }

}


#resource "hcloud_network" "spoke-network" {
#  name     = format("%s.%s", "spoke", var.domain)
#  ip_range = var.network_cidr
#  labels = {
#    type   = "network",
#    module = "spoke"
#    domain = var.domain
#  }
#}

resource "hcloud_network_subnet" "spoke-subnet" {
  #  depends_on = [
  #    hcloud_network.spoke-network
  #  ]
  type       = "cloud"
  network_id = var.hub_network.id
  # network_id   = hcloud_network.spoke-network.id
  network_zone = var.network_zone
  ip_range     = var.subnet_cidr
}

#resource "hcloud_network_route" "hub2spoke-gw" {
#  network_id  = var.hub_network.id
#  destination = var.hub_network_cidr
#  gateway     = var.spoke_gw_ip
#}


# Obtain ssh key data
data "hcloud_ssh_key" "ssh_key_pub" {
  fingerprint = var.hcloud_ssh_key_pub
}


# Create an spoke server
resource "hcloud_server" "nextcloud" {
  depends_on = [
    hcloud_network_subnet.spoke-subnet
  ]
  name        = format("%s.%s", "nextcloud", var.domain)
  image       = "ubuntu-20.04"
  server_type = "cx11"
  ssh_keys    = ["${data.hcloud_ssh_key.ssh_key_pub.id}"]
  firewall_ids = [
    hcloud_firewall.public-nextcloud-fw.id,
  ]
  labels = {
    type   = "server",
    module = "spoke",
    server = "spoke"
    domain = var.domain
  }
}

resource "hcloud_server_network" "nextcloud-nw" {
  server_id  = hcloud_server.nextcloud.id
  network_id = var.hub_network.id
  # network_id = hcloud_network.spoke-network.id
  ip = var.nextcloud_priv_ip
}
