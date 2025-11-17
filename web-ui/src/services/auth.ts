// Authentication Service
import { apiClient } from './api'

export interface AuthResponse {
  success: boolean
  token?: string
  user?: User
  message?: string
}

export interface User {
  id: string
  username: string
  email: string
  role: 'admin' | 'user'
  createdAt: string
}

class AuthService {
  private readonly TOKEN_KEY = 'auth_token'
  private readonly USER_KEY = 'user_data'

  // Login
  async login(username: string, password: string): Promise<AuthResponse> {
    try {
      // In production, this would call the actual API
      // For now, simulate login
      if (username === 'admin' && password === 'admin123') {
        const token = this.generateToken()
        const user: User = {
          id: '1',
          username: 'admin',
          email: 'admin@ghostantivirus.com',
          role: 'admin',
          createdAt: new Date().toISOString(),
        }

        this.setToken(token)
        this.setUser(user)

        return {
          success: true,
          token,
          user,
        }
      }

      return {
        success: false,
        message: 'Invalid credentials',
      }
    } catch (error) {
      return {
        success: false,
        message: 'Login failed',
      }
    }
  }

  // Logout
  async logout(): Promise<void> {
    this.clearToken()
    this.clearUser()
  }

  // Refresh token
  async refreshToken(): Promise<string | null> {
    try {
      const currentToken = this.getToken()
      if (!currentToken) {
        return null
      }

      // In production, call refresh endpoint
      // For now, return current token
      return currentToken
    } catch (error) {
      this.clearToken()
      return null
    }
  }

  // Check if user is authenticated
  isAuthenticated(): boolean {
    const token = this.getToken()
    return token !== null && token !== ''
  }

  // Get current token
  getToken(): string | null {
    return localStorage.getItem(this.TOKEN_KEY)
  }

  // Set token
  setToken(token: string): void {
    localStorage.setItem(this.TOKEN_KEY, token)
  }

  // Clear token
  clearToken(): void {
    localStorage.removeItem(this.TOKEN_KEY)
  }

  // Get current user
  getUser(): User | null {
    const userData = localStorage.getItem(this.USER_KEY)
    if (!userData) {
      return null
    }

    try {
      return JSON.parse(userData)
    } catch {
      return null
    }
  }

  // Set user data
  setUser(user: User): void {
    localStorage.setItem(this.USER_KEY, JSON.stringify(user))
  }

  // Clear user data
  clearUser(): void {
    localStorage.removeItem(this.USER_KEY)
  }

  // Generate token (for demo purposes)
  private generateToken(): string {
    return `token_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`
  }

  // Validate token format
  isValidToken(token: string): boolean {
    return token.startsWith('token_') && token.length > 20
  }

  // Check if token is expired (simplified)
  isTokenExpired(token: string): boolean {
    // In production, decode JWT and check expiration
    // For now, always return false
    return false
  }

  // Get user role
  getUserRole(): 'admin' | 'user' | null {
    const user = this.getUser()
    return user?.role || null
  }

  // Check if user has admin role
  isAdmin(): boolean {
    return this.getUserRole() === 'admin'
  }
}

// Export singleton instance
export const authService = new AuthService()

// Export types
export type { AuthService }