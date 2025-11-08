import React, { useState, useEffect } from 'react'
import {
  Box,
  Card,
  CardContent,
  Typography,
  Button,
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
  Chip,
  LinearProgress,
  Alert,
  Grid,
  Paper,
  Divider,
  IconButton,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
} from '@mui/material'
import {
  VpnLock as VpnIcon,
  Public as ServerIcon,
  Speed as SpeedIcon,
  Timer as TimerIcon,
  DataUsage as DataIcon,
  CheckCircle as ConnectedIcon,
  Error as DisconnectedIcon,
  Refresh as RefreshIcon,
  Security as SecurityIcon,
} from '@mui/icons-material'

interface VPNConnection {
  id: string
  server: string
  location: string
  country: string
  flag: string
  status: 'connected' | 'disconnected' | 'connecting'
  protocol: string
  ip: string
  connectedAt?: string
  latency: number
  load: number
}

interface VPNStats {
  status: 'connected' | 'disconnected' | 'connecting'
  server?: string
  location?: string
  ip?: string
  protocol?: string
  connectedAt?: string
  bytesIn: number
  bytesOut: number
  uptime: string
  killSwitch: boolean
}

const VPN: React.FC = () => {
  const [connectionStatus, setConnectionStatus] = useState<'disconnected' | 'connecting' | 'connected'>('disconnected')
  const [selectedServer, setSelectedServer] = useState('')
  const [stats, setStats] = useState<VPNStats>({
    status: 'disconnected',
    bytesIn: 0,
    bytesOut: 0,
    uptime: '0h 0m',
    killSwitch: true,
  })
  const [servers, setServers] = useState<VPNConnection[]>([])

  // Mock VPN servers
  const mockServers: VPNConnection[] = [
    {
      id: '1',
      server: 'us-west.vpn.ghostantivirus.com',
      location: 'US West',
      country: 'United States',
      flag: '🇺🇸',
      status: 'disconnected',
      protocol: 'WireGuard',
      ip: '192.168.100.1',
      latency: 45,
      load: 65,
    },
    {
      id: '2',
      server: 'uk-london.vpn.ghostantivirus.com',
      location: 'UK London',
      country: 'United Kingdom',
      flag: '🇬🇧',
      status: 'disconnected',
      protocol: 'WireGuard',
      ip: '192.168.100.2',
      latency: 78,
      load: 45,
    },
    {
      id: '3',
      server: 'de-berlin.vpn.ghostantivirus.com',
      location: 'Germany Berlin',
      country: 'Germany',
      flag: '🇩🇪',
      status: 'disconnected',
      protocol: 'OpenVPN',
      ip: '192.168.100.3',
      latency: 62,
      load: 38,
    },
    {
      id: '4',
      server: 'jp-tokyo.vpn.ghostantivirus.com',
      location: 'Japan Tokyo',
      country: 'Japan',
      flag: '🇯🇵',
      status: 'disconnected',
      protocol: 'WireGuard',
      ip: '192.168.100.4',
      latency: 156,
      load: 72,
    },
    {
      id: '5',
      server: 'au-sydney.vpn.ghostantivirus.com',
      location: 'Australia Sydney',
      country: 'Australia',
      flag: '🇦🇺',
      status: 'disconnected',
      protocol: 'OpenVPN',
      ip: '192.168.100.5',
      latency: 234,
      load: 28,
    },
  ]

  useEffect(() => {
    setServers(mockServers)
    setSelectedServer(mockServers[0].id)
  }, [])

  const handleConnect = () => {
    if (connectionStatus === 'connected') {
      // Disconnect
      setConnectionStatus('disconnected')
      setStats({
        status: 'disconnected',
        bytesIn: 0,
        bytesOut: 0,
        uptime: '0h 0m',
        killSwitch: true,
      })
      setServers(servers.map(s => ({ ...s, status: 'disconnected' })))
    } else {
      // Connect
      setConnectionStatus('connecting')
      
      setTimeout(() => {
        const server = servers.find(s => s.id === selectedServer)
        setConnectionStatus('connected')
        setStats({
          status: 'connected',
          server: server?.server,
          location: server?.location,
          ip: server?.ip,
          protocol: server?.protocol,
          connectedAt: new Date().toLocaleString(),
          bytesIn: Math.floor(Math.random() * 1000000),
          bytesOut: Math.floor(Math.random() * 500000),
          uptime: '0h 5m',
          killSwitch: true,
        })
        setServers(servers.map(s => ({
          ...s,
          status: s.id === selectedServer ? 'connected' : 'disconnected'
        })))
      }, 2000)
    }
  }

  const formatBytes = (bytes: number) => {
    if (bytes === 0) return '0 B'
    const k = 1024
    const sizes = ['B', 'KB', 'MB', 'GB']
    const i = Math.floor(Math.log(bytes) / Math.log(k))
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
  }

  const getLatencyColor = (latency: number) => {
    if (latency < 50) return 'success'
    if (latency < 100) return 'warning'
    return 'error'
  }

  const getLoadColor = (load: number) => {
    if (load < 50) return 'success'
    if (load < 75) return 'warning'
    return 'error'
  }

  return (
    <Box>
      <Typography variant="h4" gutterBottom sx={{ mb: 3 }}>
        VPN Service
      </Typography>

      {/* Connection Status */}
      <Grid container spacing={3} sx={{ mb: 3 }}>
        <Grid item xs={12} md={8}>
          <Card>
            <CardContent>
              <Box display="flex" justifyContent="space-between" alignItems="center" mb={2}>
                <Typography variant="h6">VPN Connection</Typography>
                <IconButton onClick={() => window.location.reload()}>
                  <RefreshIcon />
                </IconButton>
              </Box>
              
              {connectionStatus === 'connecting' && (
                <Alert severity="info" sx={{ mb: 2 }}>
                  Connecting to VPN server...
                  <LinearProgress sx={{ mt: 1 }} />
                </Alert>
              )}
              
              {connectionStatus === 'connected' && (
                <Alert severity="success" sx={{ mb: 2 }}>
                  You are connected to VPN. Your traffic is encrypted and secure.
                </Alert>
              )}
              
              {connectionStatus === 'disconnected' && (
                <Alert severity="warning" sx={{ mb: 2 }}>
                  VPN is disconnected. Your traffic is not encrypted.
                </Alert>
              )}

              <Box sx={{ display: 'flex', gap: 2, mb: 3 }}>
                <FormControl sx={{ minWidth: 300 }}>
                  <InputLabel>Select Server</InputLabel>
                  <Select
                    value={selectedServer}
                    label="Select Server"
                    onChange={(e) => setSelectedServer(e.target.value)}
                    disabled={connectionStatus !== 'disconnected'}
                  >
                    {servers.map((server) => (
                      <MenuItem key={server.id} value={server.id}>
                        <Box display="flex" alignItems="center" gap={1}>
                          <span>{server.flag}</span>
                          <span>{server.location}</span>
                          <Chip
                            label={`${server.latency}ms`}
                            size="small"
                            color={getLatencyColor(server.latency) as any}
                          />
                        </Box>
                      </MenuItem>
                    ))}
                  </Select>
                </FormControl>
                
                <Button
                  variant="contained"
                  startIcon={connectionStatus === 'connected' ? <DisconnectedIcon /> : <VpnIcon />}
                  onClick={handleConnect}
                  color={connectionStatus === 'connected' ? 'error' : 'primary'}
                  disabled={connectionStatus === 'connecting'}
                  sx={{ minWidth: 120 }}
                >
                  {connectionStatus === 'connecting' ? 'Connecting...' : 
                   connectionStatus === 'connected' ? 'Disconnect' : 'Connect'}
                </Button>
              </Box>

              <Box sx={{ display: 'flex', gap: 2, flexWrap: 'wrap' }}>
                <Chip
                  icon={<SecurityIcon />}
                  label={`Kill Switch: ${stats.killSwitch ? 'Active' : 'Inactive'}`}
                  color={stats.killSwitch ? 'success' : 'default'}
                  variant="outlined"
                />
                {stats.protocol && (
                  <Chip
                    label={`Protocol: ${stats.protocol}`}
                    variant="outlined"
                  />
                )}
                {stats.ip && (
                  <Chip
                    label={`IP: ${stats.ip}`}
                    variant="outlined"
                  />
                )}
              </Box>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} md={4}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Connection Statistics
              </Typography>
              <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
                <Box display="flex" justifyContent="space-between">
                  <Typography variant="body2">Status</Typography>
                  <Chip
                    label={connectionStatus.toUpperCase()}
                    color={
                      connectionStatus === 'connected' ? 'success' :
                      connectionStatus === 'connecting' ? 'warning' : 'default'
                    }
                    size="small"
                  />
                </Box>
                {stats.server && (
                  <Box display="flex" justifyContent="space-between">
                    <Typography variant="body2">Server</Typography>
                    <Typography variant="body2" color="text.secondary">
                      {stats.location}
                    </Typography>
                  </Box>
                )}
                {stats.connectedAt && (
                  <Box display="flex" justifyContent="space-between">
                    <Typography variant="body2">Connected</Typography>
                    <Typography variant="body2" color="text.secondary">
                      {stats.connectedAt}
                    </Typography>
                  </Box>
                )}
                <Box display="flex" justifyContent="space-between">
                  <Typography variant="body2">Uptime</Typography>
                  <Typography variant="body2" color="text.secondary">
                    {stats.uptime}
                  </Typography>
                </Box>
                <Box display="flex" justifyContent="space-between">
                  <Typography variant="body2">Data In</Typography>
                  <Typography variant="body2" color="success">
                    {formatBytes(stats.bytesIn)}
                  </Typography>
                </Box>
                <Box display="flex" justifyContent="space-between">
                  <Typography variant="body2">Data Out</Typography>
                  <Typography variant="body2" color="warning">
                    {formatBytes(stats.bytesOut)}
                  </Typography>
                </Box>
              </Box>
            </CardContent>
          </Card>
        </Grid>
      </Grid>

      {/* Server List */}
      <Paper sx={{ maxHeight: 400, overflow: 'auto' }}>
        <Box sx={{ p: 2, borderBottom: 1, borderColor: 'divider' }}>
          <Typography variant="h6">Available Servers</Typography>
        </Box>
        <List>
          {servers.map((server) => (
            <React.Fragment key={server.id}>
              <ListItem>
                <ListItemIcon>
                  {server.status === 'connected' ? (
                    <ConnectedIcon color="success" />
                  ) : (
                    <DisconnectedIcon color="disabled" />
                  )}
                </ListItemIcon>
                <ListItemText
                  primary={
                    <Box display="flex" alignItems="center" gap={1}>
                      <Typography variant="body1" sx={{ fontWeight: 'medium' }}>
                        {server.flag} {server.location}
                      </Typography>
                      {server.status === 'connected' && (
                        <Chip
                          label="CONNECTED"
                          color="success"
                          size="small"
                        />
                      )}
                    </Box>
                  }
                  secondary={
                    <Box>
                      <Typography variant="body2" color="text.secondary">
                        {server.server}
                      </Typography>
                      <Box sx={{ display: 'flex', gap: 1, mt: 0.5 }}>
                        <Chip
                          icon={<SpeedIcon />}
                          label={`${server.latency}ms`}
                          size="small"
                          color={getLatencyColor(server.latency) as any}
                        />
                        <Chip
                          icon={<DataIcon />}
                          label={`${server.load}% load`}
                          size="small"
                          color={getLoadColor(server.load) as any}
                        />
                        <Chip
                          label={server.protocol}
                          size="small"
                          variant="outlined"
                        />
                      </Box>
                    </Box>
                  }
                />
              </ListItem>
              <Divider />
            </React.Fragment>
          ))}
        </List>
      </Paper>
    </Box>
  )
}

export default VPN