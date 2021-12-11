############## Variables ###############
# ssh-key variable
variable "hcloud_ssh_key_pub" {
  default = "TODO_PASTE_SSH_KEY_HERE"
}

# stage variable
variable "stage" {
  default = "TODO_PASTE_stage_HERE"
}

# domain variable
variable "domain" {
  default = "TODO_PASTE_DOMAIN_HERE"
}

# network zone variable
variable "network_zone" {
  default = "TODO_PASTE_NETWORK_ZONE_HERE"
}

# network CIDR variable
variable "network_cidr" {
  default = "TODO_PASTE_CIDR_HERE"
}

# subnet CIDR variable
variable "subnet_cidr" {
  default = "TODO_PASTE_CIDR_HERE"
}

# nextcloud priv ip variable
variable "nextcloud_priv_ip" {
  default = "TODO_PASTE_IP_HERE"
}


##


# hub network object variable
variable "hub_network" {
  default = "TODO_PASTE_NETWORK_HERE"
}

# hub network cidr variable
variable "hub_network_cidr" {
  default = "TODO_PASTE_CIDR_HERE"
}
# spoke gw ip variable
variable "spoke_gw_ip" {
  default = "TODO_PASTE_IP_HERE"
}
