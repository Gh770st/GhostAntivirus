#!/usr/bin/env python3

print("Fixing integration tests...")

# Fix integration_tests.rs - Update imports and constructor calls
with open('tests/integration_tests.rs', 'r') as f:
    content = f.read()

# Fix 1: Update imports to use correct module names
content = content.replace(
    '''use ghost_core::{
    scanner::Scanner,
    quarantine::QuarantineManager,
    analyzer::ThreatAnalyzer,
    network::NetworkMonitor,
    firewall::Firewall,
    config::Config,
    system::SystemMonitor,
};''',
    '''use ghost_core::{
    scanner::Scanner,
    quarantine::QuarantineManager,
    analyzer::BehaviorAnalyzer,
    network::NetworkMonitor,
    firewall::FirewallIntegration,
    config::Config,
    system::SystemMonitor,
};'''
)

# Fix 2: Replace Scanner::new() with Scanner::new(config)
content = content.replace(
    'let scanner = Scanner::new().unwrap();',
    'let config = Config::default();\n           let scanner = Scanner::new(config).unwrap();'
)

# Fix 3: Replace ThreatAnalyzer with BehaviorAnalyzer
content = content.replace(
    'ThreatAnalyzer',
    'BehaviorAnalyzer'
)

# Fix 4: Replace Firewall with FirewallIntegration
content = content.replace(
    'Firewall',
    'FirewallIntegration'
)

# Fix 5: Fix QuarantineManager::new() calls
content = content.replace(
    'QuarantineManager::new().unwrap()',
    'QuarantineManager::new(&std::path::PathBuf::from("./test_quarantine")).unwrap()'
)

# Fix 6: Fix Config::load() calls - it takes no args
content = content.replace(
    'Config::load().unwrap()',
    'Config::default()'
)

# Fix 7: Fix SystemMonitor::new() calls - it takes config
content = content.replace(
    'SystemMonitor::new().unwrap()',
    'SystemMonitor::new(&Config::default()).unwrap()'
)

# Fix 8: Fix NetworkMonitor::new() calls - it takes config
content = content.replace(
    'NetworkMonitor::new().unwrap()',
    'NetworkMonitor::new(Config::default()).unwrap()'
)

# Fix 9: Fix BehaviorAnalyzer::new() calls - it takes config
content = content.replace(
    'BehaviorAnalyzer::new().unwrap()',
    'BehaviorAnalyzer::new(&Config::default()).unwrap()'
)

# Fix 10: Fix FirewallIntegration::new() calls
content = content.replace(
    'FirewallIntegration::new().unwrap()',
    'FirewallIntegration::new().unwrap()'
)

# Write back
with open('tests/integration_tests.rs', 'w') as f:
    f.write(content)

print("✅ Fixed integration_tests.rs")

# Now fix other test files that might have similar issues
test_files = [
    'tests/scanner_tests.rs',
    'tests/quarantine_tests.rs',
    'tests/network_tests.rs',
    'tests/firewall_tests.rs',
    'tests/system_tests.rs'
]

for test_file in test_files:
    try:
        with open(test_file, 'r') as f:
            content = f.read()
        
        # Common fixes for all test files
        content = content.replace('Scanner::new()', 'Scanner::new(Config::default())')
        content = content.replace('QuarantineManager::new()', 'QuarantineManager::new(&std::path::PathBuf::from("./test_quarantine"))')
        content = content.replace('ThreatAnalyzer', 'BehaviorAnalyzer')
        content = content.replace('Firewall', 'FirewallIntegration')
        
        with open(test_file, 'w') as f:
            f.write(content)
        
        print(f"✅ Fixed {test_file}")
        
    except FileNotFoundError:
        print(f"⚠️  {test_file} not found, skipping")

print("\n✅ Integration test fixes completed!")