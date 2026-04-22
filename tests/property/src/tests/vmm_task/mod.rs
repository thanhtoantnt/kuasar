use crate::proptest::prelude::*;
use vmm_task::property_test_utils::convert_sysctl_to_proc_path;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn sysctl_to_proc_path_adds_prefix(sysctl in "[a-z]+(\\.[a-z0-9_]+)*") {
        let path = convert_sysctl_to_proc_path(&sysctl);
        prop_assert!(path.starts_with("/proc/sys/"));
    }

    #[test]
    fn sysctl_to_proc_path_no_dots(sysctl in "[a-z]+(\\.[a-z0-9_]+)*") {
        let path = convert_sysctl_to_proc_path(&sysctl);
        let after_prefix = path.strip_prefix("/proc/sys/").unwrap();
        prop_assert!(!after_prefix.contains('.'));
    }

    #[test]
    fn sysctl_to_proc_path_dots_become_slashes(sysctl in "[a-z]+(\\.[a-z0-9_]+)+") {
        let path = convert_sysctl_to_proc_path(&sysctl);
        let after_prefix = path.strip_prefix("/proc/sys/").unwrap();
        let dot_count = sysctl.matches('.').count();
        let slash_count = after_prefix.matches('/').count();
        prop_assert_eq!(dot_count, slash_count);
    }

    #[test]
    fn sysctl_to_proc_path_preserves_components(
        first in "[a-z]+",
        rest in prop::collection::vec("[a-z0-9_]+", 1..5)
    ) {
        let sysctl = if rest.is_empty() {
            first.clone()
        } else {
            format!("{}.{}", first, rest.join("."))
        };
        let path = convert_sysctl_to_proc_path(&sysctl);
        prop_assert!(path.contains(&first));
        for component in &rest {
            prop_assert!(path.contains(component));
        }
    }

    #[test]
    fn sysctl_to_proc_path_empty_input() {
        let path = convert_sysctl_to_proc_path("");
        prop_assert_eq!(path, "/proc/sys/");
    }

    #[test]
    fn sysctl_to_proc_path_single_component(comp in "[a-z]+") {
        let path = convert_sysctl_to_proc_path(&comp);
        prop_assert_eq!(path, format!("/proc/sys/{}", comp));
    }
}
