// WebSocket Service - Real-time updates
import { io, Socket } from 'socket.io-client'

const WS_URL = import.meta.env.VITE_WS_URL || 'http://localhost:8000'

export interface WebSocketMessage {
  type: string
  data: any
  timestamp: string
}

export type EventCallback = (data: any) => void

class WebSocketService {
  private socket: Socket | null = null
  private reconnectAttempts = 0
  private maxReconnectAttempts = 5
  private reconnectDelay = 1000
  private eventHandlers: Map<string, Set<EventCallback>> = new Map()
  private connected = false

  // Connect to WebSocket server
  connect(url: string = WS_URL): void {
    if (this.socket?.connected) {
      console.log('WebSocket already connected')
      return
    }

    console.log('Connecting to WebSocket:', url)

    this.socket = io(url, {
      transports: ['websocket', 'polling'],
      reconnection: true,
      reconnectionAttempts: this.maxReconnectAttempts,
      reconnectionDelay: this.reconnectDelay,
      timeout: 10000,
    })

    this.setupEventListeners()
  }

  // Disconnect from WebSocket server
  disconnect(): void {
    if (this.socket) {
      console.log('Disconnecting from WebSocket')
      this.socket.disconnect()
      this.socket = null
      this.connected = false
    }
  }

  // Send message to server
  send(event: string, data: any): void {
    if (!this.socket || !this.connected) {
      console.warn('WebSocket not connected, cannot send message')
      return
    }

    this.socket.emit(event, data)
  }

  // Subscribe to event
  on(event: string, callback: EventCallback): void {
    if (!this.eventHandlers.has(event)) {
      this.eventHandlers.set(event, new Set())
    }

    this.eventHandlers.get(event)!.add(callback)

    // Also register with socket if connected
    if (this.socket) {
      this.socket.on(event, callback)
    }
  }

  // Unsubscribe from event
  off(event: string, callback: EventCallback): void {
    const handlers = this.eventHandlers.get(event)
    if (handlers) {
      handlers.delete(callback)
    }

    if (this.socket) {
      this.socket.off(event, callback)
    }
  }

  // Setup internal event listeners
  private setupEventListeners(): void {
    if (!this.socket) return

    this.socket.on('connect', () => {
      console.log('WebSocket connected')
      this.connected = true
      this.reconnectAttempts = 0
      this.notifyHandlers('connection', { status: 'connected' })
    })

    this.socket.on('disconnect', (reason) => {
      console.log('WebSocket disconnected:', reason)
      this.connected = false
      this.notifyHandlers('connection', { status: 'disconnected', reason })
    })

    this.socket.on('connect_error', (error) => {
      console.error('WebSocket connection error:', error)
      this.reconnectAttempts++
      
      if (this.reconnectAttempts >= this.maxReconnectAttempts) {
        console.error('Max reconnection attempts reached')
        this.notifyHandlers('connection', { 
          status: 'failed', 
          error: 'Max reconnection attempts reached' 
        })
      }
    })

    this.socket.on('reconnect', (attemptNumber) => {
      console.log('WebSocket reconnected after', attemptNumber, 'attempts')
      this.reconnectAttempts = 0
      this.notifyHandlers('connection', { status: 'reconnected' })
    })

    // Register all custom event handlers
    this.eventHandlers.forEach((callbacks, event) => {
      callbacks.forEach((callback) => {
        this.socket!.on(event, callback)
      })
    })
  }

  // Notify all handlers for an event
  private notifyHandlers(event: string, data: any): void {
    const handlers = this.eventHandlers.get(event)
    if (handlers) {
      handlers.forEach((callback) => callback(data))
    }
  }

  // Reconnect manually
  reconnect(): void {
    console.log('Manual reconnection triggered')
    this.disconnect()
    setTimeout(() => this.connect(), 1000)
  }

  // Check if connected
  isConnected(): boolean {
    return this.connected && this.socket?.connected === true
  }

  // Get connection status
  getStatus(): 'connected' | 'disconnected' | 'connecting' {
    if (!this.socket) return 'disconnected'
    if (this.socket.connected) return 'connected'
    if (this.socket.connecting) return 'connecting'
    return 'disconnected'
  }

  // Send heartbeat (keep-alive)
  sendHeartbeat(): void {
    if (this.isConnected()) {
      this.send('heartbeat', { timestamp: Date.now() })
    }
  }

  // Start heartbeat interval
  startHeartbeat(interval: number = 30000): void {
    setInterval(() => {
      this.sendHeartbeat()
    }, interval)
  }
}

// Export singleton instance
export const wsService = new WebSocketService()

// Auto-connect on import
wsService.connect()

// Start heartbeat
wsService.startHeartbeat()

// Export types
export type { WebSocketService }