#!/usr/bin/env python3

with open('core/src/api/handlers/firewall.rs', 'r') as f:
    content = f.read()

# Fix 1: Remove unwrap_or on priority (it's already u32)
content = content.replace('payload.priority.unwrap_or(100)', 'payload.priority')

# Fix 2: Fix add_rule call - create FirewallRule struct first
old_add_rule = '''    // Add rule to firewall
    match engine.firewall.add_rule(
        payload.name,
        action,
        source_ip,
        dest_ip,
        payload.source_port,
        payload.dest_port,
        protocol,
        payload.priority,
    ) {'''

new_add_rule = '''    // Create firewall rule
    use uuid::Uuid;
    use chrono::Utc;
    let rule = crate::firewall::FirewallRule {
        id: Uuid::new_v4().to_string(),
        name: payload.name,
        action,
        direction: payload.direction,
        protocol,
        source_ip,
        source_port: payload.source_port,
        dest_ip,
        dest_port: payload.dest_port,
        priority: payload.priority,
        enabled: true,
        created_at: Utc::now(),
    };
    
    // Add rule to firewall
    match engine.firewall.add_rule(rule) {'''

content = content.replace(old_add_rule, new_add_rule)

# Fix 3: Fix update_rule call
old_update = '''    match engine.firewall.update_rule(&id, payload.name, action, payload.enabled.unwrap_or(true)) {'''

new_update = '''    // Get existing rule and update it
    let existing_rule = match engine.firewall.get_rule(&id) {
        Some(rule) => rule,
        None => {
            return error_response(StatusCode::NOT_FOUND, "Rule not found".to_string());
        }
    };
    
    let updated_rule = crate::firewall::FirewallRule {
        id: existing_rule.id,
        name: payload.name,
        action,
        direction: payload.direction,
        protocol: payload.protocol,
        source_ip: payload.source_ip.and_then(|ip| ip.parse().ok()),
        source_port: payload.source_port,
        dest_ip: payload.dest_ip.and_then(|ip| ip.parse().ok()),
        dest_port: payload.dest_port,
        priority: payload.priority,
        enabled: existing_rule.enabled,
        created_at: existing_rule.created_at,
    };
    
    match engine.firewall.update_rule(&id, updated_rule) {'''

content = content.replace(old_update, new_update)

with open('core/src/api/handlers/firewall.rs', 'w') as f:
    f.write(content)

print("Fixed firewall.rs")