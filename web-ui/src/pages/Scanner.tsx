import React, { useState, useEffect } from 'react'
import {
  Box,
  Card,
  CardContent,
  Typography,
  Button,
  LinearProgress,
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
  Chip,
  Alert,
  Grid,
  IconButton,
  Paper,
  Divider,
} from '@mui/material'
import {
  PlayArrow as PlayIcon,
  Pause as PauseIcon,
  Stop as StopIcon,
  Folder as FolderIcon,
  Security as ScanIcon,
  BugReport as ThreatIcon,
  CheckCircle as SafeIcon,
  Error as ErrorIcon,
  Refresh as RefreshIcon,
} from '@mui/icons-material'

interface ScanResult {
  id: string
  fileName: string
  filePath: string
  status: 'safe' | 'threat' | 'suspicious' | 'scanning'
  threatType?: string
  fileSize: string
  scanTime: string
}

interface ScanStats {
  totalFiles: number
  filesScanned: number
  threatsFound: number
  filesInQuarantine: number
  scanStartTime?: string
  estimatedTimeRemaining?: string
}

const Scanner: React.FC = () => {
  const [isScanning, setIsScanning] = useState(false)
  const [scanPaused, setScanPaused] = useState(false)
  const [scanResults, setScanResults] = useState<ScanResult[]>([])
  const [scanStats, setScanStats] = useState<ScanStats>({
    totalFiles: 0,
    filesScanned: 0,
    threatsFound: 0,
    filesInQuarantine: 0,
  })

  // Mock scan results
  const mockResults: ScanResult[] = [
    {
      id: '1',
      fileName: 'document.pdf',
      filePath: '/home/user/Documents/document.pdf',
      status: 'safe',
      fileSize: '2.4 MB',
      scanTime: '0.2s',
    },
    {
      id: '2',
      fileName: 'malware_sample.exe',
      filePath: '/home/user/Downloads/malware_sample.exe',
      status: 'threat',
      threatType: 'Trojan',
      fileSize: '1.8 MB',
      scanTime: '0.5s',
    },
    {
      id: '3',
      fileName: 'application.exe',
      filePath: '/home/user/Downloads/application.exe',
      status: 'suspicious',
      threatType: 'Heuristic Detection',
      fileSize: '15.2 MB',
      scanTime: '1.2s',
    },
    {
      id: '4',
      fileName: 'photo.jpg',
      filePath: '/home/user/Pictures/photo.jpg',
      status: 'safe',
      fileSize: '3.7 MB',
      scanTime: '0.1s',
    },
  ]

  useEffect(() => {
    // Load initial scan results
    setScanResults(mockResults)
    setScanStats({
      totalFiles: 15000,
      filesScanned: 8234,
      threatsFound: 2,
      filesInQuarantine: 1,
      scanStartTime: new Date(Date.now() - 300000).toLocaleString(),
      estimatedTimeRemaining: '5 minutes',
    })
  }, [])

  const startScan = () => {
    setIsScanning(true)
    setScanPaused(false)
    
    // Simulate scanning process
    let progress = 0
    const interval = setInterval(() => {
      if (!scanPaused && isScanning) {
        progress += Math.random() * 5
        if (progress >= 100) {
          progress = 100
          setIsScanning(false)
          clearInterval(interval)
        }
        
        setScanStats(prev => ({
          ...prev,
          filesScanned: Math.min(prev.totalFiles, Math.floor((prev.totalFiles * progress) / 100)),
        }))
      }
    }, 1000)
  }

  const pauseScan = () => {
    setScanPaused(!scanPaused)
  }

  const stopScan = () => {
    setIsScanning(false)
    setScanPaused(false)
  }

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'safe':
        return <SafeIcon color="success" />
      case 'threat':
        return <ThreatIcon color="error" />
      case 'suspicious':
        return <ErrorIcon color="warning" />
      case 'scanning':
        return <ScanIcon color="info" />
      default:
        return <ScanIcon color="disabled" />
    }
  }

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'safe':
        return 'success'
      case 'threat':
        return 'error'
      case 'suspicious':
        return 'warning'
      case 'scanning':
        return 'info'
      default:
        return 'default'
    }
  }

  const scanProgress = scanStats.totalFiles > 0 
    ? (scanStats.filesScanned / scanStats.totalFiles) * 100 
    : 0

  return (
    <Box>
      <Typography variant="h4" gutterBottom sx={{ mb: 3 }}>
        Malware Scanner
      </Typography>

      {/* Scan Controls */}
      <Grid container spacing={3} sx={{ mb: 3 }}>
        <Grid item xs={12} md={8}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                System Scanner
              </Typography>
              
              <Box sx={{ mb: 3 }}>
                <Typography variant="body2" color="text.secondary" gutterBottom>
                  {isScanning 
                    ? `Scanning: ${scanStats.filesScanned.toLocaleString()} / ${scanStats.totalFiles.toLocaleString()} files`
                    : scanStats.filesScanned > 0 
                    ? `Scan completed: ${scanStats.filesScanned.toLocaleString()} files scanned`
                    : 'Ready to scan'
                  }
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
                  {scanStats.estimatedTimeRemaining && isScanning && 
                    ` • ${scanStats.estimatedTimeRemaining} remaining`
                  }
                </Typography>
              </Box>

              <Box sx={{ display: 'flex', gap: 2 }}>
                {!isScanning ? (
                  <Button
                    variant="contained"
                    startIcon={<PlayIcon />}
                    onClick={startScan}
                    color="primary"
                  >
                    Start Scan
                  </Button>
                ) : (
                  <>
                    <Button
                      variant="contained"
                      startIcon={scanPaused ? <PlayIcon /> : <PauseIcon />}
                      onClick={pauseScan}
                      color={scanPaused ? 'success' : 'warning'}
                    >
                      {scanPaused ? 'Resume' : 'Pause'}
                    </Button>
                    <Button
                      variant="contained"
                      startIcon={<StopIcon />}
                      onClick={stopScan}
                      color="error"
                    >
                      Stop
                    </Button>
                  </>
                )}
                <IconButton onClick={() => window.location.reload()}>
                  <RefreshIcon />
                </IconButton>
              </Box>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} md={4}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Scan Statistics
              </Typography>
              <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
                <Box display="flex" justifyContent="space-between">
                  <Typography variant="body2">Files Scanned</Typography>
                  <Typography variant="body2" color="primary">
                    {scanStats.filesScanned.toLocaleString()}
                  </Typography>
                </Box>
                <Box display="flex" justifyContent="space-between">
                  <Typography variant="body2">Threats Found</Typography>
                  <Typography variant="body2" color="error">
                    {scanStats.threatsFound}
                  </Typography>
                </Box>
                <Box display="flex" justifyContent="space-between">
                  <Typography variant="body2">Files in Quarantine</Typography>
                  <Typography variant="body2" color="warning">
                    {scanStats.filesInQuarantine}
                  </Typography>
                </Box>
                {scanStats.scanStartTime && (
                  <Box display="flex" justifyContent="space-between">
                    <Typography variant="body2">Scan Started</Typography>
                    <Typography variant="body2" color="text.secondary">
                      {scanStats.scanStartTime}
                    </Typography>
                  </Box>
                )}
              </Box>
            </CardContent>
          </Card>
        </Grid>
      </Grid>

      {/* Alerts */}
      {scanStats.threatsFound > 0 && (
        <Alert severity="error" sx={{ mb: 3 }}>
          {scanStats.threatsFound} threat(s) detected during scan. Review the scan results below.
        </Alert>
      )}

      {/* Scan Results */}
      <Paper sx={{ maxHeight: 600, overflow: 'auto' }}>
        <Box sx={{ p: 2, borderBottom: 1, borderColor: 'divider' }}>
          <Typography variant="h6">Scan Results</Typography>
        </Box>
        <List>
          {scanResults.map((result, index) => (
            <React.Fragment key={result.id}>
              <ListItem>
                <ListItemIcon>
                  {getStatusIcon(result.status)}
                </ListItemIcon>
                <ListItemText
                  primary={
                    <Box display="flex" alignItems="center" gap={1}>
                      <Typography variant="body1" sx={{ fontWeight: 'medium' }}>
                        {result.fileName}
                      </Typography>
                      <Chip
                        label={result.status.toUpperCase()}
                        color={getStatusColor(result.status) as any}
                        size="small"
                      />
                      {result.threatType && (
                        <Chip
                          label={result.threatType}
                          variant="outlined"
                          size="small"
                          color="error"
                        />
                      )}
                    </Box>
                  }
                  secondary={
                    <Box>
                      <Typography variant="body2" color="text.secondary">
                        {result.filePath}
                      </Typography>
                      <Typography variant="caption" color="text.secondary">
                        {result.fileSize} • Scanned in {result.scanTime}
                      </Typography>
                    </Box>
                  }
                />
              </ListItem>
              {index < scanResults.length - 1 && <Divider />}
            </React.Fragment>
          ))}
        </List>
      </Paper>
    </Box>
  )
}

export default Scanner