import React, { useState, useEffect } from 'react'
import {
  Box,
  Card,
  CardContent,
  Typography,
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
  Chip,
  Button,
  Grid,
  Paper,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  IconButton,
  Alert,
  Divider,
  LinearProgress,
} from '@mui/material'
import {
  Computer as DesktopIcon,
  Smartphone as PhoneIcon,
  TabletMac as TabletIcon,
  Security as ProtectedIcon,
  Warning as VulnerableIcon,
  Error as ErrorIcon,
  CheckCircle as OnlineIcon,
  Schedule as OfflineIcon,
  Refresh as RefreshIcon,
  Delete as DeleteIcon,
  Info as InfoIcon,
} from '@mui/icons-material'

interface Device {
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
  lastScan: string
  location?: string
}

interface DeviceStats {
  totalDevices: number
  onlineDevices: number
  protectedDevices: number
  vulnerableDevices: number
  totalThreatsBlocked: number
}

const DeviceManagement: React.FC = () => {
  const [devices, setDevices] = useState<Device[]>([])
  const [stats, setStats] = useState<DeviceStats>({
    totalDevices: 0,
    onlineDevices: 0,
    protectedDevices: 0,
    vulnerableDevices: 0,
    totalThreatsBlocked: 0,
  })
  const [selectedDevice, setSelectedDevice] = useState<Device | null>(null)
  const [detailsDialogOpen, setDetailsDialogOpen] = useState(false)

  // Mock device data
  const mockDevices: Device[] = [
    {
      id: '1',
      name: 'John\'s Desktop',
      type: 'desktop',
      platform: 'windows',
      status: 'online',
      protection: 'protected',
      lastSeen: new Date().toISOString(),
      ipAddress: '192.168.1.100',
      version: '3.0.0',
      threatsBlocked: 15,
      lastScan: new Date(Date.now() - 3600000).toLocaleString(),
      location: 'Home Office',
    },
    {
      id: '2',
      name: 'Work Laptop',
      type: 'laptop',
      platform: 'macos',
      status: 'online',
      protection: 'protected',
      lastSeen: new Date(Date.now() - 300000).toISOString(),
      ipAddress: '192.168.1.101',
      version: '3.0.0',
      threatsBlocked: 8,
      lastScan: new Date(Date.now() - 7200000).toLocaleString(),
      location: 'Remote',
    },
    {
      id: '3',
      name: 'Sarah\'s Phone',
      type: 'mobile',
      platform: 'android',
      status: 'vulnerable',
      protection: 'partial',
      lastSeen: new Date(Date.now() - 86400000).toISOString(),
      ipAddress: '192.168.1.102',
      version: '2.9.5',
      threatsBlocked: 3,
      lastScan: new Date(Date.now() - 172800000).toLocaleString(),
      location: 'Mobile',
    },
    {
      id: '4',
      name: 'iPad Pro',
      type: 'tablet',
      platform: 'ios',
      status: 'offline',
      protection: 'unprotected',
      lastSeen: new Date(Date.now() - 259200000).toISOString(),
      ipAddress: '192.168.1.103',
      version: '2.8.0',
      threatsBlocked: 0,
      lastScan: new Date(Date.now() - 604800000).toLocaleString(),
      location: 'Unknown',
    },
    {
      id: '5',
      name: 'Media Server',
      type: 'desktop',
      platform: 'linux',
      status: 'online',
      protection: 'protected',
      lastSeen: new Date(Date.now() - 60000).toISOString(),
      ipAddress: '192.168.1.104',
      version: '3.0.0',
      threatsBlocked: 22,
      lastScan: new Date(Date.now() - 1800000).toLocaleString(),
      location: 'Server Room',
    },
  ]

  useEffect(() => {
    setDevices(mockDevices)
    
    const onlineCount = mockDevices.filter(d => d.status === 'online').length
    const protectedCount = mockDevices.filter(d => d.protection === 'protected').length
    const vulnerableCount = mockDevices.filter(d => d.status === 'vulnerable').length
    const totalThreats = mockDevices.reduce((sum, d) => sum + d.threatsBlocked, 0)
    
    setStats({
      totalDevices: mockDevices.length,
      onlineDevices: onlineCount,
      protectedDevices: protectedCount,
      vulnerableDevices: vulnerableCount,
      totalThreatsBlocked: totalThreats,
    })
  }, [])

  const handleDeviceDetails = (device: Device) => {
    setSelectedDevice(device)
    setDetailsDialogOpen(true)
  }

  const handleRemoveDevice = (deviceId: string) => {
    if (window.confirm('Are you sure you want to remove this device from management?')) {
      setDevices(devices.filter(d => d.id !== deviceId))
    }
  }

  const handleRefresh = () => {
    // Simulate refreshing device list
    window.location.reload()
  }

  const getDeviceIcon = (type: string) => {
    switch (type) {
      case 'desktop':
      case 'laptop':
        return <DesktopIcon />
      case 'mobile':
        return <PhoneIcon />
      case 'tablet':
        return <TabletIcon />
      default:
        return <DesktopIcon />
    }
  }

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'online':
        return <OnlineIcon color="success" />
      case 'offline':
        return <OfflineIcon color="disabled" />
      case 'vulnerable':
        return <ErrorIcon color="error" />
      default:
        return <OfflineIcon color="disabled" />
    }
  }

  const getProtectionIcon = (protection: string) => {
    switch (protection) {
      case 'protected':
        return <ProtectedIcon color="success" />
      case 'unprotected':
        return <VulnerableIcon color="error" />
      case 'partial':
        return <WarningIcon color="warning" />
      default:
        return <VulnerableIcon color="error" />
    }
  }

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'online':
        return 'success'
      case 'offline':
        return 'default'
      case 'vulnerable':
        return 'error'
      default:
        return 'default'
    }
  }

  const getProtectionColor = (protection: string) => {
    switch (protection) {
      case 'protected':
        return 'success'
      case 'unprotected':
        return 'error'
      case 'partial':
        return 'warning'
      default:
        return 'error'
    }
  }

  const getPlatformColor = (platform: string) => {
    const colors = {
      windows: '#0078d4',
      macos: '#000000',
      linux: '#fcc624',
      android: '#3ddc84',
      ios: '#000000',
    }
    return colors[platform as keyof typeof colors] || '#666666'
  }

  const formatLastSeen = (lastSeen: string) => {
    const date = new Date(lastSeen)
    const now = new Date()
    const diffMs = now.getTime() - date.getTime()
    const diffMins = Math.floor(diffMs / 60000)
    
    if (diffMins < 1) return 'Just now'
    if (diffMins < 60) return `${diffMins} minutes ago`
    if (diffMins < 1440) return `${Math.floor(diffMins / 60)} hours ago`
    return `${Math.floor(diffMins / 1440)} days ago`
  }

  return (
    <Box>
      <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
        <Typography variant="h4">Device Management</Typography>
        <IconButton onClick={handleRefresh}>
          <RefreshIcon />
        </IconButton>
      </Box>

      {/* Stats Cards */}
      <Grid container spacing={3} sx={{ mb: 3 }}>
        <Grid item xs={12} sm={6} md={3}>
          <Card>
            <CardContent>
              <Box display="flex" alignItems="center" gap={1}>
                <DesktopIcon color="primary" />
                <Box>
                  <Typography variant="h4">{stats.totalDevices}</Typography>
                  <Typography variant="body2" color="text.secondary">Total Devices</Typography>
                </Box>
              </Box>
            </CardContent>
          </Card>
        </Grid>
        <Grid item xs={12} sm={6} md={3}>
          <Card>
            <CardContent>
              <Box display="flex" alignItems="center" gap={1}>
                <OnlineIcon color="success" />
                <Box>
                  <Typography variant="h4">{stats.onlineDevices}</Typography>
                  <Typography variant="body2" color="text.secondary">Online</Typography>
                </Box>
              </Box>
            </CardContent>
          </Card>
        </Grid>
        <Grid item xs={12} sm={6} md={3}>
          <Card>
            <CardContent>
              <Box display="flex" alignItems="center" gap={1}>
                <ProtectedIcon color="success" />
                <Box>
                  <Typography variant="h4">{stats.protectedDevices}</Typography>
                  <Typography variant="body2" color="text.secondary">Protected</Typography>
                </Box>
              </Box>
            </CardContent>
          </Card>
        </Grid>
        <Grid item xs={12} sm={6} md={3}>
          <Card>
            <CardContent>
              <Box display="flex" alignItems="center" gap={1}>
                <ErrorIcon color="error" />
                <Box>
                  <Typography variant="h4">{stats.vulnerableDevices}</Typography>
                  <Typography variant="body2" color="text.secondary">Vulnerable</Typography>
                </Box>
              </Box>
            </CardContent>
          </Card>
        </Grid>
      </Grid>

      {/* Alerts */}
      {stats.vulnerableDevices > 0 && (
        <Alert severity="warning" sx={{ mb: 3 }}>
          {stats.vulnerableDevices} device(s) require attention. Some devices may be outdated or unprotected.
        </Alert>
      )}

      {/* Device List */}
      <Paper sx={{ maxHeight: 500, overflow: 'auto' }}>
        <List>
          {devices.map((device) => (
            <React.Fragment key={device.id}>
              <ListItem
                sx={{
                  '&:hover': {
                    backgroundColor: 'rgba(255, 255, 255, 0.05)',
                  },
                }}
              >
                <ListItemIcon>
                  {getDeviceIcon(device.type)}
                </ListItemIcon>
                <ListItemText
                  primary={
                    <Box display="flex" alignItems="center" gap={1}>
                      <Typography variant="body1" sx={{ fontWeight: 'medium' }}>
                        {device.name}
                      </Typography>
                      <Chip
                        label={device.platform.toUpperCase()}
                        size="small"
                        sx={{
                          backgroundColor: getPlatformColor(device.platform),
                          color: 'white',
                          fontSize: '0.7rem',
                        }}
                      />
                    </Box>
                  }
                  secondary={
                    <Box>
                      <Typography variant="body2" color="text.secondary">
                        {device.ipAddress} • v{device.version} • {device.location}
                      </Typography>
                      <Box sx={{ display: 'flex', gap: 1, mt: 0.5 }}>
                        <Chip
                          icon={getStatusIcon(device.status)}
                          label={device.status.toUpperCase()}
                          size="small"
                          color={getStatusColor(device.status) as any}
                        />
                        <Chip
                          icon={getProtectionIcon(device.protection)}
                          label={device.protection.toUpperCase()}
                          size="small"
                          color={getProtectionColor(device.protection) as any}
                        />
                        <Chip
                          label={`${device.threatsBlocked} threats blocked`}
                          size="small"
                          variant="outlined"
                        />
                      </Box>
                      <Typography variant="caption" color="text.secondary" sx={{ mt: 0.5, display: 'block' }}>
                        Last seen: {formatLastSeen(device.lastSeen)} • Last scan: {device.lastScan}
                      </Typography>
                    </Box>
                  }
                />
                <ListItemSecondaryAction>
                  <IconButton onClick={() => handleDeviceDetails(device)}>
                    <InfoIcon />
                  </IconButton>
                  <IconButton onClick={() => handleRemoveDevice(device.id)}>
                    <DeleteIcon />
                  </IconButton>
                </ListItemSecondaryAction>
              </ListItem>
              <Divider />
            </React.Fragment>
          ))}
        </List>
      </Paper>

      {/* Device Details Dialog */}
      <Dialog open={detailsDialogOpen} onClose={() => setDetailsDialogOpen(false)} maxWidth="md" fullWidth>
        <DialogTitle>
          Device Details - {selectedDevice?.name}
        </DialogTitle>
        <DialogContent>
          {selectedDevice && (
            <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2, pt: 1 }}>
              <Grid container spacing={2}>
                <Grid item xs={12} sm={6}>
                  <Typography variant="subtitle2" color="text.secondary">Device Type</Typography>
                  <Typography variant="body1">{selectedDevice.type}</Typography>
                </Grid>
                <Grid item xs={12} sm={6}>
                  <Typography variant="subtitle2" color="text.secondary">Platform</Typography>
                  <Typography variant="body1">{selectedDevice.platform}</Typography>
                </Grid>
                <Grid item xs={12} sm={6}>
                  <Typography variant="subtitle2" color="text.secondary">IP Address</Typography>
                  <Typography variant="body1">{selectedDevice.ipAddress}</Typography>
                </Grid>
                <Grid item xs={12} sm={6}>
                  <Typography variant="subtitle2" color="text.secondary">Version</Typography>
                  <Typography variant="body1">{selectedDevice.version}</Typography>
                </Grid>
                <Grid item xs={12} sm={6}>
                  <Typography variant="subtitle2" color="text.secondary">Status</Typography>
                  <Chip
                    icon={getStatusIcon(selectedDevice.status)}
                    label={selectedDevice.status.toUpperCase()}
                    color={getStatusColor(selectedDevice.status) as any}
                  />
                </Grid>
                <Grid item xs={12} sm={6}>
                  <Typography variant="subtitle2" color="text.secondary">Protection</Typography>
                  <Chip
                    icon={getProtectionIcon(selectedDevice.protection)}
                    label={selectedDevice.protection.toUpperCase()}
                    color={getProtectionColor(selectedDevice.protection) as any}
                  />
                </Grid>
                <Grid item xs={12} sm={6}>
                  <Typography variant="subtitle2" color="text.secondary">Threats Blocked</Typography>
                  <Typography variant="body1">{selectedDevice.threatsBlocked}</Typography>
                </Grid>
                <Grid item xs={12} sm={6}>
                  <Typography variant="subtitle2" color="text.secondary">Last Scan</Typography>
                  <Typography variant="body1">{selectedDevice.lastScan}</Typography>
                </Grid>
                <Grid item xs={12} sm={6}>
                  <Typography variant="subtitle2" color="text.secondary">Last Seen</Typography>
                  <Typography variant="body1">{formatLastSeen(selectedDevice.lastSeen)}</Typography>
                </Grid>
                <Grid item xs={12} sm={6}>
                  <Typography variant="subtitle2" color="text.secondary">Location</Typography>
                  <Typography variant="body1">{selectedDevice.location || 'Unknown'}</Typography>
                </Grid>
              </Grid>
              
              {selectedDevice.status === 'vulnerable' && (
                <Alert severity="warning">
                  This device is vulnerable and may require updates or reinstallation of the antivirus software.
                </Alert>
              )}
              
              {selectedDevice.protection !== 'protected' && (
                <Alert severity="error">
                  This device is not fully protected. Please ensure GhostAntivirus is properly installed and running.
                </Alert>
              )}
            </Box>
          )}
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setDetailsDialogOpen(false)}>Close</Button>
        </DialogActions>
      </Dialog>
    </Box>
  )
}

export default DeviceManagement