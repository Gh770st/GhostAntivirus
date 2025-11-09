# Phase 2D - Implementation Plan for 9 TODO Items

## TODO Items Identified

### Critical TODOs (Functionality)
1. **scanner.rs:431** - Implement AI detection
2. **scanner.rs:481** - Implement real signature database lookup
3. **api/handlers/updates.rs:59** - Implement proper update application
4. **api/middleware.rs:85** - Implement actual rate limiting

### Medium TODOs (Features)
5. **monitor.rs:255** - Send alert or take action
6. **api/handlers.rs:26** - Calculate actual uptime

### Low TODOs (Configuration)
7. **api/handlers/settings.rs:24** - Add auto_scan to ScannerConfig
8. **api/handlers/settings.rs:25** - Add scan_schedule to ScannerConfig
9. **api/handlers/settings.rs:28** - Add notifications to Config

---

## Implementation Strategy

### Priority Order
1. **First**: Core functionality (scanner AI detection, signature database)
2. **Second**: API features (rate limiting, update application)
3. **Third**: Monitoring & alerts
4. **Fourth**: Configuration improvements

---

## Detailed Implementation Plan

### 1. Scanner AI Detection (scanner.rs:431) ⭐⭐⭐
**Location**: `scan_file_internal` method
**Current**: Always returns `Ok(None)` (no threats detected)
**Goal**: Implement ML-based threat detection

```rust
// Current:
// TODO: Implement AI detection
Ok(None)

// Plan:
// 1. Extract file features
// 2. Send to AI engine for analysis
// 3. Return threat info if detected
```

### 2. Signature Database Lookup (scanner.rs:481) ⭐⭐⭐
**Location**: `check_signature_database` method
**Current**: Always returns `Ok(None)`
**Goal**: Implement real signature matching

```rust
// Current:
// TODO: Implement real signature database lookup
Ok(None)

// Plan:
// 1. Load signature database
// 2. Check file hash against signatures
// 3. Return threat info if found
```

### 3. Rate Limiting (api/middleware.rs:85) ⭐⭐
**Current**: Always allows requests (no actual limiting)
**Goal**: Implement token bucket rate limiting

```rust
// Current:
// TODO: Implement actual rate limiting
Ok(next.run(req).await)

// Plan:
// 1. Create rate limiter with governor crate
// 2. Check rate limits
// 3. Return 429 if exceeded
```

### 4. Update Application (api/handlers/updates.rs:59) ⭐⭐
**Current**: Calls `check_for_updates` instead of applying updates
**Goal**: Implement actual update download and application

```rust
// Current:
// TODO: Implement proper update application
match engine.updater.check_for_updates().await {

// Plan:
// 1. Download update
// 2. Verify checksum
// 3. Apply update
// 4. Restart if needed
```

### 5. Monitor Alerts (monitor.rs:255) ⭐
**Current**: Just logs suspicious behavior
**Goal**: Send alerts via WebSocket/email

```rust
// Current:
// TODO: Send alert or take action
info!("Suspicious behavior detected");

// Plan:
// 1. Create alert system
// 2. Send WebSocket notifications
// 3. Log to quarantine if needed
```

### 6. Uptime Calculation (api/handlers.rs:26) ⭐
**Current**: Returns 0
**Goal**: Return actual system uptime

```rust
// Current:
uptime: 0, // TODO: Calculate actual uptime

// Plan:
// 1. Get process start time
// 2. Calculate uptime from current time
```

### 7-9. Configuration Improvements ⭐
**Current**: Hardcoded values or missing fields
**Goal**: Add proper configuration fields

```rust
// Plan:
// 1. Add fields to Config structs
// 2. Update serialization
// 3. Add validation
```

---

## Time Estimates

### Critical Items (4-6 hours)
1. AI Detection: 2-3 hours
2. Signature Database: 1-2 hours
3. Rate Limiting: 1 hour
4. Update Application: 1 hour

### Medium Items (1-2 hours)
5. Monitor Alerts: 1 hour
6. Uptime Calculation: 15 minutes

### Configuration Items (30 minutes)
7-9. Config Fields: 30 minutes

**Total Estimated Time**: 6-9 hours

---

## Implementation Approach

### Step 1: Core Scanner Functionality (Highest Impact)
- AI Detection
- Signature Database
- Test with real files

### Step 2: API Improvements
- Rate Limiting
- Update Application
- Uptime Calculation

### Step 3: Alert System
- WebSocket notifications
- Monitor alerts

### Step 4: Configuration
- Add missing config fields
- Update handlers

---

## Success Criteria

### After Implementation:
1. ✅ Scanner can detect real threats (not just return None)
2. ✅ Rate limiting prevents API abuse
3. ✅ Updates can be applied properly
4. ✅ Alerts are sent for suspicious activity
5. ✅ All 9 TODO items resolved
6. ✅ Tests still pass (100%)
7. ✅ Zero warnings maintained

---

## Testing Strategy

### For Each TODO:
1. Implement the functionality
2. Add unit tests
3. Add integration tests
4. Verify no regressions

### Critical Tests:
- AI detection with known threats
- Signature matching
- Rate limiting behavior
- Update download/apply
- Alert notifications

---

## Risks and Mitigations

### Risk 1: Breaking Existing Tests
- **Mitigation**: Run tests after each TODO fix
- **Rollback**: Keep original code as comments temporarily

### Risk 2: Performance Impact
- **Mitigation**: Benchmark critical paths
- **Optimization**: Profile if needed

### Risk 3: External Dependencies
- **Mitigation**: Use existing crates (governor, etc.)
- **Fallback**: Implement simple versions first

---

## Next Steps

### Start with Highest Priority:
1. **AI Detection** - Core antivirus functionality
2. **Signature Database** - Traditional detection
3. **Test Both** - Ensure threat detection works

### Then Move to:
4. **Rate Limiting** - API protection
5. **Update Application** - Maintenance
6. **Remaining Items** - Polish and features

---

**Status**: Ready to start implementation
**First Target**: AI Detection (scanner.rs:431)
**Estimated Duration**: 6-9 hours total