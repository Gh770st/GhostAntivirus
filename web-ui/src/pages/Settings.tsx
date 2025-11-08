import React, { useState, useEffect } from 'react'
import {
  Box,
  Card,
  CardContent,
  Typography,
  Switch,
  FormControlLabel,
  Button,
  Divider,
  Alert,
  Grid,
  TextField,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  Slider,
  Paper,
  List,
  ListItem,
  ListItemText,
  ListItemSecondaryAction,
} from '@mui/material'
import {
  Security as SecurityIcon,
  Update as UpdateIcon,
  Notifications as NotificationsIcon,
  Backup as BackupIcon,
  Privacy as PrivacyIcon,
  Speed as SpeedIcon,
} from '@mui/icons-material'

interface SettingsConfig {
  // Real-time Protection
  realTimeProtection: boolean
  fileScanning: boolean
  behaviorMonitoring: boolean
  networkProtection: boolean
  
  // Scan Settings
  scanFrequency: 'daily' | 'weekly' | 'monthly' | 'manual'
  scanType: 'quick' | 'full' | 'custom'
  heuristicAnalysis: boolean
  cloudAnalysis: boolean
  
  // Notifications
  threatNotifications: boolean
  scanNotifications: boolean
  updateNotifications: boolean
  emailReports: boolean
  
  // Privacy
  dataCollection: boolean
  crashReports: boolean
  usageStatistics: boolean
  
  // Performance
  maxCpuUsage: number
  maxMemoryUsage: number
  scanPriority: 'low' | 'normal' | 'high'
}

const Settings: React.FC = () => {
  const [config, setConfig] = useState<SettingsConfig>({
    realTimeProtection: true,
    fileScanning: true,
    behaviorMonitoring: true,
    networkProtection: true,
    scanFrequency: 'weekly',
    scanType: 'quick',
    heuristicAnalysis: true,
    cloudAnalysis: true,
    threatNotifications: true,
    scanNotifications: true,
    updateNotifications: true,
    emailReports: false,
    dataCollection: false,
    crashReports: true,
    usageStatistics: false,
    maxCpuUsage: 50,
    maxMemoryUsage: 512,
    scanPriority: 'normal',
  })
  
  const [saved, setSaved] = useState(false)

  const handleConfigChange = (key: keyof SettingsConfig, value: any) => {
    setConfig(prev => ({
      ...prev,
      [key]: value
    }))
    setSaved(false)
  }

  const handleSave = () => {
    // Simulate saving configuration
    setSaved(true)
    setTimeout(() => setSaved(false), 3000)
  }

  const handleReset = () => {
    setConfig({
      realTimeProtection: true,
      fileScanning: true,
      behaviorMonitoring: true,
      networkProtection: true,
      scanFrequency: 'weekly',
      scanType: 'quick',
      heuristicAnalysis: true,
      cloudAnalysis: true,
      threatNotifications: true,
      scanNotifications: true,
      updateNotifications: true,
      emailReports: false,
      dataCollection: false,
      crashReports: true,
      usageStatistics: false,
      maxCpuUsage: 50,
      maxMemoryUsage: 512,
      scanPriority: 'normal',
    })
  }

  const SectionCard: React.FC<{
    title: string
    icon: React.ReactNode
    children: React.ReactNode
  }> = ({ title, icon, children }) => (
    <Card sx={{ mb: 2 }}>
      <CardContent>
        <Box display="flex" alignItems="center" gap={1} mb={2}>
          {icon}
          <Typography variant="h6">{title}</Typography>
        </Box>
        {children}
      </CardContent>
    </Card>
  )

  return (
    <Box>
      <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
        <Typography variant="h4">Settings</Typography>
        <Box sx={{ display: 'flex', gap: 2 }}>
          <Button variant="outlined" onClick={handleReset}>
            Reset to Default
          </Button>
          <Button variant="contained" onClick={handleSave}>
            Save Settings
          </Button>
        </Box>
      </Box>

      {saved && (
        <Alert severity="success" sx={{ mb: 3 }}>
          Settings saved successfully!
        </Alert>
      )}

      <Grid container spacing={3}>
        <Grid item xs={12} lg={8}>
          {/* Real-time Protection */}
          <SectionCard title="Real-time Protection" icon={<SecurityIcon />}>
            <List>
              <ListItem>
                <ListItemText
                  primary="Enable Real-time Protection"
                  secondary="Continuously monitor your system for threats"
                />
                <ListItemSecondaryAction>
                  <Switch
                    checked={config.realTimeProtection}
                    onChange={(e) => handleConfigChange('realTimeProtection', e.target.checked)}
                    color="primary"
                  />
                </ListItemSecondaryAction>
              </ListItem>
              <Divider />
              <ListItem>
                <ListItemText
                  primary="File Scanning"
                  secondary="Scan files when they are accessed or created"
                />
                <ListItemSecondaryAction>
                  <Switch
                    checked={config.fileScanning}
                    onChange={(e) => handleConfigChange('fileScanning', e.target.checked)}
                    color="primary"
                    disabled={!config.realTimeProtection}
                  />
                </ListItemSecondaryAction>
              </ListItem>
              <Divider />
              <ListItem>
                <ListItemText
                  primary="Behavior Monitoring"
                  secondary="Monitor process behavior for suspicious activities"
                />
                <ListItemSecondaryAction>
                  <Switch
                    checked={config.behaviorMonitoring}
                    onChange={(e) => handleConfigChange('behaviorMonitoring', e.target.checked)}
                    color="primary"
                    disabled={!config.realTimeProtection}
                  />
                </ListItemSecondaryAction>
              </ListItem>
              <Divider />
              <ListItem>
                <ListItemText
                  primary="Network Protection"
                  secondary="Monitor network connections for malicious activity"
                />
                <ListItemSecondaryAction>
                  <Switch
                    checked={config.networkProtection}
                    onChange={(e) => handleConfigChange('networkProtection', e.target.checked)}
                    color="primary"
                    disabled={!config.realTimeProtection}
                  />
                </ListItemSecondaryAction>
              </ListItem>
            </List>
          </SectionCard>

          {/* Scan Settings */}
          <SectionCard title="Scan Settings" icon={<UpdateIcon />}>
            <Grid container spacing={2} sx={{ mb: 2 }}>
              <Grid item xs={12} sm={6}>
                <FormControl fullWidth>
                  <InputLabel>Scan Frequency</InputLabel>
                  <Select
                    value={config.scanFrequency}
                    label="Scan Frequency"
                    onChange={(e) => handleConfigChange('scanFrequency', e.target.value)}
                  >
                    <MenuItem value="daily">Daily</MenuItem>
                    <MenuItem value="weekly">Weekly</MenuItem>
                    <MenuItem value="monthly">Monthly</MenuItem>
                    <MenuItem value="manual">Manual Only</MenuItem>
                  </Select>
                </FormControl>
              </Grid>
              <Grid item xs={12} sm={6}>
                <FormControl fullWidth>
                  <InputLabel>Default Scan Type</InputLabel>
                  <Select
                    value={config.scanType}
                    label="Default Scan Type"
                    onChange={(e) => handleConfigChange('scanType', e.target.value)}
                  >
                    <MenuItem value="quick">Quick Scan</MenuItem>
                    <MenuItem value="full">Full Scan</MenuItem>
                    <MenuItem value="custom">Custom Scan</MenuItem>
                  </Select>
                </FormControl>
              </Grid>
            </Grid>
            
            <List>
              <ListItem>
                <ListItemText
                  primary="Heuristic Analysis"
                  secondary="Use advanced algorithms to detect unknown threats"
                />
                <ListItemSecondaryAction>
                  <Switch
                    checked={config.heuristicAnalysis}
                    onChange={(e) => handleConfigChange('heuristicAnalysis', e.target.checked)}
                    color="primary"
                  />
                </ListItemSecondaryAction>
              </ListItem>
              <Divider />
              <ListItem>
                <ListItemText
                  primary="Cloud Analysis"
                  secondary="Send suspicious files to cloud for analysis"
                />
                <ListItemSecondaryAction>
                  <Switch
                    checked={config.cloudAnalysis}
                    onChange={(e) => handleConfigChange('cloudAnalysis', e.target.checked)}
                    color="primary"
                  />
                </ListItemSecondaryAction>
              </ListItem>
            </List>
          </SectionCard>

          {/* Notifications */}
          <SectionCard title="Notifications" icon={<NotificationsIcon />}>
            <List>
              <ListItem>
                <ListItemText
                  primary="Threat Alerts"
                  secondary="Notify when threats are detected"
                />
                <ListItemSecondaryAction>
                  <Switch
                    checked={config.threatNotifications}
                    onChange={(e) => handleConfigChange('threatNotifications', e.target.checked)}
                    color="primary"
                  />
                </ListItemSecondaryAction>
              </ListItem>
              <Divider />
              <ListItem>
                <ListItemText
                  primary="Scan Notifications"
                  secondary="Notify when scans start or complete"
                />
                <ListItemSecondaryAction>
                  <Switch
                    checked={config.scanNotifications}
                    onChange={(e) => handleConfigChange('scanNotifications', e.target.checked)}
                    color="primary"
                  />
                </ListItemSecondaryAction>
              </ListItem>
              <Divider />
              <ListItem>
                <ListItemText
                  primary="Update Notifications"
                  secondary="Notify when updates are available"
                />
                <ListItemSecondaryAction>
                  <Switch
                    checked={config.updateNotifications}
                    onChange={(e) => handleConfigChange('updateNotifications', e.target.checked)}
                    color="primary"
                  />
                </ListItemSecondaryAction>
              </ListItem>
              <Divider />
              <ListItem>
                <ListItemText
                  primary="Email Reports"
                  secondary="Send weekly security reports via email"
                />
                <ListItemSecondaryAction>
                  <Switch
                    checked={config.emailReports}
                    onChange={(e) => handleConfigChange('emailReports', e.target.checked)}
                    color="primary"
                  />
                </ListItemSecondaryAction>
              </ListItem>
            </List>
          </SectionCard>

          {/* Privacy */}
          <SectionCard title="Privacy Settings" icon={<PrivacyIcon />}>
            <List>
              <ListItem>
                <ListItemText
                  primary="Data Collection"
                  secondary="Allow collection of threat intelligence data"
                />
                <ListItemSecondaryAction>
                  <Switch
                    checked={config.dataCollection}
                    onChange={(e) => handleConfigChange('dataCollection', e.target.checked)}
                    color="primary"
                  />
                </ListItemSecondaryAction>
              </ListItem>
              <Divider />
              <ListItem>
                <ListItemText
                  primary="Crash Reports"
                  secondary="Send anonymous crash reports to improve the software"
                />
                <ListItemSecondaryAction>
                  <Switch
                    checked={config.crashReports}
                    onChange={(e) => handleConfigChange('crashReports', e.target.checked)}
                    color="primary"
                  />
                </ListItemSecondaryAction>
              </ListItem>
              <Divider />
              <ListItem>
                <ListItemText
                  primary="Usage Statistics"
                  secondary="Send anonymous usage statistics"
                />
                <ListItemSecondaryAction>
                  <Switch
                    checked={config.usageStatistics}
                    onChange={(e) => handleConfigChange('usageStatistics', e.target.checked)}
                    color="primary"
                  />
                </ListItemSecondaryAction>
              </ListItem>
            </List>
          </SectionCard>
        </Grid>

        <Grid item xs={12} lg={4}>
          {/* Performance Settings */}
          <SectionCard title="Performance" icon={<SpeedIcon />}>
            <Box sx={{ mb: 3 }}>
              <Typography gutterBottom>
                Max CPU Usage: {config.maxCpuUsage}%
              </Typography>
              <Slider
                value={config.maxCpuUsage}
                onChange={(e, value) => handleConfigChange('maxCpuUsage', value)}
                min={10}
                max={100}
                step={5}
                marks={[
                  { value: 25, label: '25%' },
                  { value: 50, label: '50%' },
                  { value: 75, label: '75%' },
                  { value: 100, label: '100%' },
                ]}
                valueLabelDisplay="auto"
              />
            </Box>
            
            <Box sx={{ mb: 3 }}>
              <Typography gutterBottom>
                Max Memory Usage: {config.maxMemoryUsage} MB
              </Typography>
              <Slider
                value={config.maxMemoryUsage}
                onChange={(e, value) => handleConfigChange('maxMemoryUsage', value)}
                min={128}
                max={2048}
                step={128}
                marks={[
                  { value: 256, label: '256MB' },
                  { value: 512, label: '512MB' },
                  { value: 1024, label: '1GB' },
                  { value: 2048, label: '2GB' },
                ]}
                valueLabelDisplay="auto"
              />
            </Box>
            
            <FormControl fullWidth sx={{ mb: 2 }}>
              <InputLabel>Scan Priority</InputLabel>
              <Select
                value={config.scanPriority}
                label="Scan Priority"
                onChange={(e) => handleConfigChange('scanPriority', e.target.value)}
              >
                <MenuItem value="low">Low (Slower, less impact)</MenuItem>
                <MenuItem value="normal">Normal (Balanced)</MenuItem>
                <MenuItem value="high">High (Faster, more impact)</MenuItem>
              </Select>
            </FormControl>
          </SectionCard>

          {/* System Info */}
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                System Information
              </Typography>
              <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1 }}>
                <Box display="flex" justifyContent="space-between">
                  <Typography variant="body2">Version</Typography>
                  <Typography variant="body2" color="text.secondary">
                    3.0.0
                  </Typography>
                </Box>
                <Box display="flex" justifyContent="space-between">
                  <Typography variant="body2">Last Update</Typography>
                  <Typography variant="body2" color="text.secondary">
                    2024-01-20
                  </Typography>
                </Box>
                <Box display="flex" justifyContent="space-between">
                  <Typography variant="body2">License</Typography>
                  <Typography variant="body2" color="text.secondary">
                    Premium
                  </Typography>
                </Box>
                <Box display="flex" justifyContent="space-between">
                  <Typography variant="body2">Database Version</Typography>
                  <Typography variant="body2" color="text.secondary">
                    2024.01.20.001
                  </Typography>
                </Box>
              </Box>
            </CardContent>
          </Card>
        </Grid>
      </Grid>
    </Box>
  )
}

export default Settings