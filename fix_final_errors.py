#!/usr/bin/env python3

# Fix 1: firewall.rs - Fix protocol type and add updated_at
with open('core/src/api/handlers/firewall.rs', 'r') as f:
    content = f.read()

# Fix first FirewallRule - remove Some() wrapper and add updated_at
content = content.replace(
    '''        protocol: Some(protocol),
        source_ip,
        source_port: payload.source_port,
        dest_ip,
        dest_port: payload.dest_port,
        priority: payload.priority,
        enabled: true,
        created_at: SystemTime::now(),
    };''',
    '''        protocol: Some(protocol),
        source_ip,
        source_port: payload.source_port,
        dest_ip,
        dest_port: payload.dest_port,
        priority: payload.priority,
        enabled: true,
        created_at: SystemTime::now(),
        updated_at: SystemTime::now(),
    };'''
)

# Fix second FirewallRule - convert protocol type and add updated_at
content = content.replace(
    '''        protocol: Some(payload.protocol),
        source_ip: payload.source_ip.and_then(|ip| ip.parse().ok()),
        source_port: payload.source_port,
        dest_ip: payload.dest_ip.and_then(|ip| ip.parse().ok()),
        dest_port: payload.dest_port,
        priority: payload.priority,
        enabled: existing_rule.enabled,
        created_at: existing_rule.created_at,
    };''',
    '''        protocol: Some(match payload.protocol {
            crate::api::models::Protocol::TCP => crate::network::Protocol::TCP,
            crate::api::models::Protocol::UDP => crate::network::Protocol::UDP,
            crate::api::models::Protocol::ICMP => crate::network::Protocol::ICMP,
        }),
        source_ip: payload.source_ip.and_then(|ip| ip.parse().ok()),
        source_port: payload.source_port,
        dest_ip: payload.dest_ip.and_then(|ip| ip.parse().ok()),
        dest_port: payload.dest_port,
        priority: payload.priority,
        enabled: existing_rule.enabled,
        created_at: existing_rule.created_at,
        updated_at: SystemTime::now(),
    };'''
)

with open('core/src/api/handlers/firewall.rs', 'w') as f:
    f.write(content)
print("Fixed firewall.rs - protocol types and updated_at")

# Fix 2: settings.rs - Fix scan_schedule type
with open('core/src/api/handlers/settings.rs', 'r') as f:
    content = f.read()

content = content.replace(
    'scan_schedule: None, // TODO: Add to ScannerConfig',
    'scan_schedule: String::new(), // TODO: Add to ScannerConfig'
)

with open('core/src/api/handlers/settings.rs', 'w') as f:
    f.write(content)
print("Fixed settings.rs - scan_schedule type")

print("\nAll final fixes applied!")