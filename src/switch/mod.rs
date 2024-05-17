mod port;

use etherparse::Ethernet2Header;

use std::{collections::HashMap, fmt};

use crate::mac::MacAddress;
use crate::server::interface::Interface;

// a dumb 8-port switch, could be any port size but we are keeping it simple
pub struct Switch {
    ports: HashMap<u8, port::Port>,
    link_lights: HashMap<u8, bool>,
    cam_table: HashMap<MacAddress, u8>, // MAC to port number
}

impl Switch {
    pub fn new() -> Self {
        Self {
            ports: HashMap::new(),
            link_lights: HashMap::new(),
            cam_table: HashMap::new(),
        }
    }

    pub fn plug_in_interface(&mut self, port_number: u8, interface: &Interface) {
        self.ports
            .insert(port_number, port::Port::new(interface.mac));
        self.link_lights.insert(port_number, true);

        // watch out these MACs are hex
        self.cam_table.insert(interface.mac, port_number);

        let fmac = crate::mac::to_string(&interface.mac);
        println!(
            "Interface with MAC: {:?} plugged into port: {}",
            fmac, port_number
        );
    }

    pub fn forward_frame(&mut self, frame: Vec<u8>) {
        let ethernet_frame = Ethernet2Header::from_slice(&frame).unwrap().0;

        if let Some(&port_number) = self.cam_table.get(&ethernet_frame.destination) {
            if let Some(port) = self.ports.get(&port_number) {
                port.send_frame(&ethernet_frame);
                println!(
                    "Frame forwarded to MAC: {:?}",
                    crate::mac::to_string(&ethernet_frame.destination)
                );
            }
        } else {
            println!(
                "Cannot find mac {:?}",
                crate::mac::to_string(&ethernet_frame.destination)
            );
        }
    }
}

impl fmt::Debug for Switch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Switch")
            .field("ports", &self.ports)
            .field("link_lights", &self.link_lights)
            .field("macs", &self.cam_table)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_lights() {
        let mut switch = Switch::new();

        let box1 = Interface::new(
            "01:01:01:01:01:01",
            "192.168.0.10".to_owned(),
            "255.255.0.0".to_owned(),
            None,
        );
        let box2 = Interface::new(
            "02:02:02:02:02:02",
            "192.168.0.20".to_owned(),
            "255.255.0.0".to_owned(),
            None,
        );

        switch.plug_in_interface(1, &box1);
        switch.plug_in_interface(2, &box2);

        assert_eq!(switch.cam_table.len(), 2);
        assert_eq!(switch.link_lights.get(&1), Some(&true));
        assert_eq!(switch.link_lights.get(&2), Some(&true));
    }

    #[test]
    fn test_cam_table() {
        let mut switch = Switch::new();

        let box1 = Interface::new(
            "01:01:01:01:01:01",
            "192.168.0.10".to_owned(),
            "255.255.0.0".to_owned(),
            None,
        );
        let box2 = Interface::new(
            "02:02:02:02:02:02",
            "192.168.0.20".to_owned(),
            "255.255.0.0".to_owned(),
            None,
        );

        switch.plug_in_interface(1, &box1);
        switch.plug_in_interface(2, &box2);

        let mac_query: &[u8] = &[1, 1, 1, 1, 1, 1];

        assert_eq!(switch.cam_table.get(mac_query), Some(&1));
    }
}
