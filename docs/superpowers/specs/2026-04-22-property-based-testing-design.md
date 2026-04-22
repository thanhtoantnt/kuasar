# Property-Based Testing Design for Kuasar

## Summary

Add comprehensive property-based tests using proptest to Kuasar, covering parsing functions, calculation functions, and stateful operations across all crates (vmm/sandbox, vmm/task, vmm/common, quark, runc, shim, wasm).

## Goals

1. Catch edge cases in parsing functions that unit tests miss
2. Verify mathematical properties of calculation functions
3. Ensure state invariants are preserved in stateful operations
4. Establish a maintainable testing infrastructure for future development

## Non-Goals

- Replacing existing unit tests
- Testing IO-bound or system-dependent functions
- Testing external dependencies

## Architecture

### Centralized Test Crate

Create a dedicated `tests/property` crate to house all property-based tests:

```
kuasar/
├── tests/
│   ├── e2e/                    # existing e2e tests
│   └── property/               # NEW: property-based test crate
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs          # test runner, shared config
│           ├── strategies/     # custom proptest strategies
│           │   ├── mod.rs
│           │   ├── network.rs  # IP, MAC, CIDR strategies
│           │   ├── cpuset.rs   # CPU set strategies
│           │   └── mount.rs    # mount option strategies
│           └── tests/
│               ├── mod.rs
│               ├── vmm_sandbox/    # tests for vmm/sandbox
│               │   ├── mod.rs
│               │   ├── utils.rs    # cpuset, resource tests
│               │   ├── network.rs  # address tests
│               │   └── device.rs   # bus/slot tests
│               ├── vmm_task/       # tests for vmm/task
│               │   ├── mod.rs
│               │   └── sandbox.rs  # sysctl conversion tests
│               ├── vmm_common/     # tests for vmm/common
│               │   ├── mod.rs
│               │   └── mount.rs    # mount option tests
│               ├── quark/          # tests for quark
│               ├── runc/           # tests for runc
│               ├── shim/           # tests for shim
│               └── wasm/           # tests for wasm
```

### Rationale

- Single place to manage proptest configuration
- No dev-dependency pollution in production crates
- Easy to run all property tests together: `cargo test -p kuasar-property-tests`
- Can share custom strategies across tests

## Test Coverage

### High Priority: Parsing Functions

| Crate | Module | Functions | Properties |
|-------|--------|-----------|------------|
| vmm/sandbox | utils.rs | `cpuset_parts`, `cpuset_one_part`, `cpuset_tostring` | Parse/render roundtrip |
| vmm/sandbox | utils.rs | `merge_cpusets`, `cpuset_intersect` | Idempotency, associativity, containment |
| vmm/sandbox | network/address.rs | `IpNet::from`, `MacAddress::from` | String roundtrip |
| vmm/common | mount.rs | `parse_options` | Known flags map correctly, unknown preserved |
| quark | mount.rs | `parse_options` | Same as above |
| vmm/task | sandbox.rs | `convert_sysctl_to_proc_path` | Dot-to-slash conversion, prefix addition |

### Medium Priority: Calculation Functions

| Crate | Module | Functions | Properties |
|-------|--------|-----------|------------|
| vmm/sandbox | utils.rs | `merge_resources` | Additive limits, max OOM score |
| vmm/sandbox | client.rs | `checked_compute_delta` | Overflow safety, correct calculation |

### Lower Priority: Stateful Operations

| Crate | Module | Functions | Properties |
|-------|--------|-----------|------------|
| vmm/sandbox | device.rs | Bus slot alloc/dealloc | Invariant preservation |
| shim | data.rs | Container/process add/remove | Lookup correctness |

## Custom Proptest Strategies

### Network Strategies (`strategies/network.rs`)

```rust
/// Generates valid IPv4 CIDR strings like "192.168.1.0/24"
pub fn ipv4_cidr() -> impl Strategy<Value = String>

/// Generates valid IPv6 CIDR strings like "fd00::/64"
pub fn ipv6_cidr() -> impl Strategy<Value = String>

/// Generates valid MAC address strings like "aa:bb:cc:dd:ee:ff"
pub fn mac_address() -> impl Strategy<Value = String>
```

### CPU Set Strategies (`strategies/cpuset.rs`)

```rust
/// Generates valid CPU set strings like "0-3,5,7-9"
pub fn valid_cpuset() -> impl Strategy<Value = String>

/// Generates single CPU strings like "5"
pub fn single_cpu() -> impl Strategy<Value = String>

/// Generates CPU range strings like "0-3"
pub fn cpu_range() -> impl Strategy<Value = String>
```

### Mount Strategies (`strategies/mount.rs`)

```rust
/// Generates valid mount option vectors
pub fn mount_options() -> impl Strategy<Value = Vec<String>>

/// Generates known mount flag names
pub fn known_mount_flag() -> impl Strategy<Value = String>
```

## Test Examples

### CPU Set Roundtrip

```rust
proptest! {
    #[test]
    fn cpuset_parts_roundtrip(s in valid_cpuset()) {
        let parts = cpuset_parts(&s).unwrap();
        for part in parts {
            let rendered = cpuset_tostring(part);
            let reparsed = cpuset_one_part(&rendered).unwrap();
            prop_assert_eq!(part, reparsed);
        }
    }
}
```

### CPU Set Merge Idempotency

```rust
proptest! {
    #[test]
    fn cpuset_merge_idempotent(s in valid_cpuset()) {
        let merged = merge_cpusets(&s, &s).unwrap();
        prop_assert!(cpusets_equivalent(&merged, &s));
    }
}
```

### Sysctl Path Conversion

```rust
proptest! {
    #[test]
    fn sysctl_to_proc_path_conversion(sysctl in "[a-z]+(\\.[a-z0-9]+)+") {
        let path = convert_sysctl_to_proc_path(&sysctl);
        prop_assert!(path.starts_with("/proc/sys/"));
        prop_assert!(!path.contains('.'));
        prop_assert!(path.contains('/'));
    }
}
```

### Mount Options Parsing

```rust
proptest! {
    #[test]
    fn mount_options_deterministic(opts in mount_options()) {
        let (flags1, data1) = parse_options(&opts);
        let (flags2, data2) = parse_options(&opts);
        prop_assert_eq!(flags1, flags2);
        prop_assert_eq!(data1, data2);
    }
}
```

## Implementation Notes

### Function Visibility

Some internal functions may need visibility adjustments:
- Functions tested must be accessible from the test crate
- Use `pub(crate)` for functions that should remain internal to their crate
- The test crate will depend on each crate as a path dependency

### Error Handling

- Tests should handle `Result` types appropriately
- Use `prop_assert!` for property assertions
- Use `.unwrap()` only when the strategy guarantees valid input

### Proptest Configuration

```rust
// lib.rs
proptest! {
    // Reduce default case count for CI
    #![proptest_config(ProptestConfig::with_cases(256))]
}
```

## Success Criteria

1. All identified parsing functions have property tests
2. Tests pass with `cargo test -p kuasar-property-tests`
3. Tests catch at least one edge case that existing tests miss (to be verified)
4. CI integration for running property tests

## Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| Slow test execution | Configure reasonable case counts (256 default) |
| Flaky tests from randomness | Proptest is deterministic with seed |
| Functions not accessible | Add `pub(crate)` or test-only exports |
| Complex strategies | Start simple, iterate based on failures |
