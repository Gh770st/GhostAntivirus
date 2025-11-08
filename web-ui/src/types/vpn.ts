// VPN related types
export interface VPNConnection {
  id: string
  server: string
  location: string
  country: string
  flag: string
  status: 'connected' | 'disconnected' | 'connecting'
  protocol: string
  ip: string
  connectedAt?: string
  latency: number
  load: number
}

export interface VPNStats {
  status: 'connected' | 'disconnected' | 'connecting'
  server?: string
  location?: string
  ip?: string
  protocol?: string
  connectedAt?: string
  bytesIn: number
  bytesOut: number
  uptime: string
  killSwitch: boolean
}

export type VPNStatus = 'connected' | 'disconnected' | 'connecting'
export type VPNProtocol = 'WireGuard' | 'OpenVPN' | 'IKEv2'