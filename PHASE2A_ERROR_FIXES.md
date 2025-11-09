# Phase 2A - Compilation Error Fixes

## Summary
- **Total Errors**: 22
- **Warnings**: 24 (non-critical)

## Error Categories

### 1. Type Mismatches (3 errors)
- **E0308**: ScanType mismatch between models and scanner
- **E0277**: Write trait not implemented for File
- **E0382**: Borrow after move in analyzer

### 2. Method Signature Errors (6 errors)
- **E0599**: `unwrap_or` on bool (scanner.rs:60)
- **E0599**: `unwrap_or` on u32 (firewall.rs:106)
- **E0061**: add_rule takes 1 arg, 8 supplied (firewall.rs:98)
- **E0061**: update_rule takes 2 args, 4 supplied (firewall.rs:136)
- **E0061**: System::uptime() needs &self (system.rs:112, 175)

### 3. Missing Fields/Methods (3 errors)
- **E0609**: No field `enabled` on AddFirewallRuleRequest
- **E0599**: No method `iter` on Networks (needs NetworksExt trait)

### 4. Import Issues (1 error)
- Missing NetworksExt trait import in system.rs

## Fix Plan

### Fix 1: Remove unwrap_or on bool (scanner.rs:60)
```rust
// Before:
payload.deep_scan.unwrap_or(false)

// After:
payload.deep_scan  // Already a bool, no unwrap_or needed
```

### Fix 2: Convert ScanType (scanner.rs:59)
```rust
// Add conversion function or match the types
let scan_type = match payload.scan_type {
    models::ScanType::Quick => scanner::ScanType::Quick,
    models::ScanType::Full => scanner::ScanType::Full,
    models::ScanType::Custom => scanner::ScanType::Custom,
};
```

### Fix 3: Remove unwrap_or on u32 (firewall.rs:106)
```rust
// Before:
payload.priority.unwrap_or(100)

// After:
payload.priority  // Already a u32
```

### Fix 4: Fix add_rule call (firewall.rs:98)
```rust
// Create FirewallRule struct first
let rule = FirewallRule {
    id: Uuid::new_v4().to_string(),
    name: payload.name,
    action,
    direction: payload.direction,
    protocol: payload.protocol,
    source_ip: payload.source_ip,
    source_port: payload.source_port,
    dest_ip: payload.dest_ip,
    dest_port: payload.dest_port,
    priority: payload.priority,
    enabled: true,
    created_at: Utc::now(),
};
engine.firewall.add_rule(rule)
```

### Fix 5: Fix update_rule call (firewall.rs:136)
```rust
// Get existing rule and update it
let mut rule = engine.firewall.get_rule(&id)?;
rule.name = payload.name;
rule.action = action;
engine.firewall.update_rule(&id, rule)
```

### Fix 6: Remove enabled field access (firewall.rs:136)
```rust
// Remove payload.enabled.unwrap_or(true) - not in struct
```

### Fix 7: Add NetworksExt import (system.rs)
```rust
use sysinfo::{System, SystemExt, CpuExt, DiskExt, NetworkExt, NetworksExt, ProcessExt};
```

### Fix 8: Fix System::uptime() calls (system.rs:112, 175)
```rust
// Before:
let uptime = System::uptime();

// After:
let uptime = system.uptime();  // Use instance method
```

### Fix 9: Fix Write trait for File (updater.rs:297)
```rust
// Before:
response.copy_to(&mut file)?;

// After:
use std::io::Write;
let mut content = Vec::new();
response.copy_to(&mut content)?;
file.write_all(&content)?;
```

### Fix 10: Fix borrow after move (analyzer.rs:374)
```rust
// Before:
threats.push(threat);
info!("Threat added: {} - {}", threat.threat_name, threat.file_path.display());

// After:
let threat_name = threat.threat_name.clone();
let file_path = threat.file_path.clone();
threats.push(threat);
info!("Threat added: {} - {}", threat_name, file_path.display());
```

## Execution Order
1. Fix imports first (system.rs)
2. Fix simple type issues (unwrap_or removals)
3. Fix method signatures (firewall, scanner)
4. Fix complex issues (Write trait, borrow)