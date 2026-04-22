# Commit Messages for Bug Reports

## Commit 1: Report Bug - checked_compute_delta Integer Overflow

```
bug(client): checked_compute_delta overflows with large timestamps

The checked_compute_delta function in client.rs:369 overflows when
computing clock offset for large timestamp values, even when the
final result would fit in i64.

Reproduction:
  checked_compute_delta(i64::MAX, 0, 0, i64::MAX) returns Err
  Expected: Ok(i64::MAX)

Root cause:
  The formula is: delta = ((c_send - c_arrive) + (s_arrive - s_send)) / 2
  When delta_client and delta_server are both near i64::MAX,
  their sum overflows BEFORE the division by 2.

Impact:
  Clock synchronization between host and guest VM fails for large
  timestamps. Affects sync_clock() in client.rs:353.

Suggested fix:
  Use i128 for intermediate calculations to avoid overflow.

Files affected:
  - vmm/sandbox/src/client.rs:369-387

Reproduction test:
  tests/property/src/bug_reproduction_tests.rs:bug_1_checked_compute_delta_overflow_extreme_values
```

---

## Commit 2: Report Bug - merge_cpusets Fails with Empty String

```
bug(utils): merge_cpusets fails when merging with empty cpuset

The merge_cpusets function in utils.rs:197 returns an error when
one of the arguments is an empty string, but empty string should
be treated as identity (no cpuset specified).

Reproduction:
  merge_cpusets("0", "") returns Err
  merge_cpusets("", "0") returns Err
  Expected: Ok("0") in both cases

Root cause:
  In Rust, "".split(',') returns an iterator with ONE element [""],
  not zero elements. This causes cpuset_parts("") to call
  cpuset_one_part(""), which tries to parse "" as u32 and fails.

Impact:
  Container resource merging fails when one container has no CPU
  affinity set (empty cpuset_cpus or cpuset_mems). Affects
  merge_resources() in utils.rs:153.

Suggested fix:
  Add empty string check at the start of merge_cpusets() or
  cpuset_parts() to treat empty as identity.

Files affected:
  - vmm/sandbox/src/utils.rs:197-222 (merge_cpusets)
  - vmm/sandbox/src/utils.rs:251-258 (cpuset_parts)

Reproduction tests:
  tests/property/src/bug_reproduction_tests.rs:bug_2_merge_cpusets_empty_string_rhs
  tests/property/src/bug_reproduction_tests.rs:bug_2_merge_cpusets_empty_string_lhs
  tests/property/src/bug_reproduction_tests.rs:bug_2_root_cause_cpuset_parts_empty_string
```
