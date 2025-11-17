// Scan related types
export interface ScanStats {
  totalFiles: number
  filesScanned: number
  threatsDetected: number
  threatsBlocked: number
  realTimeProtection: boolean
  lastScan: string
  lastUpdate: string
}

export interface ScanResult {
  id: string
  fileName: string
  filePath: string
  status: 'safe' | 'threat' | 'suspicious' | 'scanning'
  threatType?: string
  fileSize: string
  scanTime: string
}

export interface Threat {
  id: string
  name: string
  type: string
  severity: 'low' | 'medium' | 'high' | 'critical'
  detectedAt: string
  filePath: string
  status: 'quarantined' | 'removed' | 'active'
  hash?: string
  description?: string
}

export type ScanType = 'quick' | 'full' | 'custom'
export type ScanStatus = 'idle' | 'scanning' | 'paused' | 'completed' | 'error'