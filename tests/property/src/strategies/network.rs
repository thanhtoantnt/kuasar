use proptest::prelude::*;

pub fn ipv4_octet() -> impl Strategy<Value = u8> {
    any::<u8>()
}

pub fn ipv4_addr() -> impl Strategy<Value = String> {
    (ipv4_octet(), ipv4_octet(), ipv4_octet(), ipv4_octet())
        .prop_map(|(a, b, c, d)| format!("{}.{}.{}.{}", a, b, c, d))
}

pub fn ipv4_prefix_len() -> impl Strategy<Value = u8> {
    0u8..=32u8
}

pub fn ipv4_cidr() -> impl Strategy<Value = String> {
    (ipv4_addr(), ipv4_prefix_len()).prop_map(|(ip, prefix)| format!("{}/{}", ip, prefix))
}

pub fn ipv6_hextet() -> impl Strategy<Value = u16> {
    any::<u16>()
}

pub fn ipv6_addr() -> impl Strategy<Value = String> {
    (
        ipv6_hextet(),
        ipv6_hextet(),
        ipv6_hextet(),
        ipv6_hextet(),
        ipv6_hextet(),
        ipv6_hextet(),
        ipv6_hextet(),
        ipv6_hextet(),
    )
        .prop_map(|(a, b, c, d, e, f, g, h)| {
            format!(
                "{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}",
                a, b, c, d, e, f, g, h
            )
        })
}

pub fn ipv6_prefix_len() -> impl Strategy<Value = u8> {
    0u8..=128u8
}

pub fn ipv6_cidr() -> impl Strategy<Value = String> {
    (ipv6_addr(), ipv6_prefix_len()).prop_map(|(ip, prefix)| format!("{}/{}", ip, prefix))
}

pub fn mac_octet() -> impl Strategy<Value = u8> {
    any::<u8>()
}

pub fn mac_address() -> impl Strategy<Value = String> {
    (
        mac_octet(),
        mac_octet(),
        mac_octet(),
        mac_octet(),
        mac_octet(),
        mac_octet(),
    )
        .prop_map(|(a, b, c, d, e, f)| {
            format!(
                "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                a, b, c, d, e, f
            )
        })
}
