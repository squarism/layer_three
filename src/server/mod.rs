pub mod arp;
pub mod hosts;
pub mod interface;

use core::fmt;
use std::collections::HashMap;

use interface::Interface;

#[allow(dead_code)]
struct Route {
    destination: String,
    gateway: String,
    interface_name: String,
}

#[allow(dead_code)]
pub struct Server {
    hostname: String,

    interface: Interface,               // for now, one interface
    routes: Vec<Route>,                 // Routing table
    arp_table: HashMap<String, String>, // Maps IP addresses to MAC addresses
}

#[allow(dead_code)]
impl Server {
    pub fn new(hostname: String) -> Server {
        let interface = Interface::new(
            "00:11:22:33:44:55",
            "192.168.0.5".to_owned(),
            "255.255.0.0".to_owned(),
            None,
        );

        Server {
            hostname,
            interface,
            routes: vec![],
            arp_table: HashMap::new(),
        }
    }
}

impl fmt::Debug for Server {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // chaining debug statements in rust is messy and tricky
        // if we just make a string here, it is easier
        let mac_str = self
            .interface
            .mac
            .iter()
            .map(|byte| format!("{:02X}", byte))
            .collect::<Vec<String>>()
            .join(":");

        f.debug_struct("Server")
            .field("hostname", &self.hostname)
            .field("interface.mac", &mac_str)
            .finish()
    }
}
