use crate::mac::{self, MacAddress};

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
            mac: mac::parse_mac_address(mac),
            ip,
            subnet_mask,
            gateway,
        }
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
}
