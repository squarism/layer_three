use etherparse::Ethernet2Header;

pub fn build_ethernet(source: [u8; 6], destination: [u8; 6], payload: Vec<u8>) -> Vec<u8> {
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
