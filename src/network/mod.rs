use std::{
    fs::File,
    time::{SystemTime, UNIX_EPOCH},
};

use pcap_file::pcap::{PcapPacket, PcapWriter};

pub mod arp;
pub mod icmp;
pub mod ip;

// call this function when you want to write to pcap format and open it in wireshark
#[allow(dead_code)]
pub fn write_pcap(file_name: &str, data: &[u8]) {
    let file = File::create(file_name).unwrap();
    let mut pcap_writer = PcapWriter::new(file).unwrap();

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let pcap_packet = PcapPacket::new(now, data.len() as u32, data);
    pcap_writer.write_packet(&pcap_packet).unwrap();
}
