use proptest::prelude::*;

pub fn single_cpu() -> impl Strategy<Value = String> {
    any::<u32>().prop_map(|n| n.to_string())
}

pub fn cpu_range() -> impl Strategy<Value = String> {
    (any::<u32>(), any::<u32>()).prop_map(|(a, b)| {
        let (low, high) = if a <= b { (a, b) } else { (b, a) };
        if low == high {
            low.to_string()
        } else {
            format!("{}-{}", low, high)
        }
    })
}

pub fn valid_cpuset() -> impl Strategy<Value = String> {
    prop::collection::vec(cpu_range(), 1..5).prop_map(|parts| parts.join(","))
}

pub fn valid_cpuset_parts() -> impl Strategy<Value = Vec<(u32, u32)>> {
    prop::collection::vec(
        (any::<u32>(), any::<u32>()).prop_map(|(a, b)| if a <= b { (a, b) } else { (b, a) }),
        1..5,
    )
}
