pub mod interface;

use core::fmt;
use interface::Interface;

pub struct Server {
    hostname: String,

    // for now, one interface
    interface: Interface,
}

impl Server {
    pub fn new(hostname: String) -> Server {
        let interface = Interface::new("00:11:22:33:44:55");

        Server {
            hostname,
            interface,
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
