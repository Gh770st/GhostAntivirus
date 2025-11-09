# Phase 2B - Fix Plan for 4 Failing Tests

## Test Failures Analysis

### 1. quarantine::tests::test_list_files ⚠️
**Error**: `assertion failed: left == right (left: 1, right: 3)`
**Expected**: 3 files in quarantine
**Actual**: 1 file in quarantine

**Root Cause**: The test creates 3 files and quarantines them, but only 1 appears in the list.

**Possible Issues**:
- Files might be overwriting each other (same quarantine path)
- list_files() might not be reading all files
- Quarantine operation might be failing silently for some files

**Fix Strategy**:
1. Check if quarantine_file() is properly handling multiple files
2. Verify list_files() implementation
3. Add debug output to see what's happening
4. Ensure unique quarantine paths for each file

---

### 2. scanner::tests::test_file_hash_calculation ⚠️
**Error**: Hash mismatch
**Expected**: `6a9ee6bf847a30bff0806c59430de0d749d4ea5a9b5d88b85f9f376d6d6674a1`
**Actual**: `6ae8a75555209fd6c44157c0aed8016e763ff435a19cf186f76863140143ff72`

**Root Cause**: The hash of "test content" doesn't match expected value.

**Possible Issues**:
- Test file content might include extra data (newlines, etc.)
- Hash algorithm might be different than expected
- Expected hash might be incorrect

**Fix Strategy**:
1. Calculate the correct SHA256 hash of "test content"
2. Update the expected hash in the test
3. Or verify the file content is exactly what we expect

**Verification**:
```bash
echo -n "test content" | sha256sum
# Should give us the correct hash
```

---

### 3. tests::test_engine_creation ⚠️
**Error**: `Cannot drop a runtime in a context where blocking is not allowed`

**Root Cause**: Tokio runtime lifecycle issue - trying to drop a runtime from within an async context.

**Possible Issues**:
- GhostEngine::new() creates a nested runtime
- Test is running in tokio::test which already has a runtime
- Runtime is being dropped while still in async context

**Fix Strategy**:
1. Change GhostEngine::new() to not create its own runtime
2. Or change test to not use #[tokio::test]
3. Or use Handle::current() instead of creating new runtime

---

### 4. tests::test_engine_status ⚠️
**Error**: Same as test_engine_creation

**Root Cause**: Same issue - runtime lifecycle problem

**Fix Strategy**: Same as test_engine_creation

---

## Implementation Order

### Step 1: Fix Hash Test (Easiest - 5 minutes)
- Calculate correct hash
- Update test expectation

### Step 2: Fix Quarantine Test (Medium - 15 minutes)
- Debug quarantine_file() behavior
- Fix list_files() if needed
- Ensure proper file handling

### Step 3: Fix Engine Tests (Complex - 30 minutes)
- Analyze GhostEngine::new() implementation
- Fix runtime creation/handling
- Update both tests

---

## Expected Outcome

After fixes:
- ✅ All 73 tests passing (100%)
- ✅ No runtime errors
- ✅ Clean test execution

---

## Time Estimate
- Analysis: 10 minutes
- Implementation: 50 minutes
- Testing: 10 minutes
- **Total**: ~70 minutes (1-1.5 hours)