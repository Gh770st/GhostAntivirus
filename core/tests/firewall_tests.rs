//! Firewall Module Tests
//! 
//! Comprehensive test suite for the firewall module.
//! Tests rule management, blocking functionality, and rule validation.

use ghost_core::firewall::{Firewall, FirewallRule, RuleAction, RuleDirection, Protocol};
use std::net::IpAddr;
use std::str::FromStr;

#[cfg(test)]
mod firewall_tests {
    use super::*;

    /// Test: Firewall initialization
    #[test]
    fn test_firewall_new() {
        let firewall = Firewall::new();
        assert!(firewall.is_ok(), "Firewall should initialize successfully");
    }

    /// Test: Add firewall rule
    #[test]
    fn test_add_rule() {
        let firewall = Firewall::new().unwrap();
        
        let rule = FirewallRule {
            id: None,
            name: "Test Rule".to_string(),
            action: RuleAction::Block,
            direction: RuleDirection::Inbound,
            protocol: Protocol::TCP,
            source_ip: Some(IpAddr::from_str("192.168.1.100").unwrap()),
            source_port: None,
            dest_ip: None,
            dest_port: Some(8080),
            enabled: true,
        };
        
        let result = firewall.add_rule(rule);
        assert!(result.is_ok(), "Should add rule successfully");
        
        let rule_id = result.unwrap();
        assert!(rule_id > 0, "Should return valid rule ID");
    }

    /// Test: Get all rules
    #[test]
    fn test_get_rules() {
        let firewall = Firewall::new().unwrap();
        
        // Add a test rule
        let rule = FirewallRule {
            id: None,
            name: "Test Rule".to_string(),
            action: RuleAction::Allow,
            direction: RuleDirection::Outbound,
            protocol: Protocol::UDP,
            source_ip: None,
            source_port: None,
            dest_ip: Some(IpAddr::from_str("8.8.8.8").unwrap()),
            dest_port: Some(53),
            enabled: true,
        };
        
        firewall.add_rule(rule).unwrap();
        
        let rules = firewall.get_rules();
        assert!(rules.is_ok(), "Should retrieve rules successfully");
        assert!(rules.unwrap().len() > 0, "Should have at least one rule");
    }

    /// Test: Get rule by ID
    #[test]
    fn test_get_rule_by_id() {
        let firewall = Firewall::new().unwrap();
        
        let rule = FirewallRule {
            id: None,
            name: "Test Rule".to_string(),
            action: RuleAction::Block,
            direction: RuleDirection::Inbound,
            protocol: Protocol::TCP,
            source_ip: None,
            source_port: None,
            dest_ip: None,
            dest_port: Some(22),
            enabled: true,
        };
        
        let rule_id = firewall.add_rule(rule).unwrap();
        
        let retrieved_rule = firewall.get_rule(rule_id);
        assert!(retrieved_rule.is_ok(), "Should retrieve rule by ID");
        
        let rule = retrieved_rule.unwrap();
        assert_eq!(rule.name, "Test Rule", "Rule name should match");
        assert_eq!(rule.dest_port, Some(22), "Rule port should match");
    }

    /// Test: Update rule
    #[test]
    fn test_update_rule() {
        let firewall = Firewall::new().unwrap();
        
        let rule = FirewallRule {
            id: None,
            name: "Original Rule".to_string(),
            action: RuleAction::Allow,
            direction: RuleDirection::Inbound,
            protocol: Protocol::TCP,
            source_ip: None,
            source_port: None,
            dest_ip: None,
            dest_port: Some(80),
            enabled: true,
        };
        
        let rule_id = firewall.add_rule(rule).unwrap();
        
        let updated_rule = FirewallRule {
            id: Some(rule_id),
            name: "Updated Rule".to_string(),
            action: RuleAction::Block,
            direction: RuleDirection::Inbound,
            protocol: Protocol::TCP,
            source_ip: None,
            source_port: None,
            dest_ip: None,
            dest_port: Some(80),
            enabled: false,
        };
        
        let result = firewall.update_rule(rule_id, updated_rule);
        assert!(result.is_ok(), "Should update rule successfully");
        
        let rule = firewall.get_rule(rule_id).unwrap();
        assert_eq!(rule.name, "Updated Rule", "Rule should be updated");
        assert_eq!(rule.action, RuleAction::Block, "Action should be updated");
        assert!(!rule.enabled, "Rule should be disabled");
    }

    /// Test: Delete rule
    #[test]
    fn test_delete_rule() {
        let firewall = Firewall::new().unwrap();
        
        let rule = FirewallRule {
            id: None,
            name: "Temporary Rule".to_string(),
            action: RuleAction::Block,
            direction: RuleDirection::Inbound,
            protocol: Protocol::TCP,
            source_ip: None,
            source_port: None,
            dest_ip: None,
            dest_port: Some(9999),
            enabled: true,
        };
        
        let rule_id = firewall.add_rule(rule).unwrap();
        
        let result = firewall.delete_rule(rule_id);
        assert!(result.is_ok(), "Should delete rule successfully");
        
        let retrieved = firewall.get_rule(rule_id);
        assert!(retrieved.is_err(), "Rule should not exist after deletion");
    }

    /// Test: Enable/disable rule
    #[test]
    fn test_toggle_rule() {
        let firewall = Firewall::new().unwrap();
        
        let rule = FirewallRule {
            id: None,
            name: "Toggle Rule".to_string(),
            action: RuleAction::Block,
            direction: RuleDirection::Inbound,
            protocol: Protocol::TCP,
            source_ip: None,
            source_port: None,
            dest_ip: None,
            dest_port: Some(3389),
            enabled: true,
        };
        
        let rule_id = firewall.add_rule(rule).unwrap();
        
        // Disable
        let result = firewall.disable_rule(rule_id);
        assert!(result.is_ok(), "Should disable rule successfully");
        
        let rule = firewall.get_rule(rule_id).unwrap();
        assert!(!rule.enabled, "Rule should be disabled");
        
        // Enable
        let result = firewall.enable_rule(rule_id);
        assert!(result.is_ok(), "Should enable rule successfully");
        
        let rule = firewall.get_rule(rule_id).unwrap();
        assert!(rule.enabled, "Rule should be enabled");
    }

    /// Test: Check if connection is blocked
    #[test]
    fn test_is_blocked() {
        let firewall = Firewall::new().unwrap();
        
        // Add blocking rule for specific IP
        let rule = FirewallRule {
            id: None,
            name: "Block Malicious IP".to_string(),
            action: RuleAction::Block,
            direction: RuleDirection::Inbound,
            protocol: Protocol::TCP,
            source_ip: Some(IpAddr::from_str("192.168.1.100").unwrap()),
            source_port: None,
            dest_ip: None,
            dest_port: None,
            enabled: true,
        };
        
        firewall.add_rule(rule).unwrap();
        
        let blocked = firewall.is_blocked(
            IpAddr::from_str("192.168.1.100").unwrap(),
            None,
            RuleDirection::Inbound,
            Protocol::TCP
        );
        
        assert!(blocked, "Connection should be blocked by rule");
    }

    /// Test: Rule validation
    #[test]
    fn test_validate_rule() {
        let firewall = Firewall::new().unwrap();
        
        // Valid rule
        let valid_rule = FirewallRule {
            id: None,
            name: "Valid Rule".to_string(),
            action: RuleAction::Allow,
            direction: RuleDirection::Outbound,
            protocol: Protocol::TCP,
            source_ip: None,
            source_port: None,
            dest_ip: Some(IpAddr::from_str("8.8.8.8").unwrap()),
            dest_port: Some(53),
            enabled: true,
        };
        
        assert!(firewall.validate_rule(&valid_rule).is_ok(), "Valid rule should pass validation");
        
        // Invalid rule (empty name)
        let invalid_rule = FirewallRule {
            id: None,
            name: "".to_string(),
            action: RuleAction::Block,
            direction: RuleDirection::Inbound,
            protocol: Protocol::TCP,
            source_ip: None,
            source_port: None,
            dest_ip: None,
            dest_port: None,
            enabled: true,
        };
        
        assert!(firewall.validate_rule(&invalid_rule).is_err(), "Invalid rule should fail validation");
    }

    /// Test: Get rules by action
    #[test]
    fn test_get_rules_by_action() {
        let firewall = Firewall::new().unwrap();
        
        // Add block rule
        let block_rule = FirewallRule {
            id: None,
            name: "Block Rule".to_string(),
            action: RuleAction::Block,
            direction: RuleDirection::Inbound,
            protocol: Protocol::TCP,
            source_ip: None,
            source_port: None,
            dest_ip: None,
            dest_port: Some(445),
            enabled: true,
        };
        
        firewall.add_rule(block_rule).unwrap();
        
        let block_rules = firewall.get_rules_by_action(RuleAction::Block);
        assert!(block_rules.is_ok(), "Should retrieve rules by action");
        assert!(block_rules.unwrap().len() > 0, "Should have at least one block rule");
    }

    /// Test: Get rules by protocol
    #[test]
    fn test_get_rules_by_protocol() {
        let firewall = Firewall::new().unwrap();
        
        let tcp_rule = FirewallRule {
            id: None,
            name: "TCP Rule".to_string(),
            action: RuleAction::Allow,
            direction: RuleDirection::Outbound,
            protocol: Protocol::TCP,
            source_ip: None,
            source_port: None,
            dest_ip: None,
            dest_port: Some(443),
            enabled: true,
        };
        
        firewall.add_rule(tcp_rule).unwrap();
        
        let tcp_rules = firewall.get_rules_by_protocol(Protocol::TCP);
        assert!(tcp_rules.is_ok(), "Should retrieve rules by protocol");
        assert!(tcp_rules.unwrap().len() > 0, "Should have at least one TCP rule");
    }

    /// Test: Clear all rules
    #[test]
    fn test_clear_rules() {
        let firewall = Firewall::new().unwrap();
        
        // Add multiple rules
        for i in 0..5 {
            let rule = FirewallRule {
                id: None,
                name: format!("Rule {}", i),
                action: RuleAction::Block,
                direction: RuleDirection::Inbound,
                protocol: Protocol::TCP,
                source_ip: None,
                source_port: None,
                dest_ip: None,
                dest_port: Some(8000 + i),
                enabled: true,
            };
            firewall.add_rule(rule).unwrap();
        }
        
        let result = firewall.clear_rules();
        assert!(result.is_ok(), "Should clear all rules successfully");
        
        let rules = firewall.get_rules().unwrap();
        assert_eq!(rules.len(), 0, "Should have no rules after clearing");
    }

    /// Test: Export/import rules
    #[test]
    fn test_export_import_rules() {
        let firewall = Firewall::new().unwrap();
        
        // Add test rules
        let rule = FirewallRule {
            id: None,
            name: "Export Test".to_string(),
            action: RuleAction::Block,
            direction: RuleDirection::Inbound,
            protocol: Protocol::TCP,
            source_ip: None,
            source_port: None,
            dest_ip: None,
            dest_port: Some(1234),
            enabled: true,
        };
        
        firewall.add_rule(rule).unwrap();
        
        // Export rules
        let exported = firewall.export_rules();
        assert!(exported.is_ok(), "Should export rules successfully");
        
        // Clear and import
        firewall.clear_rules().unwrap();
        
        let result = firewall.import_rules(exported.unwrap());
        assert!(result.is_ok(), "Should import rules successfully");
        
        let rules = firewall.get_rules().unwrap();
        assert!(rules.len() > 0, "Should have rules after import");
    }

    /// Test: Rule priority
    #[test]
    fn test_rule_priority() {
        let firewall = Firewall::new().unwrap();
        
        // Add rules with different priorities
        let high_priority = FirewallRule {
            id: None,
            name: "High Priority".to_string(),
            action: RuleAction::Block,
            direction: RuleDirection::Inbound,
            protocol: Protocol::TCP,
            source_ip: Some(IpAddr::from_str("192.168.1.100").unwrap()),
            source_port: None,
            dest_ip: None,
            dest_port: None,
            enabled: true,
        };
        
        let low_priority = FirewallRule {
            id: None,
            name: "Low Priority".to_string(),
            action: RuleAction::Allow,
            direction: RuleDirection::Inbound,
            protocol: Protocol::TCP,
            source_ip: None,
            source_port: None,
            dest_ip: None,
            dest_port: None,
            enabled: true,
        };
        
        firewall.add_rule(high_priority).unwrap();
        firewall.add_rule(low_priority).unwrap();
        
        // More specific rule should take precedence
        let blocked = firewall.is_blocked(
            IpAddr::from_str("192.168.1.100").unwrap(),
            None,
            RuleDirection::Inbound,
            Protocol::TCP
        );
        
        assert!(blocked, "More specific rule should take precedence");
    }

    /// Test: Concurrent rule operations
    #[test]
    fn test_concurrent_operations() {
        use std::sync::Arc;
        use std::thread;
        
        let firewall = Arc::new(Firewall::new().unwrap());
        let mut handles = vec![];
        
        for i in 0..10 {
            let firewall_clone = Arc::clone(&firewall);
            let handle = thread::spawn(move || {
                let rule = FirewallRule {
                    id: None,
                    name: format!("Concurrent Rule {}", i),
                    action: RuleAction::Block,
                    direction: RuleDirection::Inbound,
                    protocol: Protocol::TCP,
                    source_ip: None,
                    source_port: None,
                    dest_ip: None,
                    dest_port: Some(9000 + i),
                    enabled: true,
                };
                
                firewall_clone.add_rule(rule)
            });
            handles.push(handle);
        }
        
        for handle in handles {
            assert!(handle.join().unwrap().is_ok(), "Concurrent operations should succeed");
        }
        
        let rules = firewall.get_rules().unwrap();
        assert!(rules.len() >= 10, "Should have at least 10 rules");
    }
}