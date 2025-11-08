// API Service - HTTP Client for GhostAntivirus Backend
import axios, { AxiosInstance, AxiosError, AxiosRequestConfig } from 'axios'

// API Configuration
const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8000'
const CORE_API_URL = import.meta.env.VITE_CORE_URL || 'http://localhost:9000'
const NETWORK_API_URL = import.meta.env.VITE_NETWORK_URL || 'http://localhost:8080'

// Types
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
}

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

export interface VPNConnection {
  id: string
  server: string
  location: string
  status: 'connected' | 'disconnected' | 'connecting'
  protocol: string
  ip: string
  connectedAt?: string
  bytesIn: number
  bytesOut: number
}

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
}

export interface PasswordEntry {
  id: string
  title: string
  username: string
  password: string
  url?: string
  category: 'email' | 'finance' | 'social' | 'work' | 'general'
  notes?: string
  created: string
  lastModified: string
}

// API Client Class
class ApiClient {
  private aiClient: AxiosInstance
  private coreClient: AxiosInstance
  private networkClient: AxiosInstance

  constructor() {
    // AI Engine Client
    this.aiClient = axios.create({
      baseURL: API_BASE_URL,
      timeout: 30000,
      headers: {
        'Content-Type': 'application/json',
      },
    })

    // Core Engine Client
    this.coreClient = axios.create({
      baseURL: CORE_API_URL,
      timeout: 30000,
      headers: {
        'Content-Type': 'application/json',
      },
    })

    // Network Guard Client
    this.networkClient = axios.create({
      baseURL: NETWORK_API_URL,
      timeout: 30000,
      headers: {
        'Content-Type': 'application/json',
      },
    })

    // Setup interceptors
    this.setupInterceptors()
  }

  private setupInterceptors() {
    // Request interceptor - add auth token
    const requestInterceptor = (config: AxiosRequestConfig) => {
      const token = localStorage.getItem('auth_token')
      if (token && config.headers) {
        config.headers.Authorization = `Bearer ${token}`
      }
      return config
    }

    this.aiClient.interceptors.request.use(requestInterceptor as any)
    this.coreClient.interceptors.request.use(requestInterceptor as any)
    this.networkClient.interceptors.request.use(requestInterceptor as any)

    // Response interceptor - handle errors
    const responseErrorInterceptor = async (error: AxiosError) => {
      if (error.response?.status === 401) {
        // Unauthorized - clear token and redirect to login
        localStorage.removeItem('auth_token')
        window.location.href = '/login'
      }
      return Promise.reject(error)
    }

    this.aiClient.interceptors.response.use(
      (response) => response,
      responseErrorInterceptor
    )
    this.coreClient.interceptors.response.use(
      (response) => response,
      responseErrorInterceptor
    )
    this.networkClient.interceptors.response.use(
      (response) => response,
      responseErrorInterceptor
    )
  }

  // Scanner API
  async getScanStats(): Promise<ScanStats> {
    const response = await this.coreClient.get('/api/v1/scan/stats')
    return response.data
  }

  async startScan(path: string, scanType: 'quick' | 'full' | 'custom' = 'quick'): Promise<ScanResult> {
    const response = await this.coreClient.post('/api/v1/scan/start', {
      path,
      scanType,
    })
    return response.data
  }

  async getScanResults(): Promise<ScanResult[]> {
    const response = await this.coreClient.get('/api/v1/scan/results')
    return response.data
  }

  async pauseScan(): Promise<void> {
    await this.coreClient.post('/api/v1/scan/pause')
  }

  async resumeScan(): Promise<void> {
    await this.coreClient.post('/api/v1/scan/resume')
  }

  async stopScan(): Promise<void> {
    await this.coreClient.post('/api/v1/scan/stop')
  }

  // Threats API
  async getThreats(): Promise<Threat[]> {
    const response = await this.coreClient.get('/api/v1/threats')
    return response.data
  }

  async getThreatDetails(id: string): Promise<Threat> {
    const response = await this.coreClient.get(`/api/v1/threats/${id}`)
    return response.data
  }

  async quarantineThreat(id: string): Promise<void> {
    await this.coreClient.post(`/api/v1/threats/${id}/quarantine`)
  }

  async removeThreat(id: string): Promise<void> {
    await this.coreClient.delete(`/api/v1/threats/${id}`)
  }

  async restoreThreat(id: string): Promise<void> {
    await this.coreClient.post(`/api/v1/threats/${id}/restore`)
  }

  // Firewall API
  async getFirewallRules(): Promise<FirewallRule[]> {
    const response = await this.networkClient.get('/api/v1/firewall/rules')
    return response.data.data || response.data
  }

  async addFirewallRule(rule: Omit<FirewallRule, 'id' | 'created'>): Promise<FirewallRule> {
    const response = await this.networkClient.post('/api/v1/firewall/rules', rule)
    return response.data.data || response.data
  }

  async updateFirewallRule(id: string, rule: Partial<FirewallRule>): Promise<FirewallRule> {
    const response = await this.networkClient.put(`/api/v1/firewall/rules/${id}`, rule)
    return response.data.data || response.data
  }

  async deleteFirewallRule(id: string): Promise<void> {
    await this.networkClient.delete(`/api/v1/firewall/rules/${id}`)
  }

  async getFirewallStats(): Promise<any> {
    const response = await this.networkClient.get('/api/v1/firewall/stats')
    return response.data
  }

  // Network API
  async getNetworkConnections(): Promise<any[]> {
    const response = await this.networkClient.get('/api/v1/network/connections')
    return response.data.data || response.data
  }

  async getNetworkStats(): Promise<any> {
    const response = await this.networkClient.get('/api/v1/network/stats')
    return response.data.data || response.data
  }

  async scanNetwork(): Promise<any> {
    const response = await this.networkClient.post('/api/v1/network/scan')
    return response.data
  }

  // VPN API
  async getVPNStatus(): Promise<VPNConnection> {
    const response = await this.networkClient.get('/api/v1/vpn/status')
    return response.data.data || response.data
  }

  async connectVPN(server: string): Promise<VPNConnection> {
    const response = await this.networkClient.post('/api/v1/vpn/connect', { server })
    return response.data.data || response.data
  }

  async disconnectVPN(): Promise<void> {
    await this.networkClient.post('/api/v1/vpn/disconnect')
  }

  async getVPNConnections(): Promise<VPNConnection[]> {
    const response = await this.networkClient.get('/api/v1/vpn/connections')
    return response.data.data || response.data
  }

  // Settings API
  async getSettings(): Promise<any> {
    const response = await this.coreClient.get('/api/v1/settings')
    return response.data
  }

  async updateSettings(settings: any): Promise<void> {
    await this.coreClient.put('/api/v1/settings', settings)
  }

  // Devices API
  async getDevices(): Promise<Device[]> {
    const response = await this.coreClient.get('/api/v1/devices')
    return response.data
  }

  async getDeviceDetails(id: string): Promise<Device> {
    const response = await this.coreClient.get(`/api/v1/devices/${id}`)
    return response.data
  }

  async removeDevice(id: string): Promise<void> {
    await this.coreClient.delete(`/api/v1/devices/${id}`)
  }

  // Crypto Vault API
  async getPasswords(): Promise<PasswordEntry[]> {
    const response = await this.coreClient.get('/api/v1/vault/passwords')
    return response.data
  }

  async addPassword(password: Omit<PasswordEntry, 'id' | 'created' | 'lastModified'>): Promise<PasswordEntry> {
    const response = await this.coreClient.post('/api/v1/vault/passwords', password)
    return response.data
  }

  async updatePassword(id: string, password: Partial<PasswordEntry>): Promise<PasswordEntry> {
    const response = await this.coreClient.put(`/api/v1/vault/passwords/${id}`, password)
    return response.data
  }

  async deletePassword(id: string): Promise<void> {
    await this.coreClient.delete(`/api/v1/vault/passwords/${id}`)
  }

  async getEncryptedFiles(): Promise<any[]> {
    const response = await this.coreClient.get('/api/v1/vault/files')
    return response.data
  }

  async encryptFile(file: File): Promise<any> {
    const formData = new FormData()
    formData.append('file', file)
    
    const response = await this.coreClient.post('/api/v1/vault/files/encrypt', formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
      },
    })
    return response.data
  }

  // Health Check
  async healthCheck(): Promise<{ status: string; timestamp: string }> {
    const response = await this.coreClient.get('/api/v1/health')
    return response.data
  }

  // AI Engine specific
  async analyzeFile(filePath: string): Promise<any> {
    const response = await this.aiClient.post('/api/v1/analyze', {
      file_path: filePath,
    })
    return response.data
  }

  async getThreatIntelligence(hash: string): Promise<any> {
    const response = await this.aiClient.get(`/api/v1/threat-report/${hash}`)
    return response.data
  }
}

// Export singleton instance
export const apiClient = new ApiClient()

// Export types
export type { ApiClient }