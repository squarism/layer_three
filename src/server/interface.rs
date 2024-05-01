use crate::mac::MacAddress;

#[derive(Clone)]
pub struct Interface {
    pub mac: MacAddress,
    pub ip: String,
    pub subnet_mask: String,
    pub gateway: Option<String>,
}

impl Interface {
    pub fn new(mac: &str, ip: String, subnet_mask: String, gateway: Option<String>) -> Interface {
        Interface {
            mac: Self::parse_mac_address(mac),
            ip,
            subnet_mask,
            gateway,
        }
    }

    pub fn parse_mac_address(mac: &str) -> MacAddress {
        let parts: Vec<&str> = mac.split(':').collect();
        if parts.len() != 6 {
            panic!("Invalid MAC address format");
        }

        let mut mac_array = [0u8; 6];
        for (i, part) in parts.iter().enumerate() {
            mac_array[i] = u8::from_str_radix(part, 16).expect("Invalid hex value in MAC address");
        }
        mac_array
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mac = "AA:BB:CC:DD:EE:FF";
        let ip = "192.168.0.10".to_owned();
        let subnet_mask = "255.255.0.0".to_owned();
        let gateway = Some("192.168.0.1".to_owned());

        let _interface = Interface::new(mac, ip, subnet_mask, gateway);
    }

    #[test]
    fn test_parse_mac_address_valid() {
        let mac_str = "AA:BB:CC:DD:EE:FF";
        let expected = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
        assert_eq!(Interface::parse_mac_address(mac_str), expected);
    }

    #[test]
    #[should_panic(expected = "Invalid MAC address format")]
    fn test_parse_mac_address_invalid_format() {
        let mac_str = "AA:BB:CC:DD:EE"; // Missing one part
        Interface::parse_mac_address(mac_str);
    }

    #[test]
    #[should_panic(expected = "Invalid hex value in MAC address")]
    fn test_parse_mac_address_invalid_hex() {
        let mac_str = "AA:BB:CC:DD:EE:ZZ"; // 'ZZ' is not a valid hex value
        Interface::parse_mac_address(mac_str);
    }
}
