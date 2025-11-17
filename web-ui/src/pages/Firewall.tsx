import React, { useState, useEffect } from 'react'
import {
  Box,
  Card,
  CardContent,
  Typography,
  Button,
  List,
  ListItem,
  ListItemText,
  ListItemSecondaryAction,
  Switch,
  Chip,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  IconButton,
  Alert,
  Grid,
  Paper,
  Divider,
} from '@mui/material'
import {
  Add as AddIcon,
  Edit as EditIcon,
  Delete as DeleteIcon,
  Security as SecurityIcon,
  Block as BlockIcon,
  CheckCircle as AllowIcon,
  Warning as WarningIcon,
} from '@mui/icons-material'

interface FirewallRule {
  id: string
  name: string
  action: 'allow' | 'deny' | 'block'
  sourceIP: string
  destIP: string
  sourcePort?: number
  destPort?: number
  protocol: 'tcp' | 'udp' | 'icmp' | 'all'
  enabled: boolean
  priority: number
  created: string
}

interface FirewallStats {
  totalRules: number
  activeRules: number
  blockedConnections: number
  allowedConnections: number
  threatsBlocked: number
}

const Firewall: React.FC = () => {
  const [rules, setRules] = useState<FirewallRule[]>([])
  const [stats, setStats] = useState<FirewallStats>({
    totalRules: 0,
    activeRules: 0,
    blockedConnections: 0,
    allowedConnections: 0,
    threatsBlocked: 0,
  })
  const [dialogOpen, setDialogOpen] = useState(false)
  const [editingRule, setEditingRule] = useState<FirewallRule | null>(null)
  const [formData, setFormData] = useState<Partial<FirewallRule>>({
    name: '',
    action: 'allow',
    sourceIP: '0.0.0.0',
    destIP: '0.0.0.0',
    protocol: 'tcp',
    enabled: true,
    priority: 100,
  })

  // Mock firewall rules
  const mockRules: FirewallRule[] = [
    {
      id: '1',
      name: 'Allow HTTP Traffic',
      action: 'allow',
      sourceIP: '0.0.0.0',
      destIP: '0.0.0.0',
      destPort: 80,
      protocol: 'tcp',
      enabled: true,
      priority: 100,
      created: '2024-01-15 10:30:00',
    },
    {
      id: '2',
      name: 'Allow HTTPS Traffic',
      action: 'allow',
      sourceIP: '0.0.0.0',
      destIP: '0.0.0.0',
      destPort: 443,
      protocol: 'tcp',
      enabled: true,
      priority: 100,
      created: '2024-01-15 10:30:00',
    },
    {
      id: '3',
      name: 'Block Malicious IP Range',
      action: 'block',
      sourceIP: '192.168.1.100',
      destIP: '0.0.0.0',
      protocol: 'all',
      enabled: true,
      priority: 10,
      created: '2024-01-16 14:20:00',
    },
    {
      id: '4',
      name: 'Allow SSH (Admin Only)',
      action: 'allow',
      sourceIP: '192.168.1.0/24',
      destIP: '0.0.0.0',
      destPort: 22,
      protocol: 'tcp',
      enabled: false,
      priority: 50,
      created: '2024-01-17 09:15:00',
    },
  ]

  useEffect(() => {
    setRules(mockRules)
    setStats({
      totalRules: mockRules.length,
      activeRules: mockRules.filter(r => r.enabled).length,
      blockedConnections: 1234,
      allowedConnections: 15678,
      threatsBlocked: 89,
    })
  }, [])

  const handleToggleRule = (ruleId: string) => {
    setRules(rules.map(rule => 
      rule.id === ruleId 
        ? { ...rule, enabled: !rule.enabled }
        : rule
    ))
  }

  const handleDeleteRule = (ruleId: string) => {
    setRules(rules.filter(rule => rule.id !== ruleId))
  }

  const handleEditRule = (rule: FirewallRule) => {
    setEditingRule(rule)
    setFormData(rule)
    setDialogOpen(true)
  }

  const handleCreateRule = () => {
    setEditingRule(null)
    setFormData({
      name: '',
      action: 'allow',
      sourceIP: '0.0.0.0',
      destIP: '0.0.0.0',
      protocol: 'tcp',
      enabled: true,
      priority: 100,
    })
    setDialogOpen(true)
  }

  const handleSaveRule = () => {
    if (editingRule) {
      // Update existing rule
      setRules(rules.map(rule => 
        rule.id === editingRule.id 
          ? { 
              ...rule, 
              ...formData, 
              created: new Date().toLocaleString() 
            } as FirewallRule
          : rule
      ))
    } else {
      // Create new rule
      const newRule: FirewallRule = {
        id: Date.now().toString(),
        name: formData.name || 'New Rule',
        action: formData.action || 'allow',
        sourceIP: formData.sourceIP || '0.0.0.0',
        destIP: formData.destIP || '0.0.0.0',
        sourcePort: formData.sourcePort,
        destPort: formData.destPort,
        protocol: formData.protocol || 'tcp',
        enabled: formData.enabled || true,
        priority: formData.priority || 100,
        created: new Date().toLocaleString(),
      }
      setRules([...rules, newRule])
    }
    setDialogOpen(false)
  }

  const getActionIcon = (action: string) => {
    switch (action) {
      case 'allow':
        return <AllowIcon color="success" />
      case 'deny':
        return <WarningIcon color="warning" />
      case 'block':
        return <BlockIcon color="error" />
      default:
        return <SecurityIcon />
    }
  }

  const getActionColor = (action: string) => {
    switch (action) {
      case 'allow':
        return 'success'
      case 'deny':
        return 'warning'
      case 'block':
        return 'error'
      default:
        return 'default'
    }
  }

  return (
    <Box>
      <Typography variant="h4" gutterBottom sx={{ mb: 3 }}>
        Firewall Management
      </Typography>

      <Grid container spacing={3} sx={{ mb: 3 }}>
        <Grid item xs={12} md={8}>
          <Card>
            <CardContent>
              <Box display="flex" justifyContent="space-between" alignItems="center" mb={2}>
                <Typography variant="h6">Firewall Rules</Typography>
                <Button
                  variant="contained"
                  startIcon={<AddIcon />}
                  onClick={handleCreateRule}
                >
                  Add Rule
                </Button>
              </Box>
              
              <Paper sx={{ maxHeight: 400, overflow: 'auto' }}>
                <List>
                  {rules.map((rule) => (
                    <React.Fragment key={rule.id}>
                      <ListItem>
                        <ListItemIcon>
                          {getActionIcon(rule.action)}
                        </ListItemIcon>
                        <ListItemText
                          primary={
                            <Box display="flex" alignItems="center" gap={1}>
                              <Typography variant="body1" sx={{ fontWeight: 'medium' }}>
                                {rule.name}
                              </Typography>
                              <Chip
                                label={rule.action.toUpperCase()}
                                color={getActionColor(rule.action) as any}
                                size="small"
                              />
                              <Chip
                                label={rule.protocol.toUpperCase()}
                                variant="outlined"
                                size="small"
                              />
                            </Box>
                          }
                          secondary={
                            <Box>
                              <Typography variant="body2" color="text.secondary">
                                {rule.sourceIP}:{rule.sourcePort || '*'} → {rule.destIP}:{rule.destPort || '*'}
                              </Typography>
                              <Typography variant="caption" color="text.secondary">
                                Priority: {rule.priority} • Created: {rule.created}
                              </Typography>
                            </Box>
                          }
                        />
                        <ListItemSecondaryAction>
                          <Switch
                            checked={rule.enabled}
                            onChange={() => handleToggleRule(rule.id)}
                            color="primary"
                          />
                          <IconButton onClick={() => handleEditRule(rule)} size="small">
                            <EditIcon />
                          </IconButton>
                          <IconButton onClick={() => handleDeleteRule(rule.id)} size="small">
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
        </Grid>

        <Grid item xs={12} md={4}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Firewall Statistics
              </Typography>
              <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
                <Box display="flex" justifyContent="space-between">
                  <Typography variant="body2">Total Rules</Typography>
                  <Typography variant="body2" color="primary">
                    {stats.totalRules}
                  </Typography>
                </Box>
                <Box display="flex" justifyContent="space-between">
                  <Typography variant="body2">Active Rules</Typography>
                  <Typography variant="body2" color="success">
                    {stats.activeRules}
                  </Typography>
                </Box>
                <Box display="flex" justifyContent="space-between">
                  <Typography variant="body2">Blocked Connections</Typography>
                  <Typography variant="body2" color="error">
                    {stats.blockedConnections.toLocaleString()}
                  </Typography>
                </Box>
                <Box display="flex" justifyContent="space-between">
                  <Typography variant="body2">Allowed Connections</Typography>
                  <Typography variant="body2" color="success">
                    {stats.allowedConnections.toLocaleString()}
                  </Typography>
                </Box>
                <Box display="flex" justifyContent="space-between">
                  <Typography variant="body2">Threats Blocked</Typography>
                  <Typography variant="body2" color="warning">
                    {stats.threatsBlocked}
                  </Typography>
                </Box>
              </Box>
              
              <Alert severity="info" sx={{ mt: 2 }}>
                Firewall is active and protecting your network
              </Alert>
            </CardContent>
          </Card>
        </Grid>
      </Grid>

      {/* Rule Dialog */}
      <Dialog open={dialogOpen} onClose={() => setDialogOpen(false)} maxWidth="sm" fullWidth>
        <DialogTitle>
          {editingRule ? 'Edit Firewall Rule' : 'Create Firewall Rule'}
        </DialogTitle>
        <DialogContent>
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2, pt: 1 }}>
            <TextField
              label="Rule Name"
              fullWidth
              value={formData.name}
              onChange={(e) => setFormData({ ...formData, name: e.target.value })}
            />
            
            <FormControl fullWidth>
              <InputLabel>Action</InputLabel>
              <Select
                value={formData.action}
                label="Action"
                onChange={(e) => setFormData({ ...formData, action: e.target.value as any })}
              >
                <MenuItem value="allow">Allow</MenuItem>
                <MenuItem value="deny">Deny</MenuItem>
                <MenuItem value="block">Block</MenuItem>
              </Select>
            </FormControl>
            
            <TextField
              label="Source IP"
              fullWidth
              value={formData.sourceIP}
              onChange={(e) => setFormData({ ...formData, sourceIP: e.target.value })}
              placeholder="0.0.0.0"
            />
            
            <TextField
              label="Destination IP"
              fullWidth
              value={formData.destIP}
              onChange={(e) => setFormData({ ...formData, destIP: e.target.value })}
              placeholder="0.0.0.0"
            />
            
            <Grid container spacing={2}>
              <Grid item xs={6}>
                <TextField
                  label="Source Port"
                  type="number"
                  fullWidth
                  value={formData.sourcePort || ''}
                  onChange={(e) => setFormData({ 
                    ...formData, 
                    sourcePort: e.target.value ? parseInt(e.target.value) : undefined 
                  })}
                />
              </Grid>
              <Grid item xs={6}>
                <TextField
                  label="Destination Port"
                  type="number"
                  fullWidth
                  value={formData.destPort || ''}
                  onChange={(e) => setFormData({ 
                    ...formData, 
                    destPort: e.target.value ? parseInt(e.target.value) : undefined 
                  })}
                />
              </Grid>
            </Grid>
            
            <FormControl fullWidth>
              <InputLabel>Protocol</InputLabel>
              <Select
                value={formData.protocol}
                label="Protocol"
                onChange={(e) => setFormData({ ...formData, protocol: e.target.value as any })}
              >
                <MenuItem value="tcp">TCP</MenuItem>
                <MenuItem value="udp">UDP</MenuItem>
                <MenuItem value="icmp">ICMP</MenuItem>
                <MenuItem value="all">All</MenuItem>
              </Select>
            </FormControl>
            
            <TextField
              label="Priority"
              type="number"
              fullWidth
              value={formData.priority}
              onChange={(e) => setFormData({ ...formData, priority: parseInt(e.target.value) })}
            />
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setDialogOpen(false)}>Cancel</Button>
          <Button onClick={handleSaveRule} variant="contained">
            {editingRule ? 'Update' : 'Create'}
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  )
}

export default Firewall