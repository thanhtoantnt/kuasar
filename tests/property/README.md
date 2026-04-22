# Property-Based Testing

This directory contains property-based tests using [proptest](https://altsysrq.github.io/proptest-book/introduction.html).

## Running Tests

### On Linux

```bash
cargo test -p kuasar-property-tests
```

### On macOS (using Docker)

Since Kuasar has Linux-specific dependencies (netlink, rtnetlink), run tests in a Linux container:

```bash
docker run --rm -v $(pwd):/workspace -w /workspace rust:1.85 \
  bash -c "apt-get update && apt-get install -y cmake protobuf-compiler libprotobuf-dev && cargo test -p kuasar-property-tests"
```

## Test Coverage

| Module | Tests | Properties Verified |
|--------|-------|---------------------|
| `vmm_sandbox/utils` | 8 | CPU set parsing roundtrip, merge, intersection symmetry |
| `vmm_sandbox/network` | 7 | IPv4/IPv6 CIDR parsing, MAC address format |
| `shim` | 3 | ID format validation |

## Custom Strategies

Custom proptest strategies are defined in `src/strategies/`:

- `cpuset.rs` - Generates valid CPU set strings like `"0-3,5,7-9"`
- `network.rs` - Generates IPv4/IPv6 CIDR strings and MAC addresses
- `mount.rs` - Generates mount option vectors

## Adding New Tests

1. Add a strategy in `src/strategies/` if needed
2. Create test file in `src/tests/<module>/`
3. Use the `proptest!` macro:

```rust
proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn my_property_test(input in my_strategy()) {
        prop_assert!(some_condition(input));
    }
}
```

## Configuration

- Default: 256 test cases per property
- Adjust via `ProptestConfig::with_cases(n)`
- Failure cases saved to `proptest-regressions/`
