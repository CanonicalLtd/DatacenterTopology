// Copyright (c) 2014, 2015 Robert Clipsham <robert@octarineparrot.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use packet::ip::IpNextHeaderProtocol;

use pnet_macros_support::types::*;

use std::net::Ipv6Addr;

/// Represents an IPv6 Packet
#[allow(unused_attributes)]
#[derive(Debug, Clone)]
pub struct Ipv6 {
    version: u4,
    traffic_class: u8,
    flow_label: u20be,
    payload_length: u16be,
    next_header: IpNextHeaderProtocol,
    hop_limit: u8,
    source: Ipv6Addr,
    destination: Ipv6Addr,
    payload: Vec<u8>,
}









 /* ver/traffic class */
 /* traffic class/flow label */
 /* flow label */
 /* payload length */
 /* next header */
 /* hop limit */
 /* source ip */
/* dest ip */
/// A structure enabling manipulation of on the wire packets
#[derive(PartialEq)]
pub struct Ipv6Packet<'p> {
    packet: &'p [u8],
}
/// A structure enabling manipulation of on the wire packets
#[derive(PartialEq)]
pub struct MutableIpv6Packet<'p> {
    packet: &'p mut [u8],
}
impl <'a> Ipv6Packet<'a> {
    /// Constructs a new Ipv6Packet. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<Ipv6Packet<'p>> {
        if packet.len() >= Ipv6Packet::minimum_packet_size() {
            Some(Ipv6Packet{packet: packet,})
        } else { None }
    }
    /// Maps from a Ipv6Packet to a Ipv6Packet
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> Ipv6Packet<'p> {
        match *self {
            Ipv6Packet { ref packet } => Ipv6Packet{packet: packet,},
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub fn minimum_packet_size() -> usize { 40 }
    /// The size (in bytes) of a Ipv6 instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Ipv6) -> usize { 40 + _packet.payload.len() }
    /// Get the version field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_version(&self) -> u4 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u4) & 240) >> 4
    }
    /// Get the traffic_class field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_traffic_class(&self) -> u8 {
        let _self = self;
        let co = 0;
        let b0 = (((_self.packet[co + 0] as u8) & 15) << 4) as u8;
        let b1 = (((_self.packet[co + 1] as u8) & 240) >> 4) as u8;
        b0 | b1
    }
    /// Get the flow_label field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_flow_label(&self) -> u20be {
        let _self = self;
        let co = 1;
        let b0 = (((_self.packet[co + 0] as u20be) & 15) << 16) as u20be;
        let b1 = ((_self.packet[co + 1] as u20be) << 8) as u20be;
        let b2 = ((_self.packet[co + 2] as u20be)) as u20be;
        b0 | b1 | b2
    }
    /// Get the payload_length field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_payload_length(&self) -> u16be {
        let _self = self;
        let co = 4;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Get the value of the next_header field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_next_header(&self) -> IpNextHeaderProtocol {
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &Ipv6Packet) -> u8 {
            let co = 6;
            (_self.packet[co] as u8)
        }
        IpNextHeaderProtocol::new(get_arg0(&self))
    }
    /// Get the hop_limit field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_hop_limit(&self) -> u8 {
        let _self = self;
        let co = 7;
        (_self.packet[co] as u8)
    }
    /// Get the value of the source field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_source(&self) -> Ipv6Addr {
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &Ipv6Packet) -> u16 {
            let co = 8;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &Ipv6Packet) -> u16 {
            let co = 10;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &Ipv6Packet) -> u16 {
            let co = 12;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &Ipv6Packet) -> u16 {
            let co = 14;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg4(_self: &Ipv6Packet) -> u16 {
            let co = 16;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg5(_self: &Ipv6Packet) -> u16 {
            let co = 18;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg6(_self: &Ipv6Packet) -> u16 {
            let co = 20;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg7(_self: &Ipv6Packet) -> u16 {
            let co = 22;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        Ipv6Addr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                      get_arg3(&self), get_arg4(&self), get_arg5(&self),
                      get_arg6(&self), get_arg7(&self))
    }
    /// Get the value of the destination field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_destination(&self) -> Ipv6Addr {
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &Ipv6Packet) -> u16 {
            let co = 24;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &Ipv6Packet) -> u16 {
            let co = 26;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &Ipv6Packet) -> u16 {
            let co = 28;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &Ipv6Packet) -> u16 {
            let co = 30;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg4(_self: &Ipv6Packet) -> u16 {
            let co = 32;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg5(_self: &Ipv6Packet) -> u16 {
            let co = 34;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg6(_self: &Ipv6Packet) -> u16 {
            let co = 36;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg7(_self: &Ipv6Packet) -> u16 {
            let co = 38;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        Ipv6Addr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                      get_arg3(&self), get_arg4(&self), get_arg5(&self),
                      get_arg6(&self), get_arg7(&self))
    }
}
impl <'a> MutableIpv6Packet<'a> {
    /// Constructs a new MutableIpv6Packet. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8]) -> Option<MutableIpv6Packet<'p>> {
        if packet.len() >= MutableIpv6Packet::minimum_packet_size() {
            Some(MutableIpv6Packet{packet: packet,})
        } else { None }
    }
    /// Maps from a MutableIpv6Packet to a Ipv6Packet
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> Ipv6Packet<'p> {
        match *self {
            MutableIpv6Packet { ref packet } => Ipv6Packet{packet: packet,},
        }
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub fn minimum_packet_size() -> usize { 40 }
    /// The size (in bytes) of a Ipv6 instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Ipv6) -> usize { 40 + _packet.payload.len() }
    /// Populates a Ipv6Packet using a Ipv6 structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &Ipv6) {
        let _self = self;
        _self.set_version(packet.version);
        _self.set_traffic_class(packet.traffic_class);
        _self.set_flow_label(packet.flow_label);
        _self.set_payload_length(packet.payload_length);
        _self.set_next_header(packet.next_header);
        _self.set_hop_limit(packet.hop_limit);
        _self.set_source(packet.source);
        _self.set_destination(packet.destination);
        _self.set_payload(&packet.payload);
    }
    /// Get the version field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_version(&self) -> u4 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u4) & 240) >> 4
    }
    /// Get the traffic_class field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_traffic_class(&self) -> u8 {
        let _self = self;
        let co = 0;
        let b0 = (((_self.packet[co + 0] as u8) & 15) << 4) as u8;
        let b1 = (((_self.packet[co + 1] as u8) & 240) >> 4) as u8;
        b0 | b1
    }
    /// Get the flow_label field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_flow_label(&self) -> u20be {
        let _self = self;
        let co = 1;
        let b0 = (((_self.packet[co + 0] as u20be) & 15) << 16) as u20be;
        let b1 = ((_self.packet[co + 1] as u20be) << 8) as u20be;
        let b2 = ((_self.packet[co + 2] as u20be)) as u20be;
        b0 | b1 | b2
    }
    /// Get the payload_length field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_payload_length(&self) -> u16be {
        let _self = self;
        let co = 4;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Get the value of the next_header field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_next_header(&self) -> IpNextHeaderProtocol {
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableIpv6Packet) -> u8 {
            let co = 6;
            (_self.packet[co] as u8)
        }
        IpNextHeaderProtocol::new(get_arg0(&self))
    }
    /// Get the hop_limit field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_hop_limit(&self) -> u8 {
        let _self = self;
        let co = 7;
        (_self.packet[co] as u8)
    }
    /// Get the value of the source field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_source(&self) -> Ipv6Addr {
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableIpv6Packet) -> u16 {
            let co = 8;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &MutableIpv6Packet) -> u16 {
            let co = 10;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &MutableIpv6Packet) -> u16 {
            let co = 12;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &MutableIpv6Packet) -> u16 {
            let co = 14;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg4(_self: &MutableIpv6Packet) -> u16 {
            let co = 16;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg5(_self: &MutableIpv6Packet) -> u16 {
            let co = 18;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg6(_self: &MutableIpv6Packet) -> u16 {
            let co = 20;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg7(_self: &MutableIpv6Packet) -> u16 {
            let co = 22;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        Ipv6Addr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                      get_arg3(&self), get_arg4(&self), get_arg5(&self),
                      get_arg6(&self), get_arg7(&self))
    }
    /// Get the value of the destination field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_destination(&self) -> Ipv6Addr {
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableIpv6Packet) -> u16 {
            let co = 24;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &MutableIpv6Packet) -> u16 {
            let co = 26;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &MutableIpv6Packet) -> u16 {
            let co = 28;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &MutableIpv6Packet) -> u16 {
            let co = 30;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg4(_self: &MutableIpv6Packet) -> u16 {
            let co = 32;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg5(_self: &MutableIpv6Packet) -> u16 {
            let co = 34;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg6(_self: &MutableIpv6Packet) -> u16 {
            let co = 36;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg7(_self: &MutableIpv6Packet) -> u16 {
            let co = 38;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        Ipv6Addr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                      get_arg3(&self), get_arg4(&self), get_arg5(&self),
                      get_arg6(&self), get_arg7(&self))
    }
    /// Set the version field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_version(&mut self, val: u4) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] =
            ((_self.packet[co + 0] & 15) | (((val & 15) << 4) as u8)) as u8;
    }
    /// Set the traffic_class field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_traffic_class(&mut self, val: u8) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] =
            ((_self.packet[co + 0] & 240) | (((val & 240) >> 4) as u8)) as u8;
        _self.packet[co + 1] =
            ((_self.packet[co + 1] & 15) | (((val & 15) << 4) as u8)) as u8;
    }
    /// Set the flow_label field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_flow_label(&mut self, val: u20be) {
        let _self = self;
        let co = 1;
        _self.packet[co + 0] =
            ((_self.packet[co + 0] & 240) | (((val & 983040) >> 16) as u8)) as
                u8;
        _self.packet[co + 1] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 2] = (val) as u8;
    }
    /// Set the payload_length field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_payload_length(&mut self, val: u16be) {
        let _self = self;
        let co = 4;
        _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 1] = (val) as u8;
    }
    /// Set the value of the next_header field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_next_header(&mut self, val: IpNextHeaderProtocol) {
        use pnet::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableIpv6Packet, val: u8) {
            let co = 6;
            _self.packet[co + 0] = (val) as u8;
        }
        let vals = val.to_primitive_values();
        set_arg0(_self, vals.0);
    }
    /// Set the hop_limit field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_hop_limit(&mut self, val: u8) {
        let _self = self;
        let co = 7;
        _self.packet[co + 0] = (val) as u8;
    }
    /// Set the value of the source field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_source(&mut self, val: Ipv6Addr) {
        use pnet::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableIpv6Packet, val: u16) {
            let co = 8;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg1(_self: &mut MutableIpv6Packet, val: u16) {
            let co = 10;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg2(_self: &mut MutableIpv6Packet, val: u16) {
            let co = 12;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg3(_self: &mut MutableIpv6Packet, val: u16) {
            let co = 14;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg4(_self: &mut MutableIpv6Packet, val: u16) {
            let co = 16;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg5(_self: &mut MutableIpv6Packet, val: u16) {
            let co = 18;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg6(_self: &mut MutableIpv6Packet, val: u16) {
            let co = 20;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg7(_self: &mut MutableIpv6Packet, val: u16) {
            let co = 22;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        let vals = val.to_primitive_values();
        set_arg0(_self, vals.0);
        set_arg1(_self, vals.1);
        set_arg2(_self, vals.2);
        set_arg3(_self, vals.3);
        set_arg4(_self, vals.4);
        set_arg5(_self, vals.5);
        set_arg6(_self, vals.6);
        set_arg7(_self, vals.7);
    }
    /// Set the value of the destination field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_destination(&mut self, val: Ipv6Addr) {
        use pnet::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableIpv6Packet, val: u16) {
            let co = 24;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg1(_self: &mut MutableIpv6Packet, val: u16) {
            let co = 26;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg2(_self: &mut MutableIpv6Packet, val: u16) {
            let co = 28;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg3(_self: &mut MutableIpv6Packet, val: u16) {
            let co = 30;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg4(_self: &mut MutableIpv6Packet, val: u16) {
            let co = 32;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg5(_self: &mut MutableIpv6Packet, val: u16) {
            let co = 34;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg6(_self: &mut MutableIpv6Packet, val: u16) {
            let co = 36;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg7(_self: &mut MutableIpv6Packet, val: u16) {
            let co = 38;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        let vals = val.to_primitive_values();
        set_arg0(_self, vals.0);
        set_arg1(_self, vals.1);
        set_arg2(_self, vals.2);
        set_arg3(_self, vals.3);
        set_arg4(_self, vals.4);
        set_arg5(_self, vals.5);
        set_arg6(_self, vals.6);
        set_arg7(_self, vals.7);
    }
    /// Set the value of the payload field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_payload(&mut self, vals: &[u8]) {
        use std::ptr::copy_nonoverlapping;
        let mut _self = self;
        let current_offset = 40;
        unsafe {
            copy_nonoverlapping(vals[..].as_ptr(),
                                _self.packet[current_offset..].as_mut_ptr(),
                                vals.len())
        }
    }
}
impl <'a> ::pnet::packet::PacketSize for Ipv6Packet<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize { let _self = self; 40 }
}
impl <'a> ::pnet::packet::PacketSize for MutableIpv6Packet<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize { let _self = self; 40 }
}
impl <'a> ::pnet::packet::MutablePacket for MutableIpv6Packet<'a> {
    #[inline]
    fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] { &mut self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
        let _self = self;
        let start = 40;
        &mut _self.packet[start..]
    }
}
impl <'a> ::pnet::packet::Packet for MutableIpv6Packet<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 40;
        &_self.packet[start..]
    }
}
impl <'a> ::pnet::packet::Packet for Ipv6Packet<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 40;
        &_self.packet[start..]
    }
}
/// Used to iterate over a slice of `Ipv6Packet`s
pub struct Ipv6Iterable<'a> {
    buf: &'a [u8],
}
impl <'a> Iterator for Ipv6Iterable<'a> {
    type
    Item
    =
    Ipv6Packet<'a>;
    fn next(&mut self) -> Option<Ipv6Packet<'a>> {
        use pnet::packet::PacketSize;
        if self.buf.len() > 0 {
            let ret = Ipv6Packet::new(self.buf).unwrap();
            self.buf = &self.buf[ret.packet_size()..];
            return Some(ret);
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
}
impl <'p> ::pnet::packet::FromPacket for Ipv6Packet<'p> {
    type
    T
    =
    Ipv6;
    #[inline]
    fn from_packet(&self) -> Ipv6 {
        use pnet::packet::Packet;
        let _self = self;
        Ipv6{version: _self.get_version(),
             traffic_class: _self.get_traffic_class(),
             flow_label: _self.get_flow_label(),
             payload_length: _self.get_payload_length(),
             next_header: _self.get_next_header(),
             hop_limit: _self.get_hop_limit(),
             source: _self.get_source(),
             destination: _self.get_destination(),
             payload:
                 {
                     let payload = self.payload();
                     let mut vec = Vec::with_capacity(payload.len());
                     vec.extend_from_slice(payload);
                     vec
                 },}
    }
}
impl <'p> ::pnet::packet::FromPacket for MutableIpv6Packet<'p> {
    type
    T
    =
    Ipv6;
    #[inline]
    fn from_packet(&self) -> Ipv6 {
        use pnet::packet::Packet;
        let _self = self;
        Ipv6{version: _self.get_version(),
             traffic_class: _self.get_traffic_class(),
             flow_label: _self.get_flow_label(),
             payload_length: _self.get_payload_length(),
             next_header: _self.get_next_header(),
             hop_limit: _self.get_hop_limit(),
             source: _self.get_source(),
             destination: _self.get_destination(),
             payload:
                 {
                     let payload = self.payload();
                     let mut vec = Vec::with_capacity(payload.len());
                     vec.extend_from_slice(payload);
                     vec
                 },}
    }
}
impl <'p> ::std::fmt::Debug for Ipv6Packet<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "Ipv6Packet {{ version : {:?}, traffic_class : {:?}, flow_label : {:?}, payload_length : {:?}, next_header : {:?}, hop_limit : {:?}, source : {:?}, destination : {:?},  }}"
               , _self . get_version (  ) , _self . get_traffic_class (  ) ,
               _self . get_flow_label (  ) , _self . get_payload_length (  ) ,
               _self . get_next_header (  ) , _self . get_hop_limit (  ) ,
               _self . get_source (  ) , _self . get_destination (  ))
    }
}
impl <'p> ::std::fmt::Debug for MutableIpv6Packet<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "MutableIpv6Packet {{ version : {:?}, traffic_class : {:?}, flow_label : {:?}, payload_length : {:?}, next_header : {:?}, hop_limit : {:?}, source : {:?}, destination : {:?},  }}"
               , _self . get_version (  ) , _self . get_traffic_class (  ) ,
               _self . get_flow_label (  ) , _self . get_payload_length (  ) ,
               _self . get_next_header (  ) , _self . get_hop_limit (  ) ,
               _self . get_source (  ) , _self . get_destination (  ))
    }
}
#[test]
fn ipv6_header_test() {
    use packet::ip::IpNextHeaderProtocols;
    let mut packet = [0u8; 40];
    {
        let mut ip_header = MutableIpv6Packet::new(&mut packet[..]).unwrap();
        ip_header.set_version(6);
        assert_eq!(ip_header . get_version (  ) , 6);
        ip_header.set_traffic_class(17);
        assert_eq!(ip_header . get_traffic_class (  ) , 17);
        ip_header.set_flow_label(65793);
        assert_eq!(ip_header . get_flow_label (  ) , 0x10101);
        ip_header.set_payload_length(257);
        assert_eq!(ip_header . get_payload_length (  ) , 0x0101);
        ip_header.set_next_header(IpNextHeaderProtocols::Udp);
        assert_eq!(ip_header . get_next_header (  ) , IpNextHeaderProtocols::
                   Udp);
        ip_header.set_hop_limit(1);
        assert_eq!(ip_header . get_hop_limit (  ) , 1);
        let source =
            Ipv6Addr::new(272, 4097, 272, 4097, 272, 4097, 272, 4097);
        ip_header.set_source(source);
        assert_eq!(ip_header . get_source (  ) , source);
        let dest = Ipv6Addr::new(272, 4097, 272, 4097, 272, 4097, 272, 4097);
        ip_header.set_destination(dest);
        assert_eq!(ip_header . get_destination (  ) , dest);
    }
    let ref_packet =
        [97, 17, 1, 1, 1, 1, 17, 1, 1, 16, 16, 1, 1, 16, 16, 1, 1, 16, 16, 1,
         1, 16, 16, 1, 1, 16, 16, 1, 1, 16, 16, 1, 1, 16, 16, 1, 1, 16, 16,
         1];
    assert_eq!(& ref_packet [ .. ] , & packet [ .. ]);
}
