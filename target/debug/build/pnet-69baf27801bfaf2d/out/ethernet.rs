// Copyright (c) 2014, 2015 Robert Clipsham <robert@octarineparrot.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use packet::{Packet, PrimitiveValues};

use util::MacAddr;

/// Represents an Ethernet packet
#[allow(unused_attributes)]
#[derive(Debug, Clone)]
pub struct Ethernet {
    destination: MacAddr,
    source: MacAddr,
    ethertype: EtherType,
    payload: Vec<u8>,
}





 /* destination */
 /* source */
/* ethertype */




/// A structure enabling manipulation of on the wire packets
#[derive(PartialEq)]
pub struct EthernetPacket<'p> {
    packet: &'p [u8],
}
/// A structure enabling manipulation of on the wire packets
#[derive(PartialEq)]
pub struct MutableEthernetPacket<'p> {
    packet: &'p mut [u8],
}
impl <'a> EthernetPacket<'a> {
    /// Constructs a new EthernetPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<EthernetPacket<'p>> {
        if packet.len() >= EthernetPacket::minimum_packet_size() {
            Some(EthernetPacket{packet: packet,})
        } else { None }
    }
    /// Maps from a EthernetPacket to a EthernetPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> EthernetPacket<'p> {
        match *self {
            EthernetPacket { ref packet } => EthernetPacket{packet: packet,},
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub fn minimum_packet_size() -> usize { 14 }
    /// The size (in bytes) of a Ethernet instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Ethernet) -> usize {
        14 + _packet.payload.len()
    }
    /// Get the value of the destination field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_destination(&self) -> MacAddr {
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &EthernetPacket) -> u8 {
            let co = 0;
            (_self.packet[co] as u8)
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &EthernetPacket) -> u8 {
            let co = 1;
            (_self.packet[co] as u8)
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &EthernetPacket) -> u8 {
            let co = 2;
            (_self.packet[co] as u8)
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &EthernetPacket) -> u8 {
            let co = 3;
            (_self.packet[co] as u8)
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg4(_self: &EthernetPacket) -> u8 {
            let co = 4;
            (_self.packet[co] as u8)
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg5(_self: &EthernetPacket) -> u8 {
            let co = 5;
            (_self.packet[co] as u8)
        }
        MacAddr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                     get_arg3(&self), get_arg4(&self), get_arg5(&self))
    }
    /// Get the value of the source field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_source(&self) -> MacAddr {
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &EthernetPacket) -> u8 {
            let co = 6;
            (_self.packet[co] as u8)
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &EthernetPacket) -> u8 {
            let co = 7;
            (_self.packet[co] as u8)
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &EthernetPacket) -> u8 {
            let co = 8;
            (_self.packet[co] as u8)
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &EthernetPacket) -> u8 {
            let co = 9;
            (_self.packet[co] as u8)
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg4(_self: &EthernetPacket) -> u8 {
            let co = 10;
            (_self.packet[co] as u8)
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg5(_self: &EthernetPacket) -> u8 {
            let co = 11;
            (_self.packet[co] as u8)
        }
        MacAddr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                     get_arg3(&self), get_arg4(&self), get_arg5(&self))
    }
    /// Get the value of the ethertype field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_ethertype(&self) -> EtherType {
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &EthernetPacket) -> u16 {
            let co = 12;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        EtherType::new(get_arg0(&self))
    }
}
impl <'a> MutableEthernetPacket<'a> {
    /// Constructs a new MutableEthernetPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8])
     -> Option<MutableEthernetPacket<'p>> {
        if packet.len() >= MutableEthernetPacket::minimum_packet_size() {
            Some(MutableEthernetPacket{packet: packet,})
        } else { None }
    }
    /// Maps from a MutableEthernetPacket to a EthernetPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> EthernetPacket<'p> {
        match *self {
            MutableEthernetPacket { ref packet } =>
            EthernetPacket{packet: packet,},
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub fn minimum_packet_size() -> usize { 14 }
    /// The size (in bytes) of a Ethernet instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Ethernet) -> usize {
        14 + _packet.payload.len()
    }
    /// Populates a EthernetPacket using a Ethernet structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &Ethernet) {
        let _self = self;
        _self.set_destination(packet.destination);
        _self.set_source(packet.source);
        _self.set_ethertype(packet.ethertype);
        _self.set_payload(&packet.payload);
    }
    /// Get the value of the destination field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_destination(&self) -> MacAddr {
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableEthernetPacket) -> u8 {
            let co = 0;
            (_self.packet[co] as u8)
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &MutableEthernetPacket) -> u8 {
            let co = 1;
            (_self.packet[co] as u8)
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &MutableEthernetPacket) -> u8 {
            let co = 2;
            (_self.packet[co] as u8)
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &MutableEthernetPacket) -> u8 {
            let co = 3;
            (_self.packet[co] as u8)
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg4(_self: &MutableEthernetPacket) -> u8 {
            let co = 4;
            (_self.packet[co] as u8)
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg5(_self: &MutableEthernetPacket) -> u8 {
            let co = 5;
            (_self.packet[co] as u8)
        }
        MacAddr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                     get_arg3(&self), get_arg4(&self), get_arg5(&self))
    }
    /// Get the value of the source field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_source(&self) -> MacAddr {
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableEthernetPacket) -> u8 {
            let co = 6;
            (_self.packet[co] as u8)
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &MutableEthernetPacket) -> u8 {
            let co = 7;
            (_self.packet[co] as u8)
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &MutableEthernetPacket) -> u8 {
            let co = 8;
            (_self.packet[co] as u8)
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &MutableEthernetPacket) -> u8 {
            let co = 9;
            (_self.packet[co] as u8)
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg4(_self: &MutableEthernetPacket) -> u8 {
            let co = 10;
            (_self.packet[co] as u8)
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg5(_self: &MutableEthernetPacket) -> u8 {
            let co = 11;
            (_self.packet[co] as u8)
        }
        MacAddr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                     get_arg3(&self), get_arg4(&self), get_arg5(&self))
    }
    /// Get the value of the ethertype field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_ethertype(&self) -> EtherType {
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableEthernetPacket) -> u16 {
            let co = 12;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        EtherType::new(get_arg0(&self))
    }
    /// Set the value of the destination field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_destination(&mut self, val: MacAddr) {
        use pnet::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 0;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg1(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 1;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg2(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 2;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg3(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 3;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg4(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 4;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg5(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 5;
            _self.packet[co + 0] = (val) as u8;
        }
        let vals = val.to_primitive_values();
        set_arg0(_self, vals.0);
        set_arg1(_self, vals.1);
        set_arg2(_self, vals.2);
        set_arg3(_self, vals.3);
        set_arg4(_self, vals.4);
        set_arg5(_self, vals.5);
    }
    /// Set the value of the source field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_source(&mut self, val: MacAddr) {
        use pnet::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 6;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg1(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 7;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg2(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 8;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg3(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 9;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg4(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 10;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg5(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 11;
            _self.packet[co + 0] = (val) as u8;
        }
        let vals = val.to_primitive_values();
        set_arg0(_self, vals.0);
        set_arg1(_self, vals.1);
        set_arg2(_self, vals.2);
        set_arg3(_self, vals.3);
        set_arg4(_self, vals.4);
        set_arg5(_self, vals.5);
    }
    /// Set the value of the ethertype field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_ethertype(&mut self, val: EtherType) {
        use pnet::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableEthernetPacket, val: u16) {
            let co = 12;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        let vals = val.to_primitive_values();
        set_arg0(_self, vals.0);
    }
    /// Set the value of the payload field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_payload(&mut self, vals: &[u8]) {
        use std::ptr::copy_nonoverlapping;
        let mut _self = self;
        let current_offset = 14;
        unsafe {
            copy_nonoverlapping(vals[..].as_ptr(),
                                _self.packet[current_offset..].as_mut_ptr(),
                                vals.len())
        }
    }
}
impl <'a> ::pnet::packet::PacketSize for EthernetPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize { let _self = self; 14 }
}
impl <'a> ::pnet::packet::PacketSize for MutableEthernetPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize { let _self = self; 14 }
}
impl <'a> ::pnet::packet::MutablePacket for MutableEthernetPacket<'a> {
    #[inline]
    fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] { &mut self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
        let _self = self;
        let start = 14;
        &mut _self.packet[start..]
    }
}
impl <'a> ::pnet::packet::Packet for MutableEthernetPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 14;
        &_self.packet[start..]
    }
}
impl <'a> ::pnet::packet::Packet for EthernetPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 14;
        &_self.packet[start..]
    }
}
/// Used to iterate over a slice of `EthernetPacket`s
pub struct EthernetIterable<'a> {
    buf: &'a [u8],
}
impl <'a> Iterator for EthernetIterable<'a> {
    type
    Item
    =
    EthernetPacket<'a>;
    fn next(&mut self) -> Option<EthernetPacket<'a>> {
        use pnet::packet::PacketSize;
        if self.buf.len() > 0 {
            let ret = EthernetPacket::new(self.buf).unwrap();
            self.buf = &self.buf[ret.packet_size()..];
            return Some(ret);
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
}
impl <'p> ::pnet::packet::FromPacket for EthernetPacket<'p> {
    type
    T
    =
    Ethernet;
    #[inline]
    fn from_packet(&self) -> Ethernet {
        use pnet::packet::Packet;
        let _self = self;
        Ethernet{destination: _self.get_destination(),
                 source: _self.get_source(),
                 ethertype: _self.get_ethertype(),
                 payload:
                     {
                         let payload = self.payload();
                         let mut vec = Vec::with_capacity(payload.len());
                         vec.extend_from_slice(payload);
                         vec
                     },}
    }
}
impl <'p> ::pnet::packet::FromPacket for MutableEthernetPacket<'p> {
    type
    T
    =
    Ethernet;
    #[inline]
    fn from_packet(&self) -> Ethernet {
        use pnet::packet::Packet;
        let _self = self;
        Ethernet{destination: _self.get_destination(),
                 source: _self.get_source(),
                 ethertype: _self.get_ethertype(),
                 payload:
                     {
                         let payload = self.payload();
                         let mut vec = Vec::with_capacity(payload.len());
                         vec.extend_from_slice(payload);
                         vec
                     },}
    }
}
impl <'p> ::std::fmt::Debug for EthernetPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "EthernetPacket {{ destination : {:?}, source : {:?}, ethertype : {:?},  }}"
               , _self . get_destination (  ) , _self . get_source (  ) ,
               _self . get_ethertype (  ))
    }
}
impl <'p> ::std::fmt::Debug for MutableEthernetPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "MutableEthernetPacket {{ destination : {:?}, source : {:?}, ethertype : {:?},  }}"
               , _self . get_destination (  ) , _self . get_source (  ) ,
               _self . get_ethertype (  ))
    }
}
#[test]
fn ethernet_header_test() {
    let mut packet = [0u8; 14];
    {
        let mut ethernet_header =
            MutableEthernetPacket::new(&mut packet[..]).unwrap();
        let source = MacAddr(18, 52, 86, 120, 154, 188);
        ethernet_header.set_source(source);
        assert_eq!(ethernet_header . get_source (  ) , source);
        let dest = MacAddr(222, 240, 18, 52, 69, 103);
        ethernet_header.set_destination(dest);
        assert_eq!(ethernet_header . get_destination (  ) , dest);
        ethernet_header.set_ethertype(EtherTypes::Ipv6);
        assert_eq!(ethernet_header . get_ethertype (  ) , EtherTypes:: Ipv6);
    }
    let ref_packet =
        [222, 240, 18, 52, 69, 103, 18, 52, 86, 120, 154, 188, 134, 221];
    assert_eq!(& ref_packet [ .. ] , & packet [ .. ]);
}
/// EtherTypes defined at:
/// http://www.iana.org/assignments/ieee-802-numbers/ieee-802-numbers.xhtml
/// These values should be used in the Ethernet EtherType field
///
/// FIXME Should include all
/// A handful of these have been selected since most are archaic and unused.
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod EtherTypes {
    use packet::ethernet::EtherType;
    /// Internet Protocol version 4 (IPv4) [RFC7042]
    pub const Ipv4: EtherType = EtherType(2048);
    /// Address Resolution Protocol (ARP) [RFC7042]
    pub const Arp: EtherType = EtherType(2054);
    /// Wake on Lan
    pub const WakeOnLan: EtherType = EtherType(2114);
    /// Reverse Address Resolution Protocol (RARP) [RFC903]
    pub const Rarp: EtherType = EtherType(32821);
    /// Internet Protocol version 6 (IPv6) [RFC7042]
    pub const Ipv6: EtherType = EtherType(34525);
}
/// Represents the Ethernet ethertype field.
#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
pub struct EtherType(pub u16);
impl EtherType {
    /// Construct a new EtherType
    pub fn new(val: u16) -> EtherType { EtherType(val) }
}
impl PrimitiveValues for EtherType {
    type
    T
    =
    (u16,);
    fn to_primitive_values(&self) -> (u16,) { (self.0,) }
}
