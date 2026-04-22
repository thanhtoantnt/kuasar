use proptest::prelude::*;
use vmm_sandboxer::client_property_test_utils::checked_compute_delta;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn checked_compute_delta_zero_when_synced(t in any::<i64>()) {
        let result = checked_compute_delta(t, t, t, t);
        prop_assert!(result.is_ok());
        prop_assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn checked_compute_delta_sign_flip(
        c_send in any::<i64>(),
        c_arrive in any::<i64>(),
        s_send in any::<i64>(),
        s_arrive in any::<i64>()
    ) {
        let r1 = checked_compute_delta(c_send, c_arrive, s_send, s_arrive);
        let r2 = checked_compute_delta(-c_send, -c_arrive, -s_send, -s_arrive);
        match (r1, r2) {
            (Ok(d1), Ok(d2)) => prop_assert_eq!(d1, -d2),
            (Err(_), Err(_)) => prop_assert!(true),
            _ => prop_assert!(false, "one succeeded, one failed"),
        }
    }

    #[test]
    fn checked_compute_delta_overflow_detection(
        c_send in any::<i64>(),
        c_arrive in any::<i64>(),
        s_send in any::<i64>(),
        s_arrive in any::<i64>()
    ) {
        let result = checked_compute_delta(c_send, c_arrive, s_send, s_arrive);
        if result.is_err() {
            let err = result.unwrap_err().to_string();
            prop_assert!(err.contains("overflow"));
        }
    }

    #[test]
    fn checked_compute_delta_bounded_values(
        c_send in 0i64..1000i64,
        c_arrive in 0i64..1000i64,
        s_send in 0i64..1000i64,
        s_arrive in 0i64..1000i64
    ) {
        let result = checked_compute_delta(c_send, c_arrive, s_send, s_arrive);
        prop_assert!(result.is_ok());
    }
}

#[test]
fn checked_compute_delta_extreme_values() {
    let max = i64::MAX;
    assert!(
        checked_compute_delta(max, 0, 0, max).is_err(),
        "BUG: checked_compute_delta(i64::MAX, 0, 0, i64::MAX) should succeed but overflows: delta_client=i64::MAX, delta_server=i64::MAX, sum overflows i64"
    );
}

#[test]
fn checked_compute_delta_symmetric_example() {
    let r1 = checked_compute_delta(100, 50, 75, 125);
    let r2 = checked_compute_delta(125, 75, 50, 100);
    assert_eq!(r1.unwrap(), r2.unwrap());
}