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
  IconButton,
  TextField,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  Chip,
  Alert,
  Grid,
  Paper,
  Divider,
  InputAdornment,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
} from '@mui/material'
import {
  Lock as LockIcon,
  Unlock as UnlockIcon,
  Visibility as VisibilityIcon,
  VisibilityOff as VisibilityOffIcon,
  Add as AddIcon,
  Edit as EditIcon,
  Delete as DeleteIcon,
  ContentCopy as CopyIcon,
  Key as KeyIcon,
  Folder as FolderIcon,
  Refresh as RefreshIcon,
} from '@mui/icons-material'

interface PasswordEntry {
  id: string
  title: string
  username: string
  password: string
  url?: string
  category: 'email' | 'finance' | 'social' | 'work' | 'general'
  notes?: string
  created: string
  lastModified: string
}

interface EncryptedFile {
  id: string
  filename: string
  originalPath: string
  size: string
  encryptedAt: string
  algorithm: string
}

const CryptoVault: React.FC = () => {
  const [isLocked, setIsLocked] = useState(true)
  const [masterPassword, setMasterPassword] = useState('')
  const [passwordEntries, setPasswordEntries] = useState<PasswordEntry[]>([])
  const [encryptedFiles, setEncryptedFiles] = useState<EncryptedFile[]>([])
  const [activeTab, setActiveTab] = useState<'passwords' | 'files'>('passwords')
  
  // Dialog states
  const [passwordDialogOpen, setPasswordDialogOpen] = useState(false)
  const [fileDialogOpen, setFileDialogOpen] = useState(false)
  const [editingPassword, setEditingPassword] = useState<PasswordEntry | null>(null)
  const [showPassword, setShowPassword] = useState<{ [key: string]: boolean }>({})
  
  // Form data
  const [passwordForm, setPasswordForm] = useState({
    title: '',
    username: '',
    password: '',
    url: '',
    category: 'general' as const,
    notes: '',
  })

  // Mock data
  const mockPasswords: PasswordEntry[] = [
    {
      id: '1',
      title: 'Gmail Account',
      username: 'user@gmail.com',
      password: 'GmailPassword123!',
      url: 'https://gmail.com',
      category: 'email',
      created: '2024-01-15 10:30:00',
      lastModified: '2024-01-20 14:22:00',
    },
    {
      id: '2',
      title: 'Bank of America',
      username: 'john.doe',
      password: 'SecureBankPass456!',
      url: 'https://bankofamerica.com',
      category: 'finance',
      created: '2024-01-10 09:15:00',
      lastModified: '2024-01-18 16:45:00',
    },
  ]

  const mockFiles: EncryptedFile[] = [
    {
      id: '1',
      filename: 'sensitive_documents.pdf',
      originalPath: '/home/user/Documents/sensitive_documents.pdf',
      size: '2.4 MB',
      encryptedAt: '2024-01-19 11:30:00',
      algorithm: 'AES-256-GCM',
    },
    {
      id: '2',
      filename: 'financial_records.xlsx',
      originalPath: '/home/user/Documents/financial_records.xlsx',
      size: '1.8 MB',
      encryptedAt: '2024-01-18 15:45:00',
      algorithm: 'AES-256-GCM',
    },
  ]

  useEffect(() => {
    if (!isLocked) {
      setPasswordEntries(mockPasswords)
      setEncryptedFiles(mockFiles)
    }
  }, [isLocked])

  const handleUnlock = () => {
    if (masterPassword === 'admin123') {
      setIsLocked(false)
      setMasterPassword('')
    } else {
      alert('Invalid master password')
    }
  }

  const handleLock = () => {
    setIsLocked(true)
    setPasswordEntries([])
    setEncryptedFiles([])
  }

  const handleAddPassword = () => {
    setEditingPassword(null)
    setPasswordForm({
      title: '',
      username: '',
      password: '',
      url: '',
      category: 'general',
      notes: '',
    })
    setPasswordDialogOpen(true)
  }

  const handleEditPassword = (password: PasswordEntry) => {
    setEditingPassword(password)
    setPasswordForm({
      title: password.title,
      username: password.username,
      password: password.password,
      url: password.url || '',
      category: password.category,
      notes: password.notes || '',
    })
    setPasswordDialogOpen(true)
  }

  const handleSavePassword = () => {
    if (editingPassword) {
      // Update existing password
      setPasswordEntries(passwordEntries.map(p => 
        p.id === editingPassword.id 
          ? { 
              ...p, 
              ...passwordForm, 
              lastModified: new Date().toLocaleString() 
            }
          : p
      ))
    } else {
      // Add new password
      const newPassword: PasswordEntry = {
        id: Date.now().toString(),
        ...passwordForm,
        created: new Date().toLocaleString(),
        lastModified: new Date().toLocaleString(),
      }
      setPasswordEntries([...passwordEntries, newPassword])
    }
    setPasswordDialogOpen(false)
  }

  const handleDeletePassword = (id: string) => {
    if (window.confirm('Are you sure you want to delete this password entry?')) {
      setPasswordEntries(passwordEntries.filter(p => p.id !== id))
    }
  }

  const handleCopyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text)
  }

  const togglePasswordVisibility = (id: string) => {
    setShowPassword(prev => ({
      ...prev,
      [id]: !prev[id]
    }))
  }

  const generateStrongPassword = () => {
    const charset = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*'
    let password = ''
    for (let i = 0; i < 16; i++) {
      password += charset.charAt(Math.floor(Math.random() * charset.length))
    }
    setPasswordForm(prev => ({ ...prev, password }))
  }

  const getCategoryColor = (category: string) => {
    const colors = {
      email: '#4caf50',
      finance: '#f44336',
      social: '#2196f3',
      work: '#ff9800',
      general: '#9e9e9e',
    }
    return colors[category as keyof typeof colors] || '#9e9e9e'
  }

  if (isLocked) {
    return (
      <Box display="flex" justifyContent="center" alignItems="center" minHeight="60vh">
        <Card sx={{ maxWidth: 400, width: '100%' }}>
          <CardContent>
            <Box textAlign="center" mb={3}>
              <LockIcon sx={{ fontSize: 64, color: 'primary.main', mb: 2 }} />
              <Typography variant="h4" gutterBottom>
                Crypto Vault
              </Typography>
              <Typography variant="body2" color="text.secondary">
                Enter your master password to access encrypted passwords and files
              </Typography>
            </Box>
            
            <TextField
              fullWidth
              type="password"
              label="Master Password"
              value={masterPassword}
              onChange={(e) => setMasterPassword(e.target.value)}
              onKeyPress={(e) => e.key === 'Enter' && handleUnlock()}
              margin="normal"
            />
            
            <Button
              fullWidth
              variant="contained"
              onClick={handleUnlock}
              sx={{ mt: 2 }}
              disabled={!masterPassword}
            >
              Unlock Vault
            </Button>
            
            <Typography variant="caption" color="text.secondary" sx={{ mt: 2, display: 'block' }}>
              Hint: Use "admin123" for demo
            </Typography>
          </CardContent>
        </Card>
      </Box>
    )
  }

  return (
    <Box>
      <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
        <Typography variant="h4">Crypto Vault</Typography>
        <Box sx={{ display: 'flex', gap: 2 }}>
          <Button
            variant="outlined"
            startIcon={<LockIcon />}
            onClick={handleLock}
          >
            Lock Vault
          </Button>
          <IconButton onClick={() => window.location.reload()}>
            <RefreshIcon />
          </IconButton>
        </Box>
      </Box>

      <Alert severity="success" sx={{ mb: 3 }}>
        Crypto Vault is unlocked and your data is secure with AES-256 encryption
      </Alert>

      <Grid container spacing={3}>
        <Grid item xs={12} md={8}>
          {/* Tab Navigation */}
          <Card sx={{ mb: 2 }}>
            <CardContent sx={{ p: 2 }}>
              <Box sx={{ display: 'flex', gap: 1 }}>
                <Button
                  variant={activeTab === 'passwords' ? 'contained' : 'outlined'}
                  onClick={() => setActiveTab('passwords')}
                  startIcon={<KeyIcon />}
                >
                  Passwords ({passwordEntries.length})
                </Button>
                <Button
                  variant={activeTab === 'files' ? 'contained' : 'outlined'}
                  onClick={() => setActiveTab('files')}
                  startIcon={<FolderIcon />}
                >
                  Encrypted Files ({encryptedFiles.length})
                </Button>
              </Box>
            </CardContent>
          </Card>

          {/* Passwords Tab */}
          {activeTab === 'passwords' && (
            <Card>
              <CardContent>
                <Box display="flex" justifyContent="space-between" alignItems="center" mb={2}>
                  <Typography variant="h6">Password Manager</Typography>
                  <Button
                    variant="contained"
                    startIcon={<AddIcon />}
                    onClick={handleAddPassword}
                  >
                    Add Password
                  </Button>
                </Box>
                
                <Paper sx={{ maxHeight: 400, overflow: 'auto' }}>
                  <List>
                    {passwordEntries.map((password) => (
                      <React.Fragment key={password.id}>
                        <ListItem>
                          <ListItemIcon>
                            <KeyIcon color="primary" />
                          </ListItemIcon>
                          <ListItemText
                            primary={
                              <Box display="flex" alignItems="center" gap={1}>
                                <Typography variant="body1" sx={{ fontWeight: 'medium' }}>
                                  {password.title}
                                </Typography>
                                <Chip
                                  label={password.category}
                                  size="small"
                                  sx={{
                                    backgroundColor: getCategoryColor(password.category),
                                    color: 'white',
                                  }}
                                />
                              </Box>
                            }
                            secondary={
                              <Box>
                                <Typography variant="body2" color="text.secondary">
                                  {password.username}
                                </Typography>
                                <Typography variant="caption" color="text.secondary">
                                  {password.url} • Modified: {password.lastModified}
                                </Typography>
                              </Box>
                            }
                          />
                          <ListItemSecondaryAction sx={{ display: 'flex', gap: 1 }}>
                            <IconButton
                              size="small"
                              onClick={() => togglePasswordVisibility(password.id)}
                            >
                              {showPassword[password.id] ? <VisibilityOffIcon /> : <VisibilityIcon />}
                            </IconButton>
                            <IconButton
                              size="small"
                              onClick={() => handleCopyToClipboard(password.password)}
                            >
                              <CopyIcon />
                            </IconButton>
                            <IconButton size="small" onClick={() => handleEditPassword(password)}>
                              <EditIcon />
                            </IconButton>
                            <IconButton size="small" onClick={() => handleDeletePassword(password.id)}>
                              <DeleteIcon />
                            </IconButton>
                          </ListItemSecondaryAction>
                        </ListItem>
                        {showPassword[password.id] && (
                          <Box sx={{ pl: 7, pr: 2, pb: 1 }}>
                            <Typography variant="body2" sx={{ fontFamily: 'monospace', wordBreak: 'break-all' }}>
                              {password.password}
                            </Typography>
                          </Box>
                        )}
                        <Divider />
                      </React.Fragment>
                    ))}
                  </List>
                </Paper>
              </CardContent>
            </Card>
          )}

          {/* Files Tab */}
          {activeTab === 'files' && (
            <Card>
              <CardContent>
                <Box display="flex" justifyContent="space-between" alignItems="center" mb={2}>
                  <Typography variant="h6">Encrypted Files</Typography>
                  <Button
                    variant="contained"
                    startIcon={<AddIcon />}
                    onClick={() => setFileDialogOpen(true)}
                  >
                    Encrypt File
                  </Button>
                </Box>
                
                <Paper sx={{ maxHeight: 400, overflow: 'auto' }}>
                  <List>
                    {encryptedFiles.map((file) => (
                      <React.Fragment key={file.id}>
                        <ListItem>
                          <ListItemIcon>
                            <FolderIcon color="primary" />
                          </ListItemIcon>
                          <ListItemText
                            primary={file.filename}
                            secondary={
                              <Box>
                                <Typography variant="body2" color="text.secondary">
                                  {file.originalPath}
                                </Typography>
                                <Typography variant="caption" color="text.secondary">
                                  {file.size} • {file.algorithm} • Encrypted: {file.encryptedAt}
                                </Typography>
                              </Box>
                            }
                          />
                          <ListItemSecondaryAction>
                            <IconButton size="small">
                              <EditIcon />
                            </IconButton>
                            <IconButton size="small">
                              <DeleteIcon />
                            </IconButton>
                          </ListItemSecondaryAction>
                        </ListItem>
                        <Divider />
                      </React.Fragment>
                    ))}
                  </List>
                </Paper>
              </CardContent>
            </Card>
          )}
        </Grid>

        <Grid item xs={12} md={4}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Vault Statistics
              </Typography>
              <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
                <Box display="flex" justifyContent="space-between">
                  <Typography variant="body2">Total Passwords</Typography>
                  <Typography variant="body2" color="primary">
                    {passwordEntries.length}
                  </Typography>
                </Box>
                <Box display="flex" justifyContent="space-between">
                  <Typography variant="body2">Encrypted Files</Typography>
                  <Typography variant="body2" color="primary">
                    {encryptedFiles.length}
                  </Typography>
                </Box>
                <Box display="flex" justifyContent="space-between">
                  <Typography variant="body2">Encryption</Typography>
                  <Typography variant="body2" color="success">
                    AES-256-GCM
                  </Typography>
                </Box>
                <Box display="flex" justifyContent="space-between">
                  <Typography variant="body2">Status</Typography>
                  <Chip
                    label="Secure"
                    color="success"
                    size="small"
                    icon={<UnlockIcon />}
                  />
                </Box>
              </Box>
            </CardContent>
          </Card>
        </Grid>
      </Grid>

      {/* Password Dialog */}
      <Dialog open={passwordDialogOpen} onClose={() => setPasswordDialogOpen(false)} maxWidth="sm" fullWidth>
        <DialogTitle>
          {editingPassword ? 'Edit Password' : 'Add New Password'}
        </DialogTitle>
        <DialogContent>
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2, pt: 1 }}>
            <TextField
              label="Title"
              fullWidth
              value={passwordForm.title}
              onChange={(e) => setPasswordForm({ ...passwordForm, title: e.target.value })}
            />
            
            <TextField
              label="Username/Email"
              fullWidth
              value={passwordForm.username}
              onChange={(e) => setPasswordForm({ ...passwordForm, username: e.target.value })}
            />
            
            <TextField
              label="Password"
              fullWidth
              type="password"
              value={passwordForm.password}
              onChange={(e) => setPasswordForm({ ...passwordForm, password: e.target.value })}
              InputProps={{
                endAdornment: (
                  <InputAdornment position="end">
                    <Button onClick={generateStrongPassword} size="small">
                      Generate
                    </Button>
                  </InputAdornment>
                ),
              }}
            />
            
            <TextField
              label="URL"
              fullWidth
              value={passwordForm.url}
              onChange={(e) => setPasswordForm({ ...passwordForm, url: e.target.value })}
            />
            
            <FormControl fullWidth>
              <InputLabel>Category</InputLabel>
              <Select
                value={passwordForm.category}
                label="Category"
                onChange={(e) => setPasswordForm({ ...passwordForm, category: e.target.value as any })}
              >
                <MenuItem value="email">Email</MenuItem>
                <MenuItem value="finance">Finance</MenuItem>
                <MenuItem value="social">Social</MenuItem>
                <MenuItem value="work">Work</MenuItem>
                <MenuItem value="general">General</MenuItem>
              </Select>
            </FormControl>
            
            <TextField
              label="Notes"
              fullWidth
              multiline
              rows={3}
              value={passwordForm.notes}
              onChange={(e) => setPasswordForm({ ...passwordForm, notes: e.target.value })}
            />
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setPasswordDialogOpen(false)}>Cancel</Button>
          <Button onClick={handleSavePassword} variant="contained">
            {editingPassword ? 'Update' : 'Save'}
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  )
}

export default CryptoVault