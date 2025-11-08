// Firewall handlers
use axum::{
    extract::{State, Path},
    response::Response,
    http::StatusCode,
    Json,
};
use chrono::{Utc, DateTime};

use crate::api::{
    AppState,
    models::{
        FirewallRule, FirewallRulesResponse, FirewallStatsResponse,
        AddFirewallRuleRequest, RuleAction, RuleDirection, Protocol,
    },
};
use super::{success_response, error_response};

/// Convert internal Action to API RuleAction
fn map_action(action: &crate::firewall::Action) -> RuleAction {
    match action {
        crate::firewall::Action::Allow => RuleAction::Allow,
        crate::firewall::Action::Deny => RuleAction::Deny,
        crate::firewall::Action::Block => RuleAction::Block,
        crate::firewall::Action::Log => RuleAction::Allow, // Map Log to Allow for API
    }
}

/// Convert internal Protocol to API Protocol
fn map_protocol(protocol: &Option<crate::network::Protocol>) -> Protocol {
    match protocol {
        Some(crate::network::Protocol::TCP) => Protocol::TCP,
        Some(crate::network::Protocol::UDP) => Protocol::UDP,
        Some(crate::network::Protocol::ICMP) => Protocol::ICMP,
        _ => Protocol::TCP, // Default
    }
}

/// List firewall rules
pub async fn list_rules(State(state): State<AppState>) -> Response {
    let engine = state.engine.read().await;
    
    // Get actual rules from firewall
    let firewall_rules = engine.firewall.get_rules();
    
    let rules: Vec<FirewallRule> = firewall_rules.iter().map(|rule| {
        FirewallRule {
            id: rule.id.clone(),
            name: rule.name.clone(),
            action: map_action(&rule.action),
            direction: RuleDirection::Inbound, // Default, could be extended
            protocol: map_protocol(&rule.protocol),
            source_ip: rule.source_ip.map(|ip| ip.to_string()),
            source_port: rule.source_port,
            dest_ip: rule.dest_ip.map(|ip| ip.to_string()),
            dest_port: rule.dest_port,
            enabled: rule.enabled,
            priority: rule.priority,
        }
    }).collect();
    
    let total = rules.len();

    let response = FirewallRulesResponse {
        rules,
        total,
    };

    success_response(response)
}

/// Add firewall rule
pub async fn add_rule(
    State(state): State<AppState>,
    Json(payload): Json<AddFirewallRuleRequest>,
) -> Response {
    let mut engine = state.engine.write().await;
    
    // Convert API action to internal action
    let action = match payload.action {
        RuleAction::Allow => crate::firewall::Action::Allow,
        RuleAction::Deny => crate::firewall::Action::Deny,
        RuleAction::Block => crate::firewall::Action::Block,
    };
    
    // Convert API protocol to internal protocol
    let protocol = match payload.protocol {
        Protocol::TCP => Some(crate::network::Protocol::TCP),
        Protocol::UDP => Some(crate::network::Protocol::UDP),
        Protocol::ICMP => Some(crate::network::Protocol::ICMP),
    };
    
    // Parse IPs if provided
    let source_ip = payload.source_ip.and_then(|ip| ip.parse().ok());
    let dest_ip = payload.dest_ip.and_then(|ip| ip.parse().ok());
    
    // Add rule to firewall
    match engine.firewall.add_rule(
        payload.name,
        action,
        source_ip,
        dest_ip,
        payload.source_port,
        payload.dest_port,
        protocol,
        payload.priority.unwrap_or(100),
    ) {
        Ok(rule_id) => {
            success_response(serde_json::json!({
                "message": "Rule added successfully",
                "rule_id": rule_id,
            }))
        }
        Err(e) => {
            error_response(StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to add rule: {}", e))
        }
    }
}

/// Update firewall rule
pub async fn update_rule(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<AddFirewallRuleRequest>,
) -> Response {
    let mut engine = state.engine.write().await;
    
    // Convert API action to internal action
    let action = match payload.action {
        RuleAction::Allow => crate::firewall::Action::Allow,
        RuleAction::Deny => crate::firewall::Action::Deny,
        RuleAction::Block => crate::firewall::Action::Block,
    };
    
    // Update rule
    match engine.firewall.update_rule(&id, payload.name, action, payload.enabled.unwrap_or(true)) {
        Ok(_) => {
            success_response(serde_json::json!({
                "message": "Rule updated successfully",
                "rule_id": id,
            }))
        }
        Err(e) => {
            error_response(StatusCode::NOT_FOUND, format!("Failed to update rule: {}", e))
        }
    }
}

/// Delete firewall rule
pub async fn delete_rule(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Response {
    let mut engine = state.engine.write().await;
    
    // Delete rule
    match engine.firewall.delete_rule(&id) {
        Ok(_) => {
            success_response(serde_json::json!({
                "message": "Rule deleted successfully",
                "rule_id": id,
            }))
        }
        Err(e) => {
            error_response(StatusCode::NOT_FOUND, format!("Failed to delete rule: {}", e))
        }
    }
}

/// Get firewall statistics
pub async fn get_stats(State(state): State<AppState>) -> Response {
    let engine = state.engine.read().await;
    
    // Get actual stats from firewall
    let stats = engine.firewall.get_statistics();
    
    let response = FirewallStatsResponse {
        packets_allowed: stats.packets_allowed,
        packets_blocked: stats.packets_blocked,
        packets_logged: stats.packets_logged,
        active_rules: stats.active_rules,
    };

    success_response(response)
}