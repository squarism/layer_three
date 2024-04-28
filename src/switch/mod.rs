mod port;

use etherparse::Ethernet2Header;

use std::{collections::HashMap, fmt};

use crate::server::interface::Interface;

// an 8-port switch, could be any port size but we are keeping it simple
pub struct Switch {
    ports: HashMap<u8, port::Port>,
    link_lights: HashMap<u8, bool>,
    mac_table: HashMap<[u8; 6], u8>, // MAC to port number
}

impl Switch {
    pub fn new() -> Self {
        Self {
            ports: HashMap::new(),
            link_lights: HashMap::new(),
            mac_table: HashMap::new(),
        }
    }

    pub fn plug_in_interface(&mut self, port_number: u8, interface: &Interface) {
        self.ports
            .insert(port_number, port::Port::new(interface.mac));
        self.link_lights.insert(port_number, true);

        // watch out these MACs are hex
        self.mac_table.insert(interface.mac, port_number);
    }

    pub fn forward_frame(&mut self, frame: &Ethernet2Header) {
        if let Some(&port_number) = self.mac_table.get(&frame.destination) {
            if let Some(port) = self.ports.get(&port_number) {
                port.send_frame(frame);
                println!("Frame forwarded to MAC: {:02X?}", frame.destination);
            }
        }
        // TODO else statements here for error handling
    }
}

impl fmt::Debug for Switch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Switch")
            .field("ports", &self.ports)
            .field("link_lights", &self.link_lights)
            .field("macs", &self.mac_table)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_lights() {
        let mut switch = Switch::new();

        let box1 = Interface::new("01:01:01:01:01:01");
        let box2 = Interface::new("02:02:02:02:02:02");

        switch.plug_in_interface(1, &box1);
        switch.plug_in_interface(2, &box2);

        assert_eq!(switch.mac_table.len(), 2);
        assert_eq!(switch.link_lights.get(&1), Some(&true));
        assert_eq!(switch.link_lights.get(&2), Some(&true));
    }

    #[test]
    fn test_mac_table() {
        let mut switch = Switch::new();

        let box1 = Interface::new("01:01:01:01:01:01");
        let box2 = Interface::new("02:02:02:02:02:02");

        switch.plug_in_interface(1, &box1);
        switch.plug_in_interface(2, &box2);

        let mac_query: &[u8] = &[1, 1, 1, 1, 1, 1];

        assert_eq!(switch.mac_table.get(mac_query), Some(&1));
    }
}
