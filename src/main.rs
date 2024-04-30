mod mac;
mod server;
mod switch;

fn main() {
    // box1 pings box2

    // box1 calls getbyhostname(box2) which is resolved by hosts.rs

    // box1's IP stack figures out that box2 is on eth0
    // box1's IP stack figures out that the request is local, not needed to forward to the gateway

    // box1 crafts an ICMP echo request packet

    // now, this is a one-shot simulation program so we will setup this scenario but later we might turn this into
    // a long running or concurrent or GUI program where this is situation is not pre-determined, but in the meantime ...
    // box1 looks up the MAC address for box2 using ARP
    // The ARP response is not found so it broadcasts an ARP who-has and gets a response
    // box1 adds the response to its ARP cache

    // box1 now crafts an Ethernet frame with the ip datagram in it (the ICMP packet) and sends it to
    // the device.  This is simulated by a Rust channel or a method call on a Cable or Bus struct or something.

    // the entire process is unwound on box2 which will not be covered here for now
}
