use crate::proptest::prelude::*;
use crate::strategies::network::*;
use std::net::IpAddr;
use std::str::FromStr;
use vmm_sandboxer::network::address::{IpNet, MacAddress};

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn ipv4_cidr_parse_roundtrip(cidr in ipv4_cidr()) {
        let ip_net: IpNet = IpNet::from(cidr.clone());
        let rendered: String = String::from(ip_net);
        let reparsed: IpNet = IpNet::from(rendered);
        prop_assert_eq!(ip_net.ip, reparsed.ip);
        prop_assert_eq!(ip_net.prefix_len, reparsed.prefix_len);
    }

    #[test]
    fn ipv6_cidr_parse_roundtrip(cidr in ipv6_cidr()) {
        let ip_net: IpNet = IpNet::from(cidr.clone());
        let rendered: String = String::from(ip_net);
        let reparsed: IpNet = IpNet::from(rendered);
        prop_assert_eq!(ip_net.ip, reparsed.ip);
        prop_assert_eq!(ip_net.prefix_len, reparsed.prefix_len);
    }

    #[test]
    fn ip_net_preserves_ip(cidr in ipv4_cidr()) {
        let parts: Vec<&str> = cidr.split('/').collect();
        let original_ip = IpAddr::from_str(parts[0]).unwrap();
        let ip_net: IpNet = IpNet::from(cidr);
        prop_assert_eq!(ip_net.ip, original_ip);
    }

    #[test]
    fn ip_net_prefix_in_range(cidr in ipv4_cidr()) {
        let ip_net: IpNet = IpNet::from(cidr);
        prop_assert!(ip_net.prefix_len <= 32);
    }

    #[test]
    fn ipv6_net_prefix_in_range(cidr in ipv6_cidr()) {
        let ip_net: IpNet = IpNet::from(cidr);
        prop_assert!(ip_net.prefix_len <= 128);
    }

    #[test]
    fn mac_address_parse_roundtrip(mac in mac_address()) {
        let mac_addr: MacAddress = MacAddress::from(mac.clone());
        let rendered = mac_addr.to_string();
        let reparsed: MacAddress = MacAddress::from(rendered);
        prop_assert_eq!(mac_addr.0, reparsed.0);
    }

    #[test]
    fn mac_address_has_six_octets(mac in mac_address()) {
        let mac_addr: MacAddress = MacAddress::from(mac);
        prop_assert_eq!(mac_addr.0.len(), 6);
    }

    #[test]
    fn mac_address_display_format(mac in mac_address()) {
        let mac_addr: MacAddress = MacAddress::from(mac.clone());
        let displayed = mac_addr.to_string();
        let parts: Vec<&str> = displayed.split(':').collect();
        prop_assert_eq!(parts.len(), 6);
        for part in parts {
            prop_assert_eq!(part.len(), 2);
        }
    }
}
