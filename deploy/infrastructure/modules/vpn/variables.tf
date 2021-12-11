############## Variables ###############
# ssh-key variable
variable "hcloud_ssh_key_pub" {
  default = "TODO_PASTE_SSH_KEY_HERE"
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

# jumpy priv ip variable
variable "jumpy_priv_ip" {
  default = "TODO_PASTE_IP_HERE"
}

# vpn priv ip variable
variable "vpn_priv_ip" {
  default = "TODO_PASTE_IP_HERE"
}
