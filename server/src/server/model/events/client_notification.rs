use std::fmt::{Debug, Formatter};
use packets::packets::Packet;
use packets::packets_parser;

#[derive(Clone)]
pub enum Notification {
    Char(CharNotification),
    Area(AreaNotification),
}

impl Debug for Notification {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Notification::Char(n) => {
                let packet = if n.packet.len() > 0 { format!("0x{:02X?}{:02X?}", n.packet[0], n.packet[1]) } else { "empty".to_string() };
                f.debug_struct("CharNotification")
                    .field("char_id", &n.char_id)
                    .field("packet", &packet)
                    .finish()
            }
            Notification::Area(n) => {
                f.debug_struct("AreaNotification")
                    .field("map_name", &n.map_name)
                    .field("map_instance_id", &n.map_instance_id)
                    .field("range_type", &n.range_type)
                    .field("packet", &format!("0x{:02X?}{:02X?}", n.packet[0], n.packet[1]))
                    .finish()
            }
        }
    }
}

impl Notification {
    pub fn packet(&self, packetver: u32) -> Box<dyn Packet> {
        match self {
            Notification::Char(notification) => {
                packets_parser::parse(&notification.packet[..], packetver)
            }
            Notification::Area(notification) => {
                packets_parser::parse(&notification.packet[..], packetver)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CharNotification {
    char_id: u32,
    packet: Vec<u8>,
}

impl CharNotification {
    pub fn new(char_id: u32, packet: Vec<u8>) -> Self {
        Self {
            char_id,
            packet,
        }
    }
    pub fn char_id(&self) -> u32 {
        self.char_id
    }

    pub fn serialized_packet(&self) -> &Vec<u8> {
        &self.packet
    }
}

#[derive(Debug, Clone)]
pub struct AreaNotification {
    pub(crate) map_name: String,
    pub(crate) map_instance_id: u8,
    pub(crate) range_type: AreaNotificationRangeType,
    pub(crate) packet: Vec<u8>,
}
#[derive(PartialEq, Debug, Clone)]
pub enum AreaNotificationRangeType {
    #[allow(dead_code)]
    Map, // Notify all players of the map,
    Fov { x: u16, y: u16, exclude_id: Option<u32> }, // Notify all players in the FoV of the point.
}

impl AreaNotification {
    pub fn new(map_name: String, map_instance_id: u8, range_type: AreaNotificationRangeType, packet: Vec<u8>) -> Self {
        Self {
            map_name,
            map_instance_id,
            packet,
            range_type,
        }
    }
    pub fn serialized_packet(&self) -> &Vec<u8> {
        &self.packet
    }
}