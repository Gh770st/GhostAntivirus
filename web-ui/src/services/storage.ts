// Local Storage Service
class StorageService {
  // Get item from localStorage
  get<T>(key: string): T | null {
    try {
      const item = localStorage.getItem(key)
      if (!item) return null
      return JSON.parse(item) as T
    } catch (error) {
      console.error('Failed to get item from storage:', error)
      return null
    }
  }

  // Set item in localStorage
  set<T>(key: string, value: T): void {
    try {
      localStorage.setItem(key, JSON.stringify(value))
    } catch (error) {
      console.error('Failed to set item in storage:', error)
    }
  }

  // Remove item from localStorage
  remove(key: string): void {
    try {
      localStorage.removeItem(key)
    } catch (error) {
      console.error('Failed to remove item from storage:', error)
    }
  }

  // Clear all items
  clear(): void {
    try {
      localStorage.clear()
    } catch (error) {
      console.error('Failed to clear storage:', error)
    }
  }

  // Check if key exists
  has(key: string): boolean {
    return localStorage.getItem(key) !== null
  }

  // Get all keys
  keys(): string[] {
    return Object.keys(localStorage)
  }

  // Get storage size (approximate)
  getSize(): number {
    let size = 0
    for (const key in localStorage) {
      if (localStorage.hasOwnProperty(key)) {
        size += localStorage[key].length + key.length
      }
    }
    return size
  }
}

// Export singleton instance
export const storageService = new StorageService()

// Export types
export type { StorageService }