extern crate pnet;

use pnet::datalink;
use pnet::datalink::Channel;
use pnet::packet::MutablePacket;
use pnet::util::{NetworkInterface, get_network_interfaces};
use pnet::packet::arp::{ArpOperation, ArpHardwareType, MutableArpPacket};
use pnet::packet::ethernet::{EtherType, MutableEthernetPacket};
use pnet::util::MacAddr;
use std::net::{Ipv4Addr, Ipv6Addr, IpAddr};


fn main() {

    let mut senderhw = MacAddr(0x00, 0x00, 0x00, 0x00, 0x00, 0x00);
    let mut senderipv4 = Ipv4Addr::new(192,168,1,29);
    let mut senderipv6: Ipv6Addr;
    //let targetip = Ipv4Addr::new(127, 0, 0, 1);
    let mut buffer: Vec<u8> = vec![0; 60];
    let mut interface_name = String::from("enp0s3");

    let mut etherpacket = MutableEthernetPacket::new(&mut buffer).unwrap();
    etherpacket.set_ethertype(EtherType(2054));
    etherpacket.set_destination(MacAddr(0xff, 0xff, 0xff, 0xff, 0xff, 0xff));

    let iface = get_network_interfaces();

    for item in iface {
        interface_name = item.name.to_string();
        senderhw = item.mac.unwrap();
        etherpacket.set_source(senderhw);
        match item.ips {
            Some(ips) => for ipaddr in ips {
                match ipaddr {
                    IpAddr::V4(ip) => senderipv4 = ip,
                    IpAddr::V6(ip) => senderipv6 = ip
                }
            },

            None => println!("No IP addresses found!")
        }


        let interface_names_match =
        |iface: &NetworkInterface| iface.name == interface_name;


        let interfaces = get_network_interfaces();
        let interface = interfaces.into_iter()
        .filter(interface_names_match)
        .next()
        .unwrap();

        let (mut tx, mut rx) = match datalink::channel(&interface, &Default::default()) {
            Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("Unhandled channel type"),
            Err(e) => panic!("An error occurred when creating the datalink channel: {}", e)
        };

        let senderoctets = senderipv4.octets();
            for n in 1..255 {
                let targetip = Ipv4Addr::new(senderoctets[0], senderoctets[1], senderoctets[2], n);
                {
                    let mut arppacket = MutableArpPacket::new(etherpacket.payload_mut()).unwrap();
                    arppacket.set_hardware_type(ArpHardwareType(1));
                    arppacket.set_protocol_type(EtherType(2048));
                    arppacket.set_operation(ArpOperation(1));
                    arppacket.set_sender_hw_addr(senderhw);
                    arppacket.set_sender_proto_addr(senderipv4);
                    arppacket.set_target_hw_addr(MacAddr(0xff, 0xff, 0xff, 0xff, 0xff, 0xff));
                    arppacket.set_target_proto_addr(targetip);
                    arppacket.set_hw_addr_len(6);
                    arppacket.set_proto_addr_len(4);
                }
                let finalpacket = etherpacket.to_immutable();
                tx.send_to(&finalpacket, None);
        }
    }
}



