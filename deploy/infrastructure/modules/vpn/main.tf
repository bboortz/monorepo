terraform {
  required_providers {
    hcloud = {
      source  = "hetznercloud/hcloud"
      version = ">= 1.32.1"
    }
  }

  required_version = ">= 1.0.11"

}

resource "hcloud_firewall" "public-ssh-fw" {
  name = "public-ssh-fw"
  rule {
    direction = "in"
    protocol  = "tcp"
    port      = "22"
    source_ips = [
      "0.0.0.0/0",
      "::/0"
    ]
  }

}

resource "hcloud_firewall" "public-vpn-fw" {
  name = "public-vpn-fw"
  rule {
    direction = "in"
    protocol  = "udp"
    port      = "9194"
    source_ips = [
      "0.0.0.0/0",
      "::/0"
    ]
  }

}


resource "hcloud_network" "vpn-network" {
  name     = format("%s.%s", "vpn", var.domain)
  ip_range = var.network_cidr
  labels = {
    type   = "network",
    module = "vpn"
    domain = var.domain
  }
}

resource "hcloud_network_subnet" "vpn-subnet" {
  depends_on = [
    hcloud_network.vpn-network
  ]
  type         = "cloud"
  network_id   = hcloud_network.vpn-network.id
  network_zone = var.network_zone
  ip_range     = var.subnet_cidr
}


# Obtain ssh key data
data "hcloud_ssh_key" "ssh_key_pub" {
  fingerprint = var.hcloud_ssh_key_pub
}

# Create an jump server
resource "hcloud_server" "jumpy" {
  depends_on = [
    hcloud_network_subnet.vpn-subnet
  ]
  name = format("%s.%s", "jumpy", var.domain)
  # name        = format("%s-%s-%s-%s-%s-%s", var.environment, var.stage, var.depl, var.location, "app", "appsrvplan")
  image       = "ubuntu-20.04"
  server_type = "cx11"
  ssh_keys    = ["${data.hcloud_ssh_key.ssh_key_pub.id}"]
  firewall_ids = [
    hcloud_firewall.public-ssh-fw.id
  ]
  labels = {
    type   = "server",
    module = "vpn",
    server = "jumpy"
    domain = var.domain
  }
}

resource "hcloud_server_network" "jumpy-nw" {
  server_id  = hcloud_server.jumpy.id
  network_id = hcloud_network.vpn-network.id
  ip         = var.jumpy_priv_ip
}

# Create an vpn server
resource "hcloud_server" "vpn" {
  depends_on = [
    hcloud_network_subnet.vpn-subnet
  ]
  name        = format("%s.%s", "vpn", var.domain)
  image       = "ubuntu-20.04"
  server_type = "cx11"
  ssh_keys    = ["${data.hcloud_ssh_key.ssh_key_pub.id}"]
  firewall_ids = [
    hcloud_firewall.public-vpn-fw.id,
  ]
  labels = {
    type   = "server",
    module = "vpn",
    server = "vpn"
    domain = var.domain
  }
}

resource "hcloud_server_network" "vpn-nw" {
  server_id  = hcloud_server.vpn.id
  network_id = hcloud_network.vpn-network.id
  ip         = var.vpn_priv_ip
}
