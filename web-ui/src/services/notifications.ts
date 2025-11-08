// Notification Service
import toast from 'react-hot-toast'

export type NotificationType = 'success' | 'error' | 'warning' | 'info'

class NotificationService {
  // Show success notification
  success(message: string, duration: number = 4000): void {
    toast.success(message, {
      duration,
      position: 'top-right',
    })
  }

  // Show error notification
  error(message: string, duration: number = 5000): void {
    toast.error(message, {
      duration,
      position: 'top-right',
    })
  }

  // Show warning notification
  warning(message: string, duration: number = 4000): void {
    toast(message, {
      duration,
      position: 'top-right',
      icon: '⚠️',
      style: {
        background: '#ff9800',
        color: '#fff',
      },
    })
  }

  // Show info notification
  info(message: string, duration: number = 4000): void {
    toast(message, {
      duration,
      position: 'top-right',
      icon: 'ℹ️',
      style: {
        background: '#2196f3',
        color: '#fff',
      },
    })
  }

  // Show loading notification
  loading(message: string): string {
    return toast.loading(message, {
      position: 'top-right',
    })
  }

  // Dismiss notification
  dismiss(toastId: string): void {
    toast.dismiss(toastId)
  }

  // Dismiss all notifications
  dismissAll(): void {
    toast.dismiss()
  }

  // Show custom notification
  custom(message: string, options?: any): void {
    toast(message, {
      position: 'top-right',
      ...options,
    })
  }

  // Show promise notification (for async operations)
  promise<T>(
    promise: Promise<T>,
    messages: {
      loading: string
      success: string
      error: string
    }
  ): Promise<T> {
    return toast.promise(
      promise,
      {
        loading: messages.loading,
        success: messages.success,
        error: messages.error,
      },
      {
        position: 'top-right',
      }
    )
  }

  // Show threat detected notification
  threatDetected(threatName: string, filePath: string): void {
    toast.error(
      `Threat Detected: ${threatName}\nFile: ${filePath}`,
      {
        duration: 10000,
        position: 'top-right',
        icon: '🛡️',
      }
    )
  }

  // Show scan complete notification
  scanComplete(filesScanned: number, threatsFound: number): void {
    if (threatsFound > 0) {
      this.warning(
        `Scan complete: ${filesScanned} files scanned, ${threatsFound} threats found`,
        6000
      )
    } else {
      this.success(
        `Scan complete: ${filesScanned} files scanned, no threats found`,
        4000
      )
    }
  }

  // Show update available notification
  updateAvailable(version: string): void {
    toast(
      `Update available: v${version}`,
      {
        duration: 8000,
        position: 'top-right',
        icon: '🔄',
        style: {
          background: '#4caf50',
          color: '#fff',
        },
      }
    )
  }

  // Show VPN connected notification
  vpnConnected(server: string): void {
    this.success(`Connected to VPN: ${server}`)
  }

  // Show VPN disconnected notification
  vpnDisconnected(): void {
    this.info('Disconnected from VPN')
  }
}

// Export singleton instance
export const notificationService = new NotificationService()

// Export types
export type { NotificationService }