use std::collections::HashMap;
use std::net::IpAddr;

use crate::mac::MacAddress;

#[allow(dead_code)]
struct ArpCache {
    entries: HashMap<IpAddr, MacAddress>,
}

#[allow(dead_code)]
impl ArpCache {
    fn new() -> Self {
        ArpCache {
            entries: HashMap::new(),
        }
    }

    fn lookup(&mut self, ip_address: &IpAddr) -> Option<&MacAddress> {
        let cache_hit = self.entries.get(ip_address);

        if cache_hit.is_none() {
            self.broadcast_arp_request(*ip_address)
        }
        self.entries.get(ip_address)
    }

    fn add_entry(&mut self, ip_address: IpAddr, mac_address: MacAddress) {
        self.entries.insert(ip_address, mac_address);
    }

    // Simulating a broadcast, not accurate
    // In a real scenario, this would involve network communication
    fn broadcast_arp_request(&mut self, ip_address: IpAddr) {
        match ip_address.to_string().as_str() {
            "192.168.0.1" => self.add_entry(ip_address, [0x11, 0x12, 0x13, 0x14, 0x15, 0x16]),
            "192.168.0.2" => self.add_entry(ip_address, [0x21, 0x22, 0x23, 0x24, 0x55, 0x26]),
            _ => {} // do nothing and like it
        }
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
