#!/usr/bin/env python3

with open('core/src/api/handlers/firewall.rs', 'r') as f:
    content = f.read()

# Fix 1: Check if SystemTime import was added correctly
if 'use std::time::SystemTime;' not in content:
    # Find the use statements section and add it properly
    lines = content.split('\n')
    for i, line in enumerate(lines):
        if line.startswith('use axum::{'):
            lines.insert(i, 'use std::time::SystemTime;')
            break
    content = '\n'.join(lines)

# Fix 2: Add RuleAction::Log pattern
content = content.replace(
    '''    let action = match payload.action {
        RuleAction::Allow => crate::firewall::RuleAction::Allow,
        RuleAction::Deny => crate::firewall::RuleAction::Deny,
    };''',
    '''    let action = match payload.action {
        RuleAction::Allow => crate::firewall::RuleAction::Allow,
        RuleAction::Deny => crate::firewall::RuleAction::Deny,
        RuleAction::Log => crate::firewall::RuleAction::Allow, // Map Log to Allow for now
    };'''
)

# Fix 3: Add Protocol::All pattern
content = content.replace(
    '''    let protocol = match payload.protocol {
        crate::api::models::Protocol::TCP => crate::network::Protocol::TCP,
        crate::api::models::Protocol::UDP => crate::network::Protocol::UDP,
        crate::api::models::Protocol::ICMP => crate::network::Protocol::ICMP,
    };''',
    '''    let protocol = match payload.protocol {
        crate::api::models::Protocol::TCP => crate::network::Protocol::TCP,
        crate::api::models::Protocol::UDP => crate::network::Protocol::UDP,
        crate::api::models::Protocol::ICMP => crate::network::Protocol::ICMP,
        crate::api::models::Protocol::All => crate::network::Protocol::TCP, // Default to TCP for All
    };'''
)

with open('core/src/api/handlers/firewall.rs', 'w') as f:
    f.write(content)

print("Fixed firewall.rs - added missing match patterns")