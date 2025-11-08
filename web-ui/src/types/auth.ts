// Authentication related types
export interface User {
  id: string
  username: string
  email: string
  role: 'admin' | 'user'
  createdAt: string
}

export interface AuthResponse {
  success: boolean
  token?: string
  user?: User
  message?: string
}

export interface LoginCredentials {
  username: string
  password: string
}

export interface RegisterData {
  username: string
  email: string
  password: string
  confirmPassword: string
}

export type UserRole = 'admin' | 'user'