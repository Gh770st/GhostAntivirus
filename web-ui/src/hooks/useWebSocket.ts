// Custom hook for WebSocket events
import { useEffect, useCallback } from 'react'
import { wsService, EventCallback } from '@/services/websocket'

export function useWebSocket(event: string, callback: EventCallback) {
  useEffect(() => {
    // Subscribe to event
    wsService.on(event, callback)

    // Cleanup on unmount
    return () => {
      wsService.off(event, callback)
    }
  }, [event, callback])
}

export function useWebSocketConnection() {
  const [isConnected, setIsConnected] = React.useState(wsService.isConnected())

  useEffect(() => {
    const handleConnection = (data: any) => {
      setIsConnected(data.status === 'connected' || data.status === 'reconnected')
    }

    wsService.on('connection', handleConnection)

    return () => {
      wsService.off('connection', handleConnection)
    }
  }, [])

  const reconnect = useCallback(() => {
    wsService.reconnect()
  }, [])

  return {
    isConnected,
    reconnect,
    status: wsService.getStatus(),
  }
}

// Hook for real-time scan updates
export function useScanUpdates(onUpdate: (data: any) => void) {
  useWebSocket('scan:update', onUpdate)
}

// Hook for real-time threat alerts
export function useThreatAlerts(onAlert: (data: any) => void) {
  useWebSocket('threat:detected', onAlert)
}

// Hook for real-time network updates
export function useNetworkUpdates(onUpdate: (data: any) => void) {
  useWebSocket('network:update', onUpdate)
}

// Hook for real-time VPN status
export function useVPNUpdates(onUpdate: (data: any) => void) {
  useWebSocket('vpn:status', onUpdate)
}