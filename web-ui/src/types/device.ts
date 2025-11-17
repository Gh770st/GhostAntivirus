// Device related types
export interface Device {
  id: string
  name: string
  type: 'desktop' | 'laptop' | 'mobile' | 'tablet'
  platform: 'windows' | 'macos' | 'linux' | 'android' | 'ios'
  status: 'online' | 'offline' | 'vulnerable'
  protection: 'protected' | 'unprotected' | 'partial'
  lastSeen: string
  ipAddress: string
  version: string
  threatsBlocked: number
  lastScan: string
  location?: string
}

export interface DeviceStats {
  totalDevices: number
  onlineDevices: number
  protectedDevices: number
  vulnerableDevices: number
  totalThreatsBlocked: number
}

export type DeviceType = 'desktop' | 'laptop' | 'mobile' | 'tablet'
export type DevicePlatform = 'windows' | 'macos' | 'linux' | 'android' | 'ios'
export type DeviceStatus = 'online' | 'offline' | 'vulnerable'
export type ProtectionStatus = 'protected' | 'unprotected' | 'partial'