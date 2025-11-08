// API Configuration
export const API_CONFIG = {
  coreEngine: {
    baseURL: import.meta.env.VITE_CORE_ENGINE_URL || 'http://localhost:8080',
    timeout: 30000,
  },
  aiEngine: {
    baseURL: import.meta.env.VITE_AI_ENGINE_URL || 'http://localhost:8000',
    timeout: 30000,
  },
  networkGuard: {
    baseURL: import.meta.env.VITE_NETWORK_GUARD_URL || 'http://localhost:9000',
    timeout: 30000,
  },
  websocket: {
    url: import.meta.env.VITE_WS_URL || 'ws://localhost:8080/ws',
    reconnectInterval: 5000,
    maxReconnectAttempts: 5,
  },
  features: {
    enableAI: import.meta.env.VITE_ENABLE_AI === 'true',
    enableNetworkGuard: import.meta.env.VITE_ENABLE_NETWORK_GUARD === 'true',
    enableVPN: import.meta.env.VITE_ENABLE_VPN === 'true',
    enableCryptoVault: import.meta.env.VITE_ENABLE_CRYPTO_VAULT === 'true',
  },
  refresh: {
    scanStats: parseInt(import.meta.env.VITE_SCAN_STATS_REFRESH || '5000'),
    threats: parseInt(import.meta.env.VITE_THREAT_REFRESH || '10000'),
    network: parseInt(import.meta.env.VITE_NETWORK_REFRESH || '3000'),
    systemStats: parseInt(import.meta.env.VITE_SYSTEM_STATS_REFRESH || '2000'),
  },
  debug: import.meta.env.VITE_DEBUG === 'true',
  logLevel: import.meta.env.VITE_LOG_LEVEL || 'info',
};

export default API_CONFIG;