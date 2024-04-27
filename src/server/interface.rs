pub struct Interface {
    pub ip: String,
    pub mac: [u8; 6],
}

impl Interface {
    pub fn new(mac: &str) -> Interface {
        Interface {
            ip: "1.2.3.4".to_owned(),
            mac: Self::parse_mac_address(mac),
        }
    }

    pub fn parse_mac_address(mac: &str) -> [u8; 6] {
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
