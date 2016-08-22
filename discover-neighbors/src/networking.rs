use pnet::datalink;
use pnet::datalink::{Channel, EthernetDataLinkReceiver};
use pnet::packet::{Packet, MutablePacket};
use pnet::util::get_network_interfaces;
use pnet::packet::arp::{ArpOperations, ArpHardwareTypes, MutableArpPacket, ArpPacket};
use pnet::packet::ethernet::{EtherTypes, MutableEthernetPacket};
use pnet::util::{NetworkInterface, MacAddr};

use std::net::{Ipv4Addr, Ipv6Addr, IpAddr};
use std::thread;
use std::sync::mpsc::{channel, Sender};
use std::time::{Instant, Duration};
use std::collections::HashMap;


pub fn send_and_receive(juju_machine_list: HashMap<String, Ipv4Addr>) -> HashMap<String, Ipv4Addr> {

    let mut nodes: Vec<Ipv4Addr> = vec![];
    let (transmit_channel, receiver_channel) = channel();
    let iface = get_network_interfaces();

    for interface in iface {
        let transmit_channel = transmit_channel.clone();
        let interface = interface.clone();
        let interface2 = interface.clone();
        let unitips = juju_machine_list.clone();

        thread::spawn(move || {
            recieve_packets(interface, transmit_channel);
        });
        thread::spawn(move || {
            send_packets(interface2, unitips);
        });
    }

    // Ten second timeout on receiver
    let mut now = Instant::now();
    while now.elapsed() <= Duration::new(10, 0) {
        match receiver_channel.try_recv() {
            Ok(item) => {
                nodes.push(item);
                now = Instant::now()
            }
            Err(e) => {
                // TODO: Add something here?
            }
        }
    }
    let mut neighbors: HashMap<String, Ipv4Addr> = HashMap::new();

    for (machine, ip) in juju_machine_list {
        if nodes.contains(&ip) {
            neighbors.insert(machine, ip);
        }
    }
    neighbors
}


// Create and send packets given an interface
pub fn send_packets(interface: NetworkInterface, juju_machines: HashMap<String, Ipv4Addr>) {

    // Create the transmission channel in order to send packets
    let (mut tx, _) = match datalink::channel(&interface, &Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => {
            panic!("An error occurred when creating the datalink channel: {}",
                   e)
        }
    };

    // Store blank IPs so the compiler stops throwing errors
    let mut senderipv4 = Ipv4Addr::new(0, 0, 0, 0);
    let mut _senderipv6 = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0);

    let senderhw: MacAddr = interface.mac.unwrap();
    // Create buffer for ARP packets: 60 bytes
    let mut buffer: Vec<u8> = vec![0; 60];
    // Create the ethernet packet that acts as a wrapper for the ARP packet
    let mut ethernetwrapper = MutableEthernetPacket::new(&mut buffer).unwrap();

    ethernetwrapper.set_source(senderhw);
    ethernetwrapper.set_ethertype(EtherTypes::Arp);
    ethernetwrapper.set_destination(MacAddr(0xff, 0xff, 0xff, 0xff, 0xff, 0xff));

    // Match the IP address type out of the std::net::IpAddr enum
    match interface.ips {
        Some(ref ips) => {
            for ipaddr in ips {
                match ipaddr {
                    &IpAddr::V4(ip) => senderipv4 = ip,
                    &IpAddr::V6(ip) => _senderipv6 = ip,
                }
            }
        }
        None => println!("Didn't find any IPs on interface: {}", interface.name),
    }

    // Create the ARP packet inside the ethernet wrapper
    // and set the correct addresses, protocol types, and ARP operation
    {
        let mut arppacket = MutableArpPacket::new(ethernetwrapper.payload_mut()).unwrap();
        arppacket.set_hardware_type(ArpHardwareTypes::Ethernet);
        arppacket.set_protocol_type(EtherTypes::Ipv4);
        arppacket.set_operation(ArpOperations::Request);
        arppacket.set_sender_hw_addr(senderhw);
        arppacket.set_sender_proto_addr(senderipv4);
        arppacket.set_target_hw_addr(MacAddr(0xff, 0xff, 0xff, 0xff, 0xff, 0xff));
        arppacket.set_hw_addr_len(6);
        arppacket.set_proto_addr_len(4);
    }

    // let senderoctets = senderipv4.octets();

    // Iterate over each address in the last octet of the sender's IP to ping each of its
    // neighbors with an ARP request.
    // TODO: Find a way to ping entire subnet, not just last octet
    for targetip in juju_machines.values() {
        {
            MutableArpPacket::new(ethernetwrapper.payload_mut())
                .unwrap()
                .set_target_proto_addr(*targetip);
        }
        // Change the MutableEthernetPacket to an EthernetPacket
        let packet = ethernetwrapper.to_immutable();

        // Send the packet using the channel created above
        tx.send_to(&packet, None);
    }
}

// Receive packets given an interface
pub fn recieve_packets(interface: NetworkInterface, tx: Sender<Ipv4Addr>) {
    // Create the receiver channel to receive packets
    let (_, mut rx) = match datalink::channel(&interface, &Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => {
            panic!("An error occurred when creating the datalink channel: {}",
                   e)
        }
    };

    let mut nodes: Vec<Ipv4Addr> = Vec::new();

    // Create iterator to handle incoming packets then loop over them
    // Five second timeout for incoming packets
    let mut iter = rx.iter();
    let mut now = Instant::now();
    while now.elapsed() <= Duration::new(5, 0) {
        match iter.next() {
            Ok(packet) => {
                if packet.get_ethertype() == EtherTypes::Arp {
                    // Create a temporary packet object from the ethernet packet's payload
                    // This is required because the ARP information is nested within the ethernet
                    // wrapper and cannot be directly accessed
                    let temppacket = ArpPacket::new(packet.payload()).unwrap();
                    // Check to see if the Arp Operation is reply
                    if temppacket.get_operation() == ArpOperations::Reply {
                        tx.send(temppacket.get_sender_proto_addr());
                        now = Instant::now();
                    }
                }
            }

            Err(e) => panic!("An error occurred while reading:{}", e),
        }
    }
}
