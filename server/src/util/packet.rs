use std::fmt::{Display, Formatter};
use std::net::{SocketAddr};
use std::panic;
use packets::packets::Packet;
use packets::packets_parser::parse;
use crate::server::service::global_config_service::GlobalConfigService;

pub fn chain_packets(packets: Vec<&dyn Packet>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    for packet in packets {
        res = res.iter().cloned().chain(packet.raw().iter().cloned()).collect();
    }
    res
}

#[allow(dead_code)]
pub fn chain_packets_raws(packets_raw: Vec<&Vec<u8>>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    for raw in packets_raw {
        res = res.iter().cloned().chain(raw.iter().cloned()).collect();
    }
    res
}

pub fn chain_packets_raws_by_value(packets_raw: Vec<Vec<u8>>) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    for raw in packets_raw {
        res = res.iter().cloned().chain(raw).collect();
    }
    res
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PacketDirection {
    Forward,
    Backward,
}

impl Display for PacketDirection {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

pub fn debug_packets_from_vec(outgoing: &SocketAddr, direction: PacketDirection, packetver: u32, bytes: &Vec<u8>, name: &Option<String>) {
    let mut buffer = [0_u8; 2048];
    if bytes.len() < 2 {
        return;
    }
    for (i, byte) in bytes.iter().enumerate() {
        if i >= buffer.len() {
            break;
        }
        buffer[i] = *byte;
    }
    debug_packets(outgoing, direction, packetver, &buffer, bytes.len(), name);
}
pub fn debug_packets(outgoing: &SocketAddr, direction: PacketDirection, packetver: u32, buffer: &[u8; 2048], bytes_read: usize, name: &Option<String>) {
    let packet = parse(&buffer[..bytes_read], packetver);
    if packet.raw().len() < bytes_read {
        let mut offset = 0;
        while offset < bytes_read {
            let result = panic::catch_unwind(|| {
                parse(&buffer[offset..bytes_read], packetver)
            });
            if let Ok(packet) = result {
                offset += packet.raw().len();
                print_packet(name, outgoing, direction, packet);
            } else {
                break;
            }
        }
    } else {
        print_packet(name, outgoing, direction, packet);
    }
}

fn print_packet(name: &Option<String>, outgoing: &SocketAddr, direction: PacketDirection, packet: Box<dyn Packet>) {
    if packet.id(GlobalConfigService::instance().packetver()) != "0x6003"
        && packet.id(GlobalConfigService::instance().packetver()) != "0x7f00"
        && packet.id(GlobalConfigService::instance().packetver()) != "0x0887"
        && packet.id(GlobalConfigService::instance().packetver()) != "0x7e00" { // PACKET_CZ_REQUEST_TIME2
        println!("\n----------------------------Start Packet----------------------------");
        info!("{} {} {}", name.as_ref().cloned().unwrap_or("server".to_string()), if direction == PacketDirection::Backward { "<" } else { ">" }, outgoing);
        packet.display();
        packet.pretty_debug();
        info!("{:02X?}", packet.raw());
        println!("----------------------------End Packet----------------------------\n");
    }
}