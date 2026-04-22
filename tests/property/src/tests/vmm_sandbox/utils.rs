use proptest::prelude::*;
use crate::strategies::cpuset::*;
use vmm_sandboxer::property_test_utils::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn cpuset_one_part_roundtrip(s in single_cpu()) {
        let result = cpuset_one_part(&s);
        prop_assert!(result.is_ok());
        let (low, high) = result.unwrap();
        prop_assert_eq!(low, high);
    }

    #[test]
    fn cpuset_range_parts_roundtrip(s in cpu_range()) {
        let result = cpuset_one_part(&s);
        prop_assert!(result.is_ok());
        let (low, high) = result.unwrap();
        let rendered = cpuset_tostring((low, high));
        let reparsed = cpuset_one_part(&rendered).unwrap();
        prop_assert_eq!((low, high), reparsed);
    }

    #[test]
    fn cpuset_parts_non_empty(s in valid_cpuset()) {
        let result = cpuset_parts(&s);
        prop_assert!(result.is_ok());
        let parts = result.unwrap();
        prop_assert!(!parts.is_empty());
    }

    #[test]
    fn cpuset_parts_roundtrip(s in valid_cpuset()) {
        let parts = cpuset_parts(&s).unwrap();
        for part in parts {
            let rendered = cpuset_tostring(part);
            let reparsed = cpuset_one_part(&rendered).unwrap();
            prop_assert_eq!(part, reparsed);
        }
    }

    #[test]
    fn cpuset_tostring_single_cpu(n in any::<u32>()) {
        let s = cpuset_tostring((n, n));
        prop_assert!(!s.contains('-'));
        prop_assert_eq!(s, n.to_string());
    }

    #[test]
    fn cpuset_tostring_range(low in any::<u32>(), high in any::<u32>()) {
        let (lo, hi) = if low <= high { (low, high) } else { (high, low) };
        let s = cpuset_tostring((lo, hi));
        if lo == hi {
            prop_assert!(!s.contains('-'));
        } else {
            prop_assert!(s.contains('-'));
            prop_assert!(s.starts_with(&lo.to_string()));
            prop_assert!(s.ends_with(&hi.to_string()));
        }
    }

    #[test]
    fn cpuset_intersect_symmetric(a in any::<u32>(), b in any::<u32>(), c in any::<u32>(), d in any::<u32>()) {
        let (a1, b1) = if a <= b { (a, b) } else { (b, a) };
        let (c1, d1) = if c <= d { (c, d) } else { (d, c) };
        prop_assert_eq!(
            cpuset_intersect((a1, b1), (c1, d1)),
            cpuset_intersect((c1, d1), (a1, b1))
        );
    }

    #[test]
    fn cpuset_intersect_reflexive(low in any::<u32>(), high in any::<u32>()) {
        let (lo, hi) = if low <= high { (low, high) } else { (high, low) };
        prop_assert!(cpuset_intersect((lo, hi), (lo, hi)));
    }

    #[test]
    fn merge_cpuset_non_overlapping_returns_base(
        a in 0u32..100u32,
        b in 0u32..100u32,
        c in 200u32..300u32,
        d in 200u32..300u32
    ) {
        let (a1, b1) = if a <= b { (a, b) } else { (b, a) };
        let (c1, d1) = if c <= d { (c, d) } else { (d, c) };
        let merged = merge_cpuset((a1, b1), (c1, d1));
        prop_assert_eq!(merged, (a1, b1));
    }

    #[test]
    fn merge_cpuset_overlapping_extends(
        a in 0u32..50u32,
        b in 50u32..100u32,
        c in 0u32..60u32,
        d in 60u32..120u32
    ) {
        let (a1, b1) = if a <= b { (a, b) } else { (b, a) };
        let (c1, d1) = if c <= d { (c, d) } else { (d, c) };
        prop_assume!(c1 <= b1 && d1 >= a1);
        let merged = merge_cpuset((a1, b1), (c1, d1));
        prop_assert!(merged.0 <= a1.min(c1));
        prop_assert!(merged.1 >= b1.max(d1));
    }

    #[test]
    fn merge_cpusets_idempotent(s in valid_cpuset()) {
        let merged = merge_cpusets(&s, &s);
        prop_assert!(merged.is_ok());
        let result = merged.unwrap();
        let parts1 = cpuset_parts(&s).unwrap();
        let parts2 = cpuset_parts(&result).unwrap();
        prop_assert_eq!(parts1.len(), parts2.len());
    }

    #[test]
    fn merge_cpusets_commutative(a in valid_cpuset(), b in valid_cpuset()) {
        let ab = merge_cpusets(&a, &b);
        let ba = merge_cpusets(&b, &a);
        prop_assert!(ab.is_ok());
        prop_assert!(ba.is_ok());
    }

    #[test]
    fn merge_cpusets_empty_string_rhs(s in valid_cpuset()) {
        let merged = merge_cpusets(&s, "");
        prop_assert!(merged.is_ok(), "merge_cpusets(\"{}\", \"\") should succeed", s);
    }

    #[test]
    fn merge_cpusets_empty_string_lhs(s in valid_cpuset()) {
        let merged = merge_cpusets("", &s);
        prop_assert!(merged.is_ok(), "merge_cpusets(\"\", \"{}\") should succeed", s);
    }

    #[test]
    fn merge_cpuset_self_is_idempotent(low in any::<u32>(), high in any::<u32>()) {
        let (lo, hi) = if low <= high { (low, high) } else { (high, low) };
        let merged = merge_cpuset((lo, hi), (lo, hi));
        prop_assert_eq!(merged, (lo, hi));
    }
}
