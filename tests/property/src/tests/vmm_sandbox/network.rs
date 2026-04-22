use crate::proptest::prelude::*;
use crate::strategies::network::*;
use vmm_sandboxer::network::address::{IpNet, MacAddress};

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn ipv4_cidr_parse_roundtrip(cidr in ipv4_cidr()) {
        let ip_net: IpNet = IpNet::from(cidr.clone());
        let ip = ip_net.ip;
        let prefix = ip_net.prefix_len;
        let rendered: String = String::from(ip_net);
        let reparsed: IpNet = IpNet::from(rendered);
        prop_assert_eq!(ip, reparsed.ip);
        prop_assert_eq!(prefix, reparsed.prefix_len);
    }

    #[test]
    fn ipv6_cidr_parse_roundtrip(cidr in ipv6_cidr()) {
        let ip_net: IpNet = IpNet::from(cidr.clone());
        let ip = ip_net.ip;
        let prefix = ip_net.prefix_len;
        let rendered: String = String::from(ip_net);
        let reparsed: IpNet = IpNet::from(rendered);
        prop_assert_eq!(ip, reparsed.ip);
        prop_assert_eq!(prefix, reparsed.prefix_len);
    }

    #[test]
    fn ip_net_preserves_ip(cidr in ipv4_cidr()) {
        let parts: Vec<&str> = cidr.split('/').collect();
        let original_ip_str = parts[0].to_string();
        let ip_net: IpNet = IpNet::from(cidr);
        prop_assert_eq!(ip_net.addr_string(), original_ip_str);
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
        let original_display = mac_addr.to_string();
        let reparsed_display = reparsed.to_string();
        prop_assert_eq!(original_display, reparsed_display);
    }

    #[test]
    fn mac_address_display_format(mac in mac_address()) {
        let mac_addr: MacAddress = MacAddress::from(mac);
        let displayed = mac_addr.to_string();
        let parts: Vec<&str> = displayed.split(':').collect();
        prop_assert_eq!(parts.len(), 6);
        for part in parts {
            prop_assert_eq!(part.len(), 2);
        }
    }
}