//! Network Monitoring Module
//! 
//! Monitors network connections and detects suspicious network activity.

use std::collections::{HashMap, HashSet};
use std::net::{IpAddr, Ipv4Addr};
use std::time::{Duration, SystemTime};
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use log::{info, warn, debug};

use crate::config::Config;

/// Network connection information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub id: String,
    pub process_id: u32,
    pub process_name: String,
    pub local_addr: IpAddr,
    pub local_port: u16,
    pub remote_addr: IpAddr,
    pub remote_port: u16,
    pub protocol: Protocol,
    pub state: ConnectionState,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub established_at: SystemTime,
}

/// Network protocol
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Protocol {
    TCP,
    UDP,
    ICMP,
    Unknown,
}

/// Connection state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConnectionState {
    Established,
    Listen,
    TimeWait,
    CloseWait,
    Closed,
}

/// Traffic statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficStats {
    pub total_connections: usize,
    pub active_connections: usize,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_sent: u64,
    pub packets_received: u64,
    pub suspicious_connections: usize,
    pub blocked_connections: usize,
}

/// Network Monitor
pub struct NetworkMonitor {
    connections: HashMap<String, Connection>,
    suspicious_ips: HashSet<IpAddr>,
    blocked_ips: HashSet<IpAddr>,
    traffic_stats: TrafficStats,
    monitoring: bool,
    config: Config,
}

impl NetworkMonitor {
    /// Create new network monitor
    pub fn new(config: Config) -> Result<Self> {
        info!("Initializing Network Monitor");
        
        Ok(Self {
            connections: HashMap::new(),
            suspicious_ips: Self::load_suspicious_ips(),
            blocked_ips: HashSet::new(),
            traffic_stats: TrafficStats::default(),
            monitoring: false,
            config,
        })
    }
    
    /// Load known suspicious IPs
    fn load_suspicious_ips() -> HashSet<IpAddr> {
        let mut ips = HashSet::new();
        
        // Add some example suspicious IPs
        // In production, load from threat intelligence feeds
        ips.insert(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)));
        ips.insert(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)));
        
        ips
    }
    
    /// Start monitoring network connections
    pub fn start_monitoring(&mut self) -> Result<()> {
        if self.monitoring {
            warn!("Network monitoring already started");
            return Ok(());
        }
        
        info!("Starting network monitoring");
        self.monitoring = true;
        
        // In production, this would start a background thread
        // that continuously monitors network connections
        
        Ok(())
    }
    
    /// Stop monitoring
    pub fn stop_monitoring(&mut self) {
        if !self.monitoring {
            return;
        }
        
        info!("Stopping network monitoring");
        self.monitoring = false;
    }
    
    /// Get all active connections
    pub fn get_connections(&self) -> Vec<Connection> {
        self.connections.values().cloned().collect()
    }
    
    /// Get connections for a specific process
    pub fn get_process_connections(&self, process_id: u32) -> Vec<Connection> {
        self.connections.values()
            .filter(|c| c.process_id == process_id)
            .cloned()
            .collect()
    }
    
    /// Check if a connection is suspicious
    pub fn is_suspicious(&self, conn: &Connection) -> bool {
        // Check if remote IP is in suspicious list
        if self.suspicious_ips.contains(&conn.remote_addr) {
            return true;
        }
        
        // Check for suspicious ports
        if self.is_suspicious_port(conn.remote_port) {
            return true;
        }
        
        // Check for unusual traffic patterns
        if self.has_unusual_traffic(conn) {
            return true;
        }
        
        false
    }
    
    /// Check if port is suspicious
    fn is_suspicious_port(&self, port: u16) -> bool {
        // Common malware ports
        let suspicious_ports = [
            4444,  // Metasploit
            5555,  // Android Debug Bridge (when not expected)
            6666,  // IRC bots
            6667,  // IRC
            31337, // Back Orifice
            12345, // NetBus
        ];
        
        suspicious_ports.contains(&port)
    }
    
    /// Check for unusual traffic patterns
    fn has_unusual_traffic(&self, conn: &Connection) -> bool {
        // Check for excessive data transfer
        let total_bytes = conn.bytes_sent + conn.bytes_received;
        
        // More than 1GB in a short time is suspicious
        if total_bytes > 1_000_000_000 {
            if let Ok(duration) = SystemTime::now().duration_since(conn.established_at) {
                if duration < Duration::from_secs(60) {
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Block a connection
    pub fn block_connection(&mut self, conn: &Connection) -> Result<()> {
        info!("Blocking connection to {}:{}", conn.remote_addr, conn.remote_port);
        
        // Add IP to blocked list
        self.blocked_ips.insert(conn.remote_addr);
        
        // Remove from active connections
        self.connections.remove(&conn.id);
        
        // Update statistics
        self.traffic_stats.blocked_connections += 1;
        
        // In production, this would use firewall rules to block the IP
        
        Ok(())
    }
    
    /// Unblock an IP address
    pub fn unblock_ip(&mut self, ip: IpAddr) -> Result<()> {
        info!("Unblocking IP: {}", ip);
        self.blocked_ips.remove(&ip);
        Ok(())
    }
    
    /// Check if IP is blocked
    pub fn is_blocked(&self, ip: &IpAddr) -> bool {
        self.blocked_ips.contains(ip)
    }
    
    /// Add IP to suspicious list
    pub fn mark_suspicious(&mut self, ip: IpAddr) {
        info!("Marking IP as suspicious: {}", ip);
        self.suspicious_ips.insert(ip);
    }
    
    /// Remove IP from suspicious list
    pub fn unmark_suspicious(&mut self, ip: &IpAddr) {
        self.suspicious_ips.remove(ip);
    }
    
    /// Get traffic statistics
    pub fn get_statistics(&self) -> TrafficStats {
        self.traffic_stats.clone()
    }
    
    /// Update connection information
    pub fn update_connection(&mut self, conn: Connection) {
        let is_new = !self.connections.contains_key(&conn.id);
        
        if is_new {
            self.traffic_stats.total_connections += 1;
            if conn.state == ConnectionState::Established {
                self.traffic_stats.active_connections += 1;
            }
        }
        
        // Update traffic stats
        if let Some(old_conn) = self.connections.get(&conn.id) {
            self.traffic_stats.bytes_sent += conn.bytes_sent.saturating_sub(old_conn.bytes_sent);
            self.traffic_stats.bytes_received += conn.bytes_received.saturating_sub(old_conn.bytes_received);
        }
        
        // Check if suspicious
        if self.is_suspicious(&conn) {
            self.traffic_stats.suspicious_connections += 1;
        }
        
        self.connections.insert(conn.id.clone(), conn);
    }
    
    /// Remove closed connections
    pub fn cleanup_closed_connections(&mut self) {
        let closed: Vec<String> = self.connections.iter()
            .filter(|(_, conn)| conn.state == ConnectionState::Closed)
            .map(|(id, _)| id.clone())
            .collect();
        
        for id in closed {
            self.connections.remove(&id);
            self.traffic_stats.active_connections = self.traffic_stats.active_connections.saturating_sub(1);
        }
    }
    
    /// Get connections to a specific IP
    pub fn get_connections_to_ip(&self, ip: &IpAddr) -> Vec<Connection> {
        self.connections.values()
            .filter(|c| &c.remote_addr == ip)
            .cloned()
            .collect()
    }
    
    /// Get suspicious connections
    pub fn get_suspicious_connections(&self) -> Vec<Connection> {
        self.connections.values()
            .filter(|c| self.is_suspicious(c))
            .cloned()
            .collect()
    }
    
    /// Reset statistics
    pub fn reset_statistics(&mut self) {
        info!("Resetting network statistics");
        self.traffic_stats = TrafficStats::default();
    }
    
    /// Check if monitoring is active
    pub fn is_monitoring(&self) -> bool {
        self.monitoring
    }
    
    /// Scan network for active connections
    pub async fn scan_network(&mut self) -> Result<String> {
        use uuid::Uuid;
        
        info!("Starting network scan");
        
        let scan_id = Uuid::new_v4().to_string();
        
        // Start monitoring if not already started
        if !self.monitoring {
            self.start_monitoring()?;
        }
        
        // In a real implementation, this would:
        // 1. Enumerate all network interfaces
        // 2. Scan for active connections
        // 3. Identify processes using connections
        // 4. Check for suspicious activity
        
        info!("Network scan started with ID: {}", scan_id);
        
        Ok(scan_id)
    }
}

impl Default for TrafficStats {
    fn default() -> Self {
        Self {
            total_connections: 0,
            active_connections: 0,
            bytes_sent: 0,
            bytes_received: 0,
            packets_sent: 0,
            packets_received: 0,
            suspicious_connections: 0,
            blocked_connections: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_config() -> Config {
        Config::default()
    }
    
    fn create_test_connection() -> Connection {
        Connection {
            id: "test_conn_1".to_string(),
            process_id: 1234,
            process_name: "test.exe".to_string(),
            local_addr: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 10)),
            local_port: 50000,
            remote_addr: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
            remote_port: 443,
            protocol: Protocol::TCP,
            state: ConnectionState::Established,
            bytes_sent: 1024,
            bytes_received: 2048,
            established_at: SystemTime::now(),
        }
    }
    
    #[test]
    fn test_network_monitor_creation() {
        let config = create_test_config();
        let monitor = NetworkMonitor::new(config);
        assert!(monitor.is_ok());
    }
    
    #[test]
    fn test_start_stop_monitoring() {
        let config = create_test_config();
        let mut monitor = NetworkMonitor::new(config).unwrap();
        
        assert!(!monitor.is_monitoring());
        
        monitor.start_monitoring().unwrap();
        assert!(monitor.is_monitoring());
        
        monitor.stop_monitoring();
        assert!(!monitor.is_monitoring());
    }
    
    #[test]
    fn test_update_connection() {
        let config = create_test_config();
        let mut monitor = NetworkMonitor::new(config).unwrap();
        
        let conn = create_test_connection();
        monitor.update_connection(conn.clone());
        
        let connections = monitor.get_connections();
        assert_eq!(connections.len(), 1);
        assert_eq!(connections[0].id, conn.id);
    }
    
    #[test]
    fn test_suspicious_port_detection() {
        let config = create_test_config();
        let monitor = NetworkMonitor::new(config).unwrap();
        
        assert!(monitor.is_suspicious_port(4444));
        assert!(monitor.is_suspicious_port(31337));
        assert!(!monitor.is_suspicious_port(443));
        assert!(!monitor.is_suspicious_port(80));
    }
    
    #[test]
    fn test_block_unblock_ip() {
        let config = create_test_config();
        let mut monitor = NetworkMonitor::new(config).unwrap();
        
        let ip = IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4));
        
        assert!(!monitor.is_blocked(&ip));
        
        let mut conn = create_test_connection();
        conn.remote_addr = ip;
        monitor.update_connection(conn.clone());
        monitor.block_connection(&conn).unwrap();
        
        assert!(monitor.is_blocked(&ip));
        
        monitor.unblock_ip(ip).unwrap();
        assert!(!monitor.is_blocked(&ip));
    }
    
    #[test]
    fn test_mark_suspicious() {
        let config = create_test_config();
        let mut monitor = NetworkMonitor::new(config).unwrap();
        
        let ip = IpAddr::V4(Ipv4Addr::new(5, 6, 7, 8));
        
        monitor.mark_suspicious(ip);
        
        let mut conn = create_test_connection();
        conn.remote_addr = ip;
        
        assert!(monitor.is_suspicious(&conn));
    }
    
    #[test]
    fn test_get_process_connections() {
        let config = create_test_config();
        let mut monitor = NetworkMonitor::new(config).unwrap();
        
        let mut conn1 = create_test_connection();
        conn1.id = "conn1".to_string();
        conn1.process_id = 1234;
        
        let mut conn2 = create_test_connection();
        conn2.id = "conn2".to_string();
        conn2.process_id = 1234;
        
        let mut conn3 = create_test_connection();
        conn3.id = "conn3".to_string();
        conn3.process_id = 5678;
        
        monitor.update_connection(conn1);
        monitor.update_connection(conn2);
        monitor.update_connection(conn3);
        
        let process_conns = monitor.get_process_connections(1234);
        assert_eq!(process_conns.len(), 2);
    }
}