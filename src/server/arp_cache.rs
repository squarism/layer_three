use std::collections::HashMap;
use std::net::IpAddr;

use crate::mac::MacAddress;
use crate::network::arp::Arp;

pub struct ArpCache {
    entries: HashMap<IpAddr, MacAddress>,
}

impl ArpCache {
    pub fn new() -> Self {
        ArpCache {
            entries: HashMap::new(),
        }
    }

    pub fn lookup(&mut self, ip_address: &IpAddr) -> Option<&MacAddress> {
        let cache_hit = self.entries.get(ip_address);

        if cache_hit.is_none() {
            let mac = Arp::broadcast_arp_request(*ip_address);
            if let Some(mac) = mac {
                self.add_entry(*ip_address, mac);
            }
        }
        self.entries.get(ip_address)
    }

    fn add_entry(&mut self, ip_address: IpAddr, mac_address: MacAddress) {
        self.entries.insert(ip_address, mac_address);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_cache_is_empty() {
        let cache = ArpCache::new();
        assert!(cache.entries.is_empty());
    }

    #[test]
    fn test_cache_miss() {
        let mut cache = ArpCache::new();
        let ip = IpAddr::V4("192.168.0.4".parse().unwrap());

        let result = cache.lookup(&ip);
        let expected = None;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_lookup_and_cache() {
        let mut cache = ArpCache::new();
        let ip_address = IpAddr::V4("192.168.0.1".parse().unwrap());

        let result = cache.lookup(&ip_address);
        let expected = Some([0x11, 0x12, 0x13, 0x14, 0x15, 0x16]);

        assert_eq!(result, expected.as_ref());
    }
}
