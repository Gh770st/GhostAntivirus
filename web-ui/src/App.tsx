import React from 'react'
import { Routes, Route } from 'react-router-dom'
import { Box } from '@mui/material'

import Layout from '@/components/Layout'
import Dashboard from '@/pages/Dashboard'
import Scanner from '@/pages/Scanner'
import Firewall from '@/pages/Firewall'
import VPN from '@/pages/VPN'
import Settings from '@/pages/Settings'
import CryptoVault from '@/pages/CryptoVault'
import DeviceManagement from '@/pages/DeviceManagement'

function App() {
  return (
    <Box sx={{ display: 'flex', minHeight: '100vh' }}>
      <Layout>
        <Routes>
          <Route path="/" element={<Dashboard />} />
          <Route path="/dashboard" element={<Dashboard />} />
          <Route path="/scanner" element={<Scanner />} />
          <Route path="/firewall" element={<Firewall />} />
          <Route path="/vpn" element={<VPN />} />
          <Route path="/settings" element={<Settings />} />
          <Route path="/crypto-vault" element={<CryptoVault />} />
          <Route path="/devices" element={<DeviceManagement />} />
        </Routes>
      </Layout>
    </Box>
  )
}

export default App