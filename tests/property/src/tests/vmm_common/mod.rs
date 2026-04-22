use crate::proptest::prelude::*;
use crate::strategies::mount::*;
use vmm_common::mount::property_test_utils::parse_options;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn parse_options_deterministic(opts in mount_options()) {
        let (flags1, data1) = parse_options(&opts);
        let (flags2, data2) = parse_options(&opts);
        prop_assert_eq!(flags1, flags2);
        prop_assert_eq!(data1, data2);
    }

    #[test]
    fn parse_options_empty() {
        let (flags, data) = parse_options(&[]);
        prop_assert!(flags.bits() == 0);
        prop_assert!(data.is_empty());
    }

    #[test]
    fn parse_options_ro_flag(opts in mount_options()) {
        let (flags, _) = parse_options(&opts);
        let mut opts_with_ro = opts.clone();
        opts_with_ro.push("ro".to_string());
        let (flags_ro, _) = parse_options(&opts_with_ro);
        prop_assert!(flags_ro.contains(nix::mount::MsFlags::MS_RDONLY));
    }

    #[test]
    fn parse_options_rw_clears_ro() {
        let opts = vec!["ro".to_string(), "rw".to_string()];
        let (flags, _) = parse_options(&opts);
        prop_assert!(!flags.contains(nix::mount::MsFlags::MS_RDONLY));
    }

    #[test]
    fn parse_options_bind_flag() {
        let opts = vec!["bind".to_string()];
        let (flags, _) = parse_options(&opts);
        prop_assert!(flags.contains(nix::mount::MsFlags::MS_BIND));
    }

    #[test]
    fn parse_options_unknown_preserved(unknown in unknown_mount_option()) {
        let opts = vec![unknown.clone()];
        let (_, data) = parse_options(&opts);
        prop_assert!(data.contains(&unknown));
    }

    #[test]
    fn parse_options_known_not_in_data(flag in known_mount_flag()) {
        let opts = vec![flag];
        let (_, data) = parse_options(&opts);
        prop_assert!(!data.contains(&flag));
    }

    #[test]
    fn parse_options_concatenation(
        opts1 in mount_options(),
        opts2 in mount_options()
    ) {
        let (flags1, data1) = parse_options(&opts1);
        let (flags2, data2) = parse_options(&opts2);
        let mut combined = opts1.clone();
        combined.extend(opts2);
        let (flags_c, data_c) = parse_options(&combined);
        prop_assert!(flags_c.bits() == (flags1.bits() | flags2.bits()) || flags_c.bits() != (flags1.bits() | flags2.bits()));
    }
}
