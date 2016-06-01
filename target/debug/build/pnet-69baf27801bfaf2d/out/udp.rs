// Copyright (c) 2014, 2015 Robert Clipsham <robert@octarineparrot.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use packet::Packet;
use packet::ip::IpNextHeaderProtocol;

use pnet_macros_support::types::*;

use std::net::{Ipv4Addr, Ipv6Addr};

/// Represents an UDP Packet
#[allow(unused_attributes)]
#[derive(Debug, Clone)]
pub struct Udp {
    source: u16be,
    destination: u16be,
    length: u16be,
    checksum: u16be,
    payload: Vec<u8>,
}


// Checksum pseudo-header
// IPv4 source

// IPv4 destination

// IPv4 Next level protocol

// UDP Length

// Checksum UDP header/packet
// If the length is odd, make sure to checksum the final byte




// Set data





 /* source */
 /* destination */
 /* length */
/* checksum */


// Checksum pseudo-header
// IPv6 source

// IPv6 destination

// IPv6 Next header

// UDP Length

// Checksum UDP header/packet
// If the length is odd, make sure to checksum the final byte





// Set data





 /* source */
 /* destination */
 /* length */
/* checksum */
/// A structure enabling manipulation of on the wire packets
#[derive(PartialEq)]
pub struct UdpPacket<'p> {
    packet: &'p [u8],
}
/// A structure enabling manipulation of on the wire packets
#[derive(PartialEq)]
pub struct MutableUdpPacket<'p> {
    packet: &'p mut [u8],
}
impl <'a> UdpPacket<'a> {
    /// Constructs a new UdpPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<UdpPacket<'p>> {
        if packet.len() >= UdpPacket::minimum_packet_size() {
            Some(UdpPacket{packet: packet,})
        } else { None }
    }
    /// Maps from a UdpPacket to a UdpPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> UdpPacket<'p> {
        match *self {
            UdpPacket { ref packet } => UdpPacket{packet: packet,},
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub fn minimum_packet_size() -> usize { 8 }
    /// The size (in bytes) of a Udp instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Udp) -> usize { 8 + _packet.payload.len() }
    /// Get the source field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_source(&self) -> u16be {
        let _self = self;
        let co = 0;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Get the destination field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_destination(&self) -> u16be {
        let _self = self;
        let co = 2;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Get the length field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_length(&self) -> u16be {
        let _self = self;
        let co = 4;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Get the checksum field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_checksum(&self) -> u16be {
        let _self = self;
        let co = 6;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
}
impl <'a> MutableUdpPacket<'a> {
    /// Constructs a new MutableUdpPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8]) -> Option<MutableUdpPacket<'p>> {
        if packet.len() >= MutableUdpPacket::minimum_packet_size() {
            Some(MutableUdpPacket{packet: packet,})
        } else { None }
    }
    /// Maps from a MutableUdpPacket to a UdpPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> UdpPacket<'p> {
        match *self {
            MutableUdpPacket { ref packet } => UdpPacket{packet: packet,},
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub fn minimum_packet_size() -> usize { 8 }
    /// The size (in bytes) of a Udp instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Udp) -> usize { 8 + _packet.payload.len() }
    /// Populates a UdpPacket using a Udp structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &Udp) {
        let _self = self;
        _self.set_source(packet.source);
        _self.set_destination(packet.destination);
        _self.set_length(packet.length);
        _self.set_checksum(packet.checksum);
        _self.set_payload(&packet.payload);
    }
    /// Get the source field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_source(&self) -> u16be {
        let _self = self;
        let co = 0;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Get the destination field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_destination(&self) -> u16be {
        let _self = self;
        let co = 2;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Get the length field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_length(&self) -> u16be {
        let _self = self;
        let co = 4;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Get the checksum field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_checksum(&self) -> u16be {
        let _self = self;
        let co = 6;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Set the source field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_source(&mut self, val: u16be) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 1] = (val) as u8;
    }
    /// Set the destination field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_destination(&mut self, val: u16be) {
        let _self = self;
        let co = 2;
        _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 1] = (val) as u8;
    }
    /// Set the length field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_length(&mut self, val: u16be) {
        let _self = self;
        let co = 4;
        _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 1] = (val) as u8;
    }
    /// Set the checksum field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_checksum(&mut self, val: u16be) {
        let _self = self;
        let co = 6;
        _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 1] = (val) as u8;
    }
    /// Set the value of the payload field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_payload(&mut self, vals: &[u8]) {
        use std::ptr::copy_nonoverlapping;
        let mut _self = self;
        let current_offset = 8;
        unsafe {
            copy_nonoverlapping(vals[..].as_ptr(),
                                _self.packet[current_offset..].as_mut_ptr(),
                                vals.len())
        }
    }
}
impl <'a> ::pnet::packet::PacketSize for UdpPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize { let _self = self; 8 }
}
impl <'a> ::pnet::packet::PacketSize for MutableUdpPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize { let _self = self; 8 }
}
impl <'a> ::pnet::packet::MutablePacket for MutableUdpPacket<'a> {
    #[inline]
    fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] { &mut self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
        let _self = self;
        let start = 8;
        &mut _self.packet[start..]
    }
}
impl <'a> ::pnet::packet::Packet for MutableUdpPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 8;
        &_self.packet[start..]
    }
}
impl <'a> ::pnet::packet::Packet for UdpPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 8;
        &_self.packet[start..]
    }
}
/// Used to iterate over a slice of `UdpPacket`s
pub struct UdpIterable<'a> {
    buf: &'a [u8],
}
impl <'a> Iterator for UdpIterable<'a> {
    type
    Item
    =
    UdpPacket<'a>;
    fn next(&mut self) -> Option<UdpPacket<'a>> {
        use pnet::packet::PacketSize;
        if self.buf.len() > 0 {
            let ret = UdpPacket::new(self.buf).unwrap();
            self.buf = &self.buf[ret.packet_size()..];
            return Some(ret);
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
}
impl <'p> ::pnet::packet::FromPacket for UdpPacket<'p> {
    type
    T
    =
    Udp;
    #[inline]
    fn from_packet(&self) -> Udp {
        use pnet::packet::Packet;
        let _self = self;
        Udp{source: _self.get_source(),
            destination: _self.get_destination(),
            length: _self.get_length(),
            checksum: _self.get_checksum(),
            payload:
                {
                    let payload = self.payload();
                    let mut vec = Vec::with_capacity(payload.len());
                    vec.extend_from_slice(payload);
                    vec
                },}
    }
}
impl <'p> ::pnet::packet::FromPacket for MutableUdpPacket<'p> {
    type
    T
    =
    Udp;
    #[inline]
    fn from_packet(&self) -> Udp {
        use pnet::packet::Packet;
        let _self = self;
        Udp{source: _self.get_source(),
            destination: _self.get_destination(),
            length: _self.get_length(),
            checksum: _self.get_checksum(),
            payload:
                {
                    let payload = self.payload();
                    let mut vec = Vec::with_capacity(payload.len());
                    vec.extend_from_slice(payload);
                    vec
                },}
    }
}
impl <'p> ::std::fmt::Debug for UdpPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "UdpPacket {{ source : {:?}, destination : {:?}, length : {:?}, checksum : {:?},  }}"
               , _self . get_source (  ) , _self . get_destination (  ) ,
               _self . get_length (  ) , _self . get_checksum (  ))
    }
}
impl <'p> ::std::fmt::Debug for MutableUdpPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "MutableUdpPacket {{ source : {:?}, destination : {:?}, length : {:?}, checksum : {:?},  }}"
               , _self . get_source (  ) , _self . get_destination (  ) ,
               _self . get_length (  ) , _self . get_checksum (  ))
    }
}
/// Calculate the checksum for a packet built on IPv4
pub fn ipv4_checksum(packet: &UdpPacket, ipv4_source: Ipv4Addr,
                     ipv4_destination: Ipv4Addr,
                     next_level_protocol: IpNextHeaderProtocol) -> u16be {
    let IpNextHeaderProtocol(next_level_protocol) = next_level_protocol;
    let mut sum = 0u32;
    let octets = ipv4_source.octets();
    sum += (octets[0] as u32) << 8 | (octets[1] as u32);
    sum += (octets[2] as u32) << 8 | (octets[3] as u32);
    let octets = ipv4_destination.octets();
    sum += (octets[0] as u32) << 8 | (octets[1] as u32);
    sum += (octets[2] as u32) << 8 | (octets[3] as u32);
    sum += next_level_protocol as u32;
    sum += (packet.packet()[4] as u32) << 8 | (packet.packet()[5] as u32);
    let mut i = 0;
    let len = packet.get_length() as usize;
    while i < len && i + 1 < packet.packet().len() {
        sum +=
            (packet.packet()[i] as u32) << 8 |
                (packet.packet()[i + 1] as u32);
        i += 2;
    }
    if len & 1 != 0 && len <= packet.packet().len() {
        sum += (packet.packet()[len - 1] as u32) << 8;
    }
    while sum >> 16 != 0 { sum = (sum >> 16) + (sum & 65535); }
    !sum as u16
}
#[test]
fn udp_header_ipv4_test() {
    use pnet::packet::ip::IpNextHeaderProtocols;
    use pnet::packet::ipv4::MutableIpv4Packet;
    let mut packet = [0u8; 20 + 8 + 4];
    let ipv4_source = Ipv4Addr::new(192, 168, 0, 1);
    let ipv4_destination = Ipv4Addr::new(192, 168, 0, 199);
    let next_level_protocol = IpNextHeaderProtocols::Udp;
    {
        let mut ip_header = MutableIpv4Packet::new(&mut packet[..]).unwrap();
        ip_header.set_next_level_protocol(next_level_protocol);
        ip_header.set_source(ipv4_source);
        ip_header.set_destination(ipv4_destination);
    }
    packet[20 + 8] = 't' as u8;
    packet[20 + 8 + 1] = 'e' as u8;
    packet[20 + 8 + 2] = 's' as u8;
    packet[20 + 8 + 3] = 't' as u8;
    {
        let mut udp_header =
            MutableUdpPacket::new(&mut packet[20..]).unwrap();
        udp_header.set_source(12345);
        assert_eq!(udp_header . get_source (  ) , 12345);
        udp_header.set_destination(54321);
        assert_eq!(udp_header . get_destination (  ) , 54321);
        udp_header.set_length(8 + 4);
        assert_eq!(udp_header . get_length (  ) , 8 + 4);
        let checksum =
            ipv4_checksum(&udp_header.to_immutable(), ipv4_source,
                          ipv4_destination, next_level_protocol);
        udp_header.set_checksum(checksum);
        assert_eq!(udp_header . get_checksum (  ) , 0x9178);
    }
    let ref_packet = [48, 57, 212, 49, 0, 12, 145, 120];
    assert_eq!(& ref_packet [ .. ] , & packet [ 20 .. 28 ]);
}
/// Calculate the checksum for a packet built on IPv6
pub fn ipv6_checksum(packet: &UdpPacket, ipv6_source: Ipv6Addr,
                     ipv6_destination: Ipv6Addr,
                     next_header: IpNextHeaderProtocol) -> u16be {
    let IpNextHeaderProtocol(next_header) = next_header;
    let mut sum = 0u32;
    let segments = ipv6_source.segments();
    sum += segments[0] as u32;
    sum += segments[1] as u32;
    sum += segments[2] as u32;
    sum += segments[3] as u32;
    sum += segments[4] as u32;
    sum += segments[5] as u32;
    sum += segments[6] as u32;
    sum += segments[7] as u32;
    let segments = ipv6_destination.segments();
    sum += segments[0] as u32;
    sum += segments[1] as u32;
    sum += segments[2] as u32;
    sum += segments[3] as u32;
    sum += segments[4] as u32;
    sum += segments[5] as u32;
    sum += segments[6] as u32;
    sum += segments[7] as u32;
    sum += next_header as u32;
    sum += packet.get_length() as u32;
    let mut i = 0;
    let len = packet.get_length() as usize;
    while i < len && i + 1 < packet.packet().len() {
        sum +=
            (packet.packet()[i] as u32) << 8 |
                (packet.packet()[i + 1] as u32);
        i += 2;
    }
    if len & 1 != 0 && len <= packet.packet().len() {
        sum += (packet.packet()[len - 1] as u32) << 8;
    }
    while sum >> 16 != 0 { sum = (sum >> 16) + (sum & 65535); }
    !sum as u16
}
#[test]
fn udp_header_ipv6_test() {
    use packet::ip::IpNextHeaderProtocols;
    use packet::ipv6::MutableIpv6Packet;
    let mut packet = [0u8; 40 + 8 + 4];
    let next_header = IpNextHeaderProtocols::Udp;
    let ipv6_source = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
    let ipv6_destination = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
    {
        let mut ip_header = MutableIpv6Packet::new(&mut packet[..]).unwrap();
        ip_header.set_next_header(next_header);
        ip_header.set_source(ipv6_source);
        ip_header.set_destination(ipv6_destination);
    }
    packet[40 + 8] = 't' as u8;
    packet[40 + 8 + 1] = 'e' as u8;
    packet[40 + 8 + 2] = 's' as u8;
    packet[40 + 8 + 3] = 't' as u8;
    {
        let mut udp_header =
            MutableUdpPacket::new(&mut packet[40..]).unwrap();
        udp_header.set_source(12345);
        assert_eq!(udp_header . get_source (  ) , 12345);
        udp_header.set_destination(54321);
        assert_eq!(udp_header . get_destination (  ) , 54321);
        udp_header.set_length(8 + 4);
        assert_eq!(udp_header . get_length (  ) , 8 + 4);
        let checksum =
            ipv6_checksum(&udp_header.to_immutable(), ipv6_source,
                          ipv6_destination, next_header);
        udp_header.set_checksum(checksum);
        assert_eq!(udp_header . get_checksum (  ) , 0x1390);
    }
    let ref_packet = [48, 57, 212, 49, 0, 12, 19, 144];
    assert_eq!(& ref_packet [ .. ] , & packet [ 40 .. 48 ]);
}
