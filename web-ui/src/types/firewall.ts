// Firewall related types
export interface FirewallRule {
  id: string
  name: string
  action: 'allow' | 'deny' | 'block'
  sourceIP: string
  destIP: string
  sourcePort?: number
  destPort?: number
  protocol: 'tcp' | 'udp' | 'icmp' | 'all'
  enabled: boolean
  priority: number
  created: string
}

export interface FirewallStats {
  totalRules: number
  activeRules: number
  blockedConnections: number
  allowedConnections: number
  threatsBlocked: number
}

export type FirewallAction = 'allow' | 'deny' | 'block'
export type NetworkProtocol = 'tcp' | 'udp' | 'icmp' | 'all'