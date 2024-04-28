// use crate::server::Server;

use crate::{server::interface::Interface, switch::Switch};
use etherparse::Ethernet2Header;

mod server;
mod switch;

fn main() {
    let mut switch = Switch::new();

    let box1 = Interface::new("01:01:01:01:01:01");
    let box2 = Interface::new("22:22:22:22:22:22");

    switch.plug_in_interface(1, &box1);
    switch.plug_in_interface(2, &box2);

    let frame = create_frame(box1.mac, box2.mac, "Hi".as_bytes());
    switch.forward_frame(&frame);

    println!("{:?}", switch);
}

// TODO: this should be somewhere else like the server as a device driver abstraction
fn create_frame(src_mac: [u8; 6], dst_mac: [u8; 6], _payload: &[u8]) -> Ethernet2Header {
    let mut frame = Vec::<u8>::new();

    let ethernet_header = etherparse::Ethernet2Header {
        source: src_mac,
        destination: dst_mac,
        ether_type: etherparse::EtherType::IPV4,
    };

    ethernet_header.write(&mut frame).unwrap();
    ethernet_header
}
