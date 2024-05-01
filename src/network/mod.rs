use std::{
    fs::File,
    time::{SystemTime, UNIX_EPOCH},
};

use pcap_file::pcap::{PcapPacket, PcapWriter};

pub mod icmp;
pub mod ip;

pub fn write_pcap(file_name: &str, data: &[u8]) {
    let mut file = File::create(file_name).unwrap();
    let mut pcap_writer = PcapWriter::new(file).unwrap();

    // Current time for the pcap timestamp
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let ts_sec = now.as_secs() as u32;
    let ts_usec = now.subsec_micros();

    // Create and write the pcap packet
    let pcap_packet = PcapPacket::new(now, data.len() as u32, data);
    pcap_writer.write_packet(&pcap_packet).unwrap();
}
