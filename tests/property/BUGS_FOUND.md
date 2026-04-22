# Bugs Found by Property-Based Testing

**Date:** 2026-04-22  
**Test Suite:** `kuasar-property-tests`  
**Total Tests:** 31  
**Passed:** 29  
**Failed:** 2 (real bugs confirmed)

---

## Quick Summary

| Bug | Severity | Location | Status |
|-----|----------|----------|--------|
| `checked_compute_delta` overflow | **High** | client.rs:369 | ✅ Confirmed |
| `merge_cpusets` empty string | **Medium** | utils.rs:197 | ✅ Confirmed |

---

## Bug 1: `checked_compute_delta` Integer Overflow

**Location:** `vmm/sandbox/src/client.rs:369`

**Reproduction:**
```rust
checked_compute_delta(i64::MAX, 0, 0, i64::MAX)  // Returns Err, should return Ok(i64::MAX)
```

**Impact:** Clock synchronization between host and VM fails when timestamps are large.

**Root Cause:** Intermediate sum `delta_client + delta_server` overflows before division by 2.

---

## Bug 2: `merge_cpusets` Fails with Empty String

**Location:** `vmm/sandbox/src/utils.rs:197`

**Reproduction:**
```rust
merge_cpusets("0", "")  // Returns Err, should return Ok("0")
merge_cpusets("", "0")  // Returns Err, should return Ok("0")
```

**Impact:** Container resource merging fails when one container has no CPU affinity set.

**Root Cause:** `"".split(',')` returns `[""]` (one empty element), which fails to parse as `u32`.

---

## Documentation

- **Reproducible Unit Tests:** `tests/property/src/bug_reproduction_tests.rs`
- **Root Cause Analysis:** `tests/property/ROOT_CAUSE_ANALYSIS.md`

---

## Running the Bug Reproduction Tests

```bash
# On Linux
cargo test -p kuasar-property-tests bug_reproduction_tests

# On macOS (Docker)
docker run --rm -v $(pwd):/workspace -w /workspace rust:1.85 \
  bash -c "apt-get update && apt-get install -y cmake protobuf-compiler libprotobuf-dev && \
           cargo test -p kuasar-property-tests bug_reproduction_tests"
```

All 10 reproduction tests pass, confirming both bugs exist in the current codebase.
