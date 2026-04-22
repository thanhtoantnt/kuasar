/*
 * Reproducible unit tests for confirmed bugs found by property-based testing.
 * 
 * These tests demonstrate the exact failure cases and can be used to verify fixes.
 */

use vmm_sandboxer::client_property_test_utils::checked_compute_delta;
use vmm_sandboxer::property_test_utils::{cpuset_parts, merge_cpusets};

// ============================================================================
// BUG 1: checked_compute_delta overflow with extreme values
// ============================================================================

/// Reproduces Bug 1: checked_compute_delta overflows when both delta_client
/// and delta_server are near i64::MAX.
///
/// The formula is: delta = ((c_send - c_arrive) + (s_arrive - s_send)) / 2
///
/// With c_send=i64::MAX, c_arrive=0, s_send=0, s_arrive=i64::MAX:
///   delta_client = i64::MAX - 0 = i64::MAX
///   delta_server = i64::MAX - 0 = i64::MAX
///   delta_sum = i64::MAX + i64::MAX = OVERFLOW!
///
/// The correct mathematical result would be i64::MAX, which fits in i64.
#[test]
fn bug_1_checked_compute_delta_overflow_extreme_values() {
    let max = i64::MAX;
    
    // This should succeed because the final result (i64::MAX) fits in i64
    // But it fails because the intermediate sum overflows
    let result = checked_compute_delta(max, 0, 0, max);
    
    assert!(
        result.is_err(),
        "BUG CONFIRMED: checked_compute_delta(i64::MAX, 0, 0, i64::MAX) returned {:?}, \
         but should return Ok({}) because the mathematical result fits in i64",
        result,
        max
    );
    
    // Verify the error message mentions overflow
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("overflow"),
        "Error message should mention overflow: {}",
        err
    );
}

/// Additional test case: overflow happens with values that sum to > i64::MAX
#[test]
fn bug_1_checked_compute_delta_overflow_near_max() {
    // Two values that individually fit but sum overflows
    let a = i64::MAX - 100;
    let b = 200i64;
    
    // delta_client = a, delta_server = b
    // delta_sum = a + b = i64::MAX - 100 + 200 = i64::MAX + 100 = OVERFLOW
    let result = checked_compute_delta(a, 0, 0, b);
    
    assert!(
        result.is_err(),
        "BUG CONFIRMED: overflow when sum exceeds i64::MAX"
    );
}

/// Shows that the function works correctly for values that don't overflow
#[test]
fn bug_1_checked_compute_delta_works_for_bounded_values() {
    // Normal case: works fine
    let result = checked_compute_delta(1000, 0, 0, 1000);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1000);
    
    // Larger but not overflowing
    let large = 1_000_000_000i64;
    let result = checked_compute_delta(large, 0, 0, large);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), large);
}

// ============================================================================
// BUG 2: merge_cpusets fails with empty string
// ============================================================================

/// Reproduces Bug 2: merge_cpusets fails when one argument is empty string.
///
/// Root cause analysis:
/// 1. merge_cpusets("0", "") calls cpuset_parts("0") and cpuset_parts("")
/// 2. cpuset_parts("") splits "" on ',' getting iterator with one element: ""
/// 3. cpuset_one_part("") tries to parse "" as u32, which fails
/// 4. The error propagates up, causing merge_cpusets to fail
///
/// Expected behavior: empty string should be treated as identity (no cpuset)
#[test]
fn bug_2_merge_cpusets_empty_string_rhs() {
    let result = merge_cpusets("0", "");
    
    assert!(
        result.is_err(),
        "BUG CONFIRMED: merge_cpusets(\"0\", \"\") returned {:?}, \
         but should return Ok(\"0\") treating empty as identity",
        result
    );
}

#[test]
fn bug_2_merge_cpusets_empty_string_lhs() {
    let result = merge_cpusets("", "0");
    
    assert!(
        result.is_err(),
        "BUG CONFIRMED: merge_cpusets(\"\", \"0\") returned {:?}, \
         but should return Ok(\"0\") treating empty as identity",
        result
    );
}

#[test]
fn bug_2_merge_cpusets_both_empty() {
    let result = merge_cpusets("", "");
    
    assert!(
        result.is_err(),
        "BUG CONFIRMED: merge_cpusets(\"\", \"\") returned {:?}, \
         but should return Ok(\"\") treating empty as identity",
        result
    );
}

/// Shows the root cause: cpuset_parts fails on empty string
#[test]
fn bug_2_root_cause_cpuset_parts_empty_string() {
    let result = cpuset_parts("");
    
    assert!(
        result.is_err(),
        "ROOT CAUSE CONFIRMED: cpuset_parts(\"\") returned {:?}, \
         but empty string should be valid (representing no cpuset)",
        result
    );
}

/// Shows that merge_cpusets works correctly for non-empty strings
#[test]
fn bug_2_merge_cpusets_works_for_non_empty() {
    let result = merge_cpusets("0", "1");
    assert!(result.is_ok());
    
    let result = merge_cpusets("0-3", "4-7");
    assert!(result.is_ok());
}

// ============================================================================
// Additional context tests
// ============================================================================

/// Shows the real-world context: clock synchronization in VMs
/// 
/// The checked_compute_delta function is used in sync_clock (client.rs:353)
/// to compute the time offset between host and guest VM for clock synchronization.
///
/// Timestamps are nanoseconds since boot, which can reach i64::MAX after ~292 years.
/// While unlikely in practice, the overflow could theoretically occur.
#[test]
fn bug_1_context_clock_synchronization() {
    // Simulate realistic clock sync scenario
    let host_send = 1_000_000_000i64;      // 1 second in nanos
    let host_arrive = 1_100_000_000i64;    // 1.1 seconds
    let guest_send = 1_050_000_000i64;     // 1.05 seconds
    let guest_arrive = 1_150_000_000i64;   // 1.15 seconds
    
    let result = checked_compute_delta(host_send, host_arrive, guest_send, guest_arrive);
    assert!(result.is_ok(), "Normal clock sync should work");
}

/// Shows the real-world context: container resource merging
///
/// The merge_cpusets function is used in merge_resources (utils.rs:153)
/// to combine CPU affinity settings when multiple containers share resources.
///
/// An empty cpuset means "no CPU affinity specified", which should be valid.
#[test]
fn bug_2_context_resource_merging() {
    // This is how merge_cpusets is called in merge_resources
    // When a container has no cpuset specified, it passes empty string
    
    // Container 1: cpuset "0-3"
    // Container 2: no cpuset specified (empty string)
    // Expected: result should be "0-3"
    
    let result = merge_cpusets("0-3", "");
    assert!(
        result.is_err(),
        "BUG: This fails in production when merging resources with empty cpuset"
    );
}
