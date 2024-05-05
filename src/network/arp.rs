use std::net::IpAddr;

use crate::mac::MacAddress;

#[allow(dead_code)]
pub struct Arp {}

#[allow(dead_code)]
impl Arp {
    // Simulating a broadcast, not accurate or realistic
    // In a real scenario, this would involve network communication
    pub fn broadcast_arp_request(ip_address: IpAddr) -> Option<MacAddress> {
        match ip_address.to_string().as_str() {
            "192.168.0.1" => Some([0x11, 0x12, 0x13, 0x14, 0x15, 0x16]),
            "192.168.0.2" => Some([0x21, 0x22, 0x23, 0x24, 0x25, 0x26]),
            _ => None, // do nothing and like it
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_broadcast_unknown() {
        let ip_address = IpAddr::V4("192.168.0.4".parse().unwrap());
        let result = Arp::broadcast_arp_request(ip_address);

        let expected = None;

        assert_eq!(result, expected);
    }
}
