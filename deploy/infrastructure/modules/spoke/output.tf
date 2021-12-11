# Output server IPs
output "nextcloud-server_ip" {
  value = hcloud_server.nextcloud.ipv4_address
}
