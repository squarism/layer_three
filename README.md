# Layer Three

This project is a simplified model of networking up to layer three. It's designed to help understand the fundamentals of how devices communicate within networks, focusing on the interaction between network interfaces, switches, and IP routing but not delving into the full complexities of a TCP/IP implementation.

The goal is to simulate the process of a server (box1) pinging another server (box2), using ICMP to illustrate the basic interactions between network components such as Ethernet frames, switches, and interfaces.

The other goal of this project is to stay close enough to the abstraction that Linux users and ops people would experience but not so detailed that network engineer specialists would find this realistic or interesting.


## Components

### Layer 2

* Server Network Interface Cards (NICs): Each server can have one or more NICs. Initially, each server will start with one NIC. This NIC is responsible for handling all data that comes in and out of the server at the Ethernet level.
* Switch: Manages connections between multiple NICs. It maintains a table to map MAC addresses to ports and uses this information to forward Ethernet frames to the correct destination.
* Ethernet Frame: Comprises source and destination MAC addresses and a payload. Frames can be captured and analyzed with tools like Wireshark to verify their contents and the functionality of the network simulation.

### Layer 3

* IP Addressing: Each server is assigned an IP address. This IP address, combined with subnet masking, determines whether outgoing traffic is routed to the local network or through a gateway to reach external networks.
* Routing and NAT Table: Servers or routers can perform bit-math operations on IP addresses to decide routing paths based on a NAT table and routing rules. This is crucial for determining the path packets should take through the network.
* ICMP Protocol: Used by network devices to send error messages and operational information indicating success or failure when communicating with other devices, such as during a ping operation. ICMP does not use ports, and it follows specific data structures as defined in the protocol specification.


### Commands

* `ping` Command: This command simulates sending an ICMP echo request from one server to another. It uses the local IP stack to determine the best interface to send out the ICMP packet. The decision involves:

    - Routing Table Lookup: The server checks its routing table to determine if the destination IP is on the local network or if it needs to be routed through a gateway.
    - Interface Selection: Based on the routing decision, the appropriate network interface is selected for sending out the ICMP packet.
    - ICMP: A ICMP packet is made with the appropriate type 8 code for echo and sends it through the selected interface.

### Responsibilities

* Server: Manages its network interfaces, IP configuration, and routing table. It decides the routing path for outgoing packets based on its IP stack configuration.
* NIC: Sends and receives Ethernet frames, encapsulates and decapsulates IP packets into/from Ethernet frames.
* Switch: Forwards frames based on MAC address mappings.
* Router (future addition): Determines the next hop for packets based on a more complex routing table and handles NAT operations if necessary.


## Running

```
$ cargo run
Interface with MAC: "11:12:13:14:15:16" plugged into port: 1
Interface with MAC: "21:22:23:24:25:26" plugged into port: 2
Sending frame out on port with MAC: [21, 22, 23, 24, 25, 26]
Frame forwarded to MAC: "21:22:23:24:25:26"
```

Two boxes are plugged into the switch and the switch notes the MAC.  This is not how a real switch would work.  It would involve a broadcast to all ports which is not currently done in this scenario.

Then box1 sends an ICMP ping.  This turns into an Ethernet frame.
> Sending frame out

Then the switch forwards it to MAC "21:22" which is box2.  Box1's MAC all start with 1s.  Box2's MAC all start with 2s.

```
box1 = 192.168.0.1 = 11:12:13:14:15:16
box2 = 192.168.0.2 = 21:22:23:24:25:26
```

There are a few other steps which has no output which you can see in `main.rs`.


## TODO
- switch flooding, the switch does not learn ports' MACs when plugged in, it learns on send.  If the MAC doesn't exist in the table yet then all ports get a frame.
- multiple interfaces on servers
- routing outside of local lan ie: to a router, implement a router
- port should send the frame out to the attached interface somehow
