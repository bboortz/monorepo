# Output server IPs
output "jumpy-server_ip" {
  value = module.vpn.jumpy-server_ip
}
output "vpn-server_ip" {
  value = module.vpn.vpn-server_ip
}
output "spoke-nextcloud-server_ip" {
  value = module.spoke.nextcloud-server_ip
}
