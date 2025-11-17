import React, { useEffect, useState } from 'react'
import {
  Grid,
  Card,
  CardContent,
  Typography,
  Box,
  LinearProgress,
  Alert,
  Chip,
} from '@mui/material'
import {
  Security as SecurityIcon,
  BugReport as ThreatIcon,
  Shield as ShieldIcon,
  Update as UpdateIcon,
  TrendingUp as TrendingUpIcon,
  Warning as WarningIcon,
} from '@mui/icons-material'

interface DashboardStats {
  totalFiles: number
  filesScanned: number
  threatsDetected: number
  threatsBlocked: number
  realTimeProtection: boolean
  lastScan: string
  lastUpdate: string
}

const Dashboard: React.FC = () => {
  const [stats, setStats] = useState<DashboardStats>({
    totalFiles: 0,
    filesScanned: 0,
    threatsDetected: 0,
    threatsBlocked: 0,
    realTimeProtection: true,
    lastScan: '',
    lastUpdate: '',
  })
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    // Simulate fetching dashboard data
    const fetchDashboardData = async () => {
      setLoading(true)
      
      // Simulate API delay
      await new Promise(resolve => setTimeout(resolve, 1000))
      
      setStats({
        totalFiles: 15234,
        filesScanned: 14234,
        threatsDetected: 12,
        threatsBlocked: 8,
        realTimeProtection: true,
        lastScan: new Date(Date.now() - 3600000).toLocaleString(),
        lastUpdate: new Date(Date.now() - 86400000).toLocaleString(),
      })
      
      setLoading(false)
    }

    fetchDashboardData()
  }, [])

  const scanProgress = (stats.filesScanned / stats.totalFiles) * 100

  const StatCard: React.FC<{
    title: string
    value: string | number
    icon: React.ReactNode
    color?: 'primary' | 'secondary' | 'success' | 'warning' | 'error'
    trend?: number
  }> = ({ title, value, icon, color = 'primary', trend }) => (
    <Card
      sx={{
        height: '100%',
        background: 'linear-gradient(135deg, #1a1a1a 0%, #2d2d2d 100%)',
        border: '1px solid rgba(255, 255, 255, 0.1)',
        transition: 'all 0.3s ease',
        '&:hover': {
          transform: 'translateY(-4px)',
          boxShadow: '0 8px 25px rgba(0, 188, 212, 0.2)',
        },
      }}
    >
      <CardContent>
        <Box display="flex" alignItems="center" justifyContent="space-between" mb={2}>
          <Box color={`${color}.main`}>{icon}</Box>
          {trend !== undefined && (
            <Chip
              icon={<TrendingUpIcon />}
              label={`${trend > 0 ? '+' : ''}${trend}%`}
              size="small"
              color={trend > 0 ? 'success' : 'error'}
              variant="outlined"
            />
          )}
        </Box>
        <Typography variant="h4" component="div" gutterBottom>
          {value}
        </Typography>
        <Typography variant="body2" color="text.secondary">
          {title}
        </Typography>
      </CardContent>
    </Card>
  )

  if (loading) {
    return (
      <Box>
        <Typography variant="h4" gutterBottom>
          Dashboard
        </Typography>
        <LinearProgress />
      </Box>
    )
  }

  return (
    <Box>
      <Typography variant="h4" gutterBottom sx={{ mb: 3 }}>
        Security Dashboard
      </Typography>

      {/* Alert for real-time protection status */}
      {stats.realTimeProtection ? (
        <Alert severity="success" sx={{ mb: 3 }}>
          Real-time protection is active and monitoring your system
        </Alert>
      ) : (
        <Alert severity="warning" sx={{ mb: 3 }}>
          Real-time protection is disabled - your system may be at risk
        </Alert>
      )}

      {/* Stats Grid */}
      <Grid container spacing={3}>
        <Grid item xs={12} sm={6} md={3}>
          <StatCard
            title="Total Files"
            value={stats.totalFiles.toLocaleString()}
            icon={<SecurityIcon sx={{ fontSize: 40 }} />}
            color="primary"
          />
        </Grid>
        <Grid item xs={12} sm={6} md={3}>
          <StatCard
            title="Files Scanned"
            value={stats.filesScanned.toLocaleString()}
            icon={<ShieldIcon sx={{ fontSize: 40 }} />}
            color="success"
            trend={12}
          />
        </Grid>
        <Grid item xs={12} sm={6} md={3}>
          <StatCard
            title="Threats Detected"
            value={stats.threatsDetected}
            icon={<ThreatIcon sx={{ fontSize: 40 }} />}
            color="warning"
          />
        </Grid>
        <Grid item xs={12} sm={6} md={3}>
          <StatCard
            title="Threats Blocked"
            value={stats.threatsBlocked}
            icon={<ShieldIcon sx={{ fontSize: 40 }} />}
            color="error"
          />
        </Grid>
      </Grid>

      {/* Progress and Status */}
      <Grid container spacing={3} sx={{ mt: 1 }}>
        <Grid item xs={12} md={8}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                System Scan Progress
              </Typography>
              <Box sx={{ mb: 2 }}>
                <Typography variant="body2" color="text.secondary" gutterBottom>
                  {stats.filesScanned.toLocaleString()} of {stats.totalFiles.toLocaleString()} files scanned
                </Typography>
                <LinearProgress
                  variant="determinate"
                  value={scanProgress}
                  sx={{
                    height: 8,
                    borderRadius: 4,
                    backgroundColor: 'rgba(255, 255, 255, 0.1)',
                    '& .MuiLinearProgress-bar': {
                      borderRadius: 4,
                      backgroundColor: '#00bcd4',
                    },
                  }}
                />
                <Typography variant="body2" color="text.secondary" sx={{ mt: 1 }}>
                  {scanProgress.toFixed(1)}% Complete
                </Typography>
              </Box>
            </CardContent>
          </Card>
        </Grid>
        
        <Grid item xs={12} md={4}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                System Status
              </Typography>
              <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
                <Box display="flex" alignItems="center" justifyContent="space-between">
                  <Typography variant="body2">Real-time Protection</Typography>
                  <Chip
                    label={stats.realTimeProtection ? 'Active' : 'Inactive'}
                    color={stats.realTimeProtection ? 'success' : 'error'}
                    size="small"
                  />
                </Box>
                <Box display="flex" alignItems="center" justifyContent="space-between">
                  <Typography variant="body2">Last Scan</Typography>
                  <Typography variant="body2" color="text.secondary">
                    {stats.lastScan}
                  </Typography>
                </Box>
                <Box display="flex" alignItems="center" justifyContent="space-between">
                  <Typography variant="body2">Last Update</Typography>
                  <Typography variant="body2" color="text.secondary">
                    {stats.lastUpdate}
                  </Typography>
                </Box>
              </Box>
            </CardContent>
          </Card>
        </Grid>
      </Grid>

      {/* Recent Activity */}
      <Grid container spacing={3} sx={{ mt: 1 }}>
        <Grid item xs={12}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Recent Security Events
              </Typography>
              <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
                <Box display="flex" alignItems="center" gap={2}>
                  <WarningIcon color="warning" />
                  <Typography variant="body2">
                    Suspicious file detected: malware_sample.exe in Downloads folder
                  </Typography>
                  <Typography variant="caption" color="text.secondary" sx={{ ml: 'auto' }}>
                    2 minutes ago
                  </Typography>
                </Box>
                <Box display="flex" alignItems="center" gap={2}>
                  <ShieldIcon color="success" />
                  <Typography variant="body2">
                    System scan completed successfully
                  </Typography>
                  <Typography variant="caption" color="text.secondary" sx={{ ml: 'auto' }}>
                    1 hour ago
                  </Typography>
                </Box>
                <Box display="flex" alignItems="center" gap={2}>
                  <UpdateIcon color="info" />
                  <Typography variant="body2">
                    Virus definitions updated to version 3.0.1
                  </Typography>
                  <Typography variant="caption" color="text.secondary" sx={{ ml: 'auto' }}>
                    3 hours ago
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

export default Dashboard