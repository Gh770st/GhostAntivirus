#!/usr/bin/env python3

with open('core/src/api/handlers/firewall.rs', 'r') as f:
    lines = f.readlines()

# Fix both match statements for action
for i in range(len(lines)):
    if 'RuleAction::Block => crate::firewall::Action::Block,' in lines[i]:
        # Check if next line is closing brace
        if i + 1 < len(lines) and '    };' in lines[i + 1]:
            # Insert the Log pattern before the closing brace
            lines.insert(i + 1, '        RuleAction::Log => crate::firewall::Action::Allow, // Map Log to Allow for now\n')

# Fix both match statements for protocol
for i in range(len(lines)):
    if 'Protocol::ICMP => Some(crate::network::Protocol::ICMP),' in lines[i]:
        # Check if next line is closing brace
        if i + 1 < len(lines) and '    };' in lines[i + 1]:
            # Insert the All pattern before the closing brace
            lines.insert(i + 1, '        Protocol::All => Some(crate::network::Protocol::TCP), // Default to TCP for All\n')

with open('core/src/api/handlers/firewall.rs', 'w') as f:
    f.writelines(lines)

print("Fixed firewall.rs - added missing match arms")