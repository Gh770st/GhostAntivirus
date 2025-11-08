// Custom hook for API calls with loading and error states
import { useState, useCallback } from 'react'
import { apiClient } from '@/services/api'
import { notificationService } from '@/services/notifications'

interface UseApiState<T> {
  data: T | null
  loading: boolean
  error: string | null
}

interface UseApiReturn<T> extends UseApiState<T> {
  execute: (...args: any[]) => Promise<T | null>
  reset: () => void
}

export function useApi<T>(
  apiFunction: (...args: any[]) => Promise<T>,
  options?: {
    onSuccess?: (data: T) => void
    onError?: (error: string) => void
    showSuccessNotification?: boolean
    showErrorNotification?: boolean
  }
): UseApiReturn<T> {
  const [state, setState] = useState<UseApiState<T>>({
    data: null,
    loading: false,
    error: null,
  })

  const execute = useCallback(
    async (...args: any[]): Promise<T | null> => {
      setState({ data: null, loading: true, error: null })

      try {
        const result = await apiFunction(...args)
        setState({ data: result, loading: false, error: null })

        if (options?.onSuccess) {
          options.onSuccess(result)
        }

        if (options?.showSuccessNotification) {
          notificationService.success('Operation completed successfully')
        }

        return result
      } catch (error: any) {
        const errorMessage = error.response?.data?.message || error.message || 'An error occurred'
        setState({ data: null, loading: false, error: errorMessage })

        if (options?.onError) {
          options.onError(errorMessage)
        }

        if (options?.showErrorNotification !== false) {
          notificationService.error(errorMessage)
        }

        return null
      }
    },
    [apiFunction, options]
  )

  const reset = useCallback(() => {
    setState({ data: null, loading: false, error: null })
  }, [])

  return {
    ...state,
    execute,
    reset,
  }
}

// Specific hooks for common operations
export function useScanStats() {
  return useApi(() => apiClient.getScanStats())
}

export function useStartScan() {
  return useApi(
    (path: string, scanType?: 'quick' | 'full' | 'custom') => 
      apiClient.startScan(path, scanType),
    {
      showSuccessNotification: true,
      showErrorNotification: true,
    }
  )
}

export function useThreats() {
  return useApi(() => apiClient.getThreats())
}

export function useFirewallRules() {
  return useApi(() => apiClient.getFirewallRules())
}

export function useVPNStatus() {
  return useApi(() => apiClient.getVPNStatus())
}

export function useDevices() {
  return useApi(() => apiClient.getDevices())
}

export function usePasswords() {
  return useApi(() => apiClient.getPasswords())
}