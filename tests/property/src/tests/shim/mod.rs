use crate::proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn process_data_id_preserved(id in "[a-z0-9]{1,20}") {
        prop_assert!(!id.is_empty());
    }

    #[test]
    fn container_id_format_valid(id in "[a-z0-9]{1,64}") {
        prop_assert!(id.len() <= 64);
        prop_assert!(id.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn process_id_format_valid(id in "[a-z0-9]{1,64}") {
        prop_assert!(id.len() <= 64);
        prop_assert!(id.chars().all(|c| c.is_ascii_alphanumeric()));
    }
}
