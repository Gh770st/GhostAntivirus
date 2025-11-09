//! Firewall Integration Module
//! 
//! Manages firewall rules and packet filtering.

use std::collections::HashMap;
use std::net::IpAddr;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use anyhow::{Result, bail};
use log::{info, warn, debug};

use crate::network::{Connection, Protocol};

/// Firewall rule identifier
pub type RuleId = String;

/// Firewall action
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
    Allow,
    Deny,
    Block,
    Log,
}

/// Firewall rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallRule {
    pub id: RuleId,
    pub name: String,
    pub action: Action,
    pub source_ip: Option<IpAddr>,
    pub dest_ip: Option<IpAddr>,
    pub source_port: Option<u16>,
    pub dest_port: Option<u16>,
    pub protocol: Option<Protocol>,
    pub enabled: bool,
    pub priority: u32,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

/// Firewall statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallStats {
    pub total_rules: usize,
    pub active_rules: usize,
    pub packets_allowed: u64,
    pub packets_denied: u64,
    pub packets_blocked: u64,
    pub packets_logged: u64,
}

/// Firewall Integration Manager
pub struct FirewallIntegration {
    rules: HashMap<RuleId, FirewallRule>,
    stats: FirewallStats,
    enabled: bool,
    default_action: Action,
}

impl FirewallIntegration {
    /// Create new firewall integration
    pub fn new() -> Result<Self> {
        info!("Initializing Firewall Integration");
        
        let mut rules = HashMap::new();
        
        // Add default rules
        for rule in Self::default_rules() {
            rules.insert(rule.id.clone(), rule);
        }
        
        Ok(Self {
            rules,
            stats: FirewallStats::default(),
            enabled: true,
            default_action: Action::Allow,
        })
    }
    
    /// Get default firewall rules
    fn default_rules() -> Vec<FirewallRule> {
        let now = SystemTime::now();
        
        vec![
            FirewallRule {
                id: "rule_001".to_string(),
                name: "Allow HTTP".to_string(),
                action: Action::Allow,
                source_ip: None,
                dest_ip: None,
                source_port: None,
                dest_port: Some(80),
                protocol: Some(Protocol::TCP),
                enabled: true,
                priority: 100,
                created_at: now,
                updated_at: now,
            },
            FirewallRule {
                id: "rule_002".to_string(),
                name: "Allow HTTPS".to_string(),
                action: Action::Allow,
                source_ip: None,
                dest_ip: None,
                source_port: None,
                dest_port: Some(443),
                protocol: Some(Protocol::TCP),
                enabled: true,
                priority: 100,
                created_at: now,
                updated_at: now,
            },
            FirewallRule {
                id: "rule_003".to_string(),
                name: "Allow DNS".to_string(),
                action: Action::Allow,
                source_ip: None,
                dest_ip: None,
                source_port: None,
                dest_port: Some(53),
                protocol: Some(Protocol::UDP),
                enabled: true,
                priority: 100,
                created_at: now,
                updated_at: now,
            },
        ]
    }
    
    /// Add a firewall rule
    pub fn add_rule(&mut self, rule: FirewallRule) -> Result<()> {
        info!("Adding firewall rule: {} ({})", rule.name, rule.id);
        
        if self.rules.contains_key(&rule.id) {
            bail!("Rule with ID {} already exists", rule.id);
        }
        
        self.rules.insert(rule.id.clone(), rule);
        self.update_stats();
        
        Ok(())
    }
    
    /// Remove a firewall rule
    pub fn remove_rule(&mut self, id: &RuleId) -> Result<()> {
        info!("Removing firewall rule: {}", id);
        
        if self.rules.remove(id).is_none() {
            bail!("Rule with ID {} not found", id);
        }
        
        self.update_stats();
        Ok(())
    }
    
    /// Update a firewall rule
    pub fn update_rule(&mut self, id: &RuleId, updated_rule: FirewallRule) -> Result<()> {
        info!("Updating firewall rule: {}", id);
        
        if !self.rules.contains_key(id) {
            bail!("Rule with ID {} not found", id);
        }
        
        let mut rule = updated_rule;
        rule.id = id.clone();
        rule.updated_at = SystemTime::now();
        
        self.rules.insert(id.clone(), rule);
        self.update_stats();
        
        Ok(())
    }
    
    /// Get a firewall rule
    pub fn get_rule(&self, id: &RuleId) -> Option<&FirewallRule> {
        self.rules.get(id)
    }
    
    /// Get all firewall rules
    pub fn get_all_rules(&self) -> Vec<FirewallRule> {
        let mut rules: Vec<FirewallRule> = self.rules.values().cloned().collect();
        rules.sort_by(|a, b| a.priority.cmp(&b.priority));
        rules
    }
    
    /// Check connection against firewall rules
    pub fn check_connection(&mut self, conn: &Connection) -> Action {
        if !self.enabled {
            return Action::Allow;
        }
        
        debug!("Checking connection against firewall rules: {:?}", conn.id);
        
        // Get rules sorted by priority
        let mut rules: Vec<&FirewallRule> = self.rules.values()
            .filter(|r| r.enabled)
            .collect();
        rules.sort_by(|a, b| a.priority.cmp(&b.priority));
        
        // Check each rule
        for rule in rules {
            if self.rule_matches(rule, conn) {
                debug!("Connection matched rule: {} ({})", rule.name, rule.id);
                
                // Update statistics
                match rule.action {
                    Action::Allow => self.stats.packets_allowed += 1,
                    Action::Deny => self.stats.packets_denied += 1,
                    Action::Block => self.stats.packets_blocked += 1,
                    Action::Log => self.stats.packets_logged += 1,
                }
                
                return rule.action.clone();
            }
        }
        
        // No rule matched, use default action
        debug!("No rule matched, using default action: {:?}", self.default_action);
        self.default_action.clone()
    }
    
    /// Check if a rule matches a connection
    fn rule_matches(&self, rule: &FirewallRule, conn: &Connection) -> bool {
        // Check protocol
        if let Some(ref rule_protocol) = rule.protocol {
            if rule_protocol != &conn.protocol {
                return false;
            }
        }
        
        // Check source IP
        if let Some(ref rule_source_ip) = rule.source_ip {
            if rule_source_ip != &conn.local_addr {
                return false;
            }
        }
        
        // Check destination IP
        if let Some(ref rule_dest_ip) = rule.dest_ip {
            if rule_dest_ip != &conn.remote_addr {
                return false;
            }
        }
        
        // Check source port
        if let Some(rule_source_port) = rule.source_port {
            if rule_source_port != conn.local_port {
                return false;
            }
        }
        
        // Check destination port
        if let Some(rule_dest_port) = rule.dest_port {
            if rule_dest_port != conn.remote_port {
                return false;
            }
        }
        
        true
    }
    
    /// Enable firewall
    pub fn enable(&mut self) {
        info!("Enabling firewall");
        self.enabled = true;
    }
    
    /// Disable firewall
    pub fn disable(&mut self) {
        warn!("Disabling firewall");
        self.enabled = false;
    }
    
    /// Check if firewall is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Set default action
    pub fn set_default_action(&mut self, action: Action) {
        info!("Setting default firewall action to: {:?}", action);
        self.default_action = action;
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> FirewallStats {
        self.stats.clone()
    }
    
    /// Update statistics
    fn update_stats(&mut self) {
        self.stats.total_rules = self.rules.len();
        self.stats.active_rules = self.rules.values()
            .filter(|r| r.enabled)
            .count();
    }
    
    /// Reset statistics
    pub fn reset_statistics(&mut self) {
        info!("Resetting firewall statistics");
        self.stats = FirewallStats::default();
        self.update_stats();
    }
    
    /// Get all rules (alias for get_all_rules for API compatibility)
    pub fn get_rules(&self) -> Vec<FirewallRule> {
        self.get_all_rules()
    }
    
    /// Add a new rule with parameters (helper for API)
    pub fn add_rule_with_params(
        &mut self,
        name: String,
        action: Action,
        source_ip: Option<std::net::IpAddr>,
        dest_ip: Option<std::net::IpAddr>,
        source_port: Option<u16>,
        dest_port: Option<u16>,
        protocol: Option<crate::network::Protocol>,
        priority: u32,
    ) -> Result<RuleId> {
        use uuid::Uuid;
        use std::time::SystemTime;
        
        let rule_id = Uuid::new_v4().to_string();
        let now = SystemTime::now();
        
        let rule = FirewallRule {
            id: rule_id.clone(),
            name,
            action,
            source_ip,
            dest_ip,
            source_port,
            dest_port,
            protocol,
            enabled: true,
            priority,
            created_at: now,
            updated_at: now,
        };
        
        self.rules.insert(rule_id.clone(), rule);
        self.update_stats();
        
        info!("Added firewall rule: {}", rule_id);
        Ok(rule_id)
    }
    
    /// Update an existing rule (simplified for API)
    pub fn update_rule_simple(
        &mut self,
        id: &RuleId,
        name: String,
        action: Action,
        enabled: bool,
    ) -> Result<()> {
        use std::time::SystemTime;
        
        if let Some(rule) = self.rules.get_mut(id) {
            rule.name = name;
            rule.action = action;
            rule.enabled = enabled;
            rule.updated_at = SystemTime::now();
            
            info!("Updated firewall rule: {}", id);
            Ok(())
        } else {
            bail!("Rule not found: {}", id)
        }
    }
    
    /// Delete a rule (alias for remove_rule for API compatibility)
    pub fn delete_rule(&mut self, id: &RuleId) -> Result<()> {
        self.remove_rule(id)
    }
}

impl Default for FirewallStats {
    fn default() -> Self {
        Self {
            total_rules: 0,
            active_rules: 0,
            packets_allowed: 0,
            packets_denied: 0,
            packets_blocked: 0,
            packets_logged: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;
    
    fn create_test_connection() -> Connection {
        Connection {
            id: "test_conn".to_string(),
            process_id: 1234,
            process_name: "test.exe".to_string(),
            local_addr: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 10)),
            local_port: 50000,
            remote_addr: IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
            remote_port: 443,
            protocol: Protocol::TCP,
            state: crate::network::ConnectionState::Established,
            bytes_sent: 0,
            bytes_received: 0,
            established_at: SystemTime::now(),
        }
    }
    
    #[test]
    fn test_firewall_creation() {
        let firewall = FirewallIntegration::new();
        assert!(firewall.is_ok());
    }
    
    #[test]
    fn test_default_rules() {
        let firewall = FirewallIntegration::new().unwrap();
        let rules = firewall.get_all_rules();
        assert!(rules.len() >= 3); // At least HTTP, HTTPS, DNS
    }
    
    #[test]
    fn test_add_remove_rule() {
        let mut firewall = FirewallIntegration::new().unwrap();
        
        let rule = FirewallRule {
            id: "test_rule".to_string(),
            name: "Test Rule".to_string(),
            action: Action::Block,
            source_ip: None,
            dest_ip: None,
            source_port: None,
            dest_port: Some(8080),
            protocol: Some(Protocol::TCP),
            enabled: true,
            priority: 50,
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        };
        
        firewall.add_rule(rule.clone()).unwrap();
        assert!(firewall.get_rule(&rule.id).is_some());
        
        firewall.remove_rule(&rule.id).unwrap();
        assert!(firewall.get_rule(&rule.id).is_none());
    }
    
    #[test]
    fn test_check_connection() {
        let mut firewall = FirewallIntegration::new().unwrap();
        let conn = create_test_connection();
        
        let action = firewall.check_connection(&conn);
        assert_eq!(action, Action::Allow); // HTTPS should be allowed by default
    }
    
    #[test]
    fn test_enable_disable() {
        let mut firewall = FirewallIntegration::new().unwrap();
        
        assert!(firewall.is_enabled());
        
        firewall.disable();
        assert!(!firewall.is_enabled());
        
        firewall.enable();
        assert!(firewall.is_enabled());
    }
    
    #[test]
    fn test_rule_matching() {
        let mut firewall = FirewallIntegration::new().unwrap();
        
        // Add a blocking rule for port 8080
        let rule = FirewallRule {
            id: "block_8080".to_string(),
            name: "Block 8080".to_string(),
            action: Action::Block,
            source_ip: None,
            dest_ip: None,
            source_port: None,
            dest_port: Some(8080),
            protocol: Some(Protocol::TCP),
            enabled: true,
            priority: 10,
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        };
        
        firewall.add_rule(rule).unwrap();
        
        let mut conn = create_test_connection();
        conn.remote_port = 8080;
        
        let action = firewall.check_connection(&conn);
        assert_eq!(action, Action::Block);
    }
}