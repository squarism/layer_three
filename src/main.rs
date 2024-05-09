use network::icmp;
use server::{hosts::Host, Server};

mod mac;
mod network;
mod server;
mod switch;

fn main() {
    // Let's say that a server `box1` pings `box2`.

    // We need to set up our network "by hand"
    let mut switch = switch::Switch::new();

    let box1_interface = server::interface::Interface::new(
        "11:12:13:14:15:16",
        "192.168.0.1".to_owned(),
        "255.255.0.0".to_owned(),
        None,
    );
    let mut box1 = Server::new("box1".to_owned(), box1_interface.clone());

    switch.plug_in_interface(1, &box1.interface);

    let box2_interface = server::interface::Interface::new(
        "21:22:23:24:25:26",
        "192.168.0.2".to_owned(),
        "255.255.0.0".to_owned(),
        None,
    );
    let box2 = Server::new("box2".to_owned(), box2_interface.clone());

    switch.plug_in_interface(2, &box2.interface);

    // box1 calls getbyhostname(box2) which is simulated here
    let hosts_file = make_hosts_file();
    let box2_host = hosts_file
        .iter()
        .find(|&h| h.host == box2.hostname)
        .expect("The demo has gone south because box2 is not in hosts");

    // box1's IP stack figures out that the request is local, not needed to forward to the gateway
    let local_lan = crate::network::ip::same_subnet(
        box1_interface.ip.parse().expect("box1 ip is not an IP"),
        box2_interface.ip.parse().expect("box2 ip is not an IP"),
        box1_interface.subnet_mask.clone(),
    );

    if !local_lan {
        // TODO: routing and routers
        panic!("Routing not implemented.");
    }

    // box1 crafts an ICMP echo request and IP packet
    let payload = "This is a ping, weee";
    let icmp_packet = icmp::packet(box1.interface.ip, box2.interface.ip, payload);

    // Once the IP address of box2 is known, box1 checks its ARP cache.
    // now, this is a one-shot simulation program so we will setup this scenario but later we
    // might turn this into a long running or concurrent or GUI program where this is situation
    // is not pre-determined, but in the meantime ... box1 looks up the MAC address for box2
    // using ARP The ARP response is not found so it broadcasts an ARP who-has and gets a
    // response box1 adds the response to its ARP cache
    let dest_mac = box1
        .arp_table
        .lookup(&box2_host.ip)
        .expect("The demo has gone south because box2 ARP resolution failed");

    let ethernet_frame =
        network::ethernet::build_ethernet(box1_interface.mac, *dest_mac, icmp_packet);

    // uncomment to see the packet and open in wireshark
    // write_pcap("ping.pcap", &ethernet_frame);

    // the packet is sent over ethernet to the switch which has its own MAC table etc
    switch.forward_frame(ethernet_frame);

    // the entire process is unwound on box2 which will not be covered here for now
}

fn make_hosts_file() -> Vec<Host> {
    let host1 = Host {
        host: "box1".to_owned(),
        ip: "192.168.0.1".parse().unwrap(),
    };

    let host2 = Host {
        host: "box2".to_owned(),
        ip: "192.168.0.2".parse().unwrap(),
    };

    vec![host1, host2]
}
