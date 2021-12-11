# Output server IPs
output "jumpy-server_ip" {
  value = hcloud_server.jumpy.ipv4_address
}
output "vpn-server_ip" {
  value = hcloud_server.vpn.ipv4_address
}
output "network" {
  value = hcloud_network.vpn-network
}
