use etherparse::Ethernet2Header;

use crate::mac::MacAddress;

pub fn build_ethernet(source: MacAddress, destination: MacAddress, payload: Vec<u8>) -> Vec<u8> {
    let mut buffer = Vec::<u8>::new();

    let header = Ethernet2Header {
        source,
        destination,
        ether_type: etherparse::EtherType::IPV4,
    };

    header
        .write(&mut buffer)
        .expect("Failed to write Ethernet frame");
    buffer.extend(payload);

    buffer
}
