use std::fmt;

use etherparse::Ethernet2Header;

pub struct Port {
    mac_address: [u8; 6],
}

impl Port {
    pub fn new(mac_address: [u8; 6]) -> Self {
        Self { mac_address }
    }

    pub fn send_frame(&self, _frame: &Ethernet2Header) {
        // TODO: sim sending frame out of this port
        println!(
            "Sending frame out on port with MAC: {:02X?}",
            self.mac_address
        );
    }
}

impl fmt::Debug for Port {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Port MAC: {:02X?}", self.mac_address)
    }
}
