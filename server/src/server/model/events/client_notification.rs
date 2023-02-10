#[derive(Debug)]
pub enum Notification {
    Char(CharNotification),
    Area(AreaNotification),
}

#[derive(Debug)]
pub struct CharNotification {
    char_id: u32,
    packet: Vec<u8>,
}

impl CharNotification {
    pub fn new(char_id: u32, packet: Vec<u8>) -> Self {
        Self {
            char_id,
            packet
        }
    }
    pub fn char_id(&self) -> u32 {
        self.char_id
    }

    pub fn serialized_packet(&self) -> &Vec<u8> {
        &self.packet
    }
}

#[derive(Debug)]
pub struct AreaNotification {
    pub(crate) map_name: String,
    pub(crate) map_instance_id: u8,
    pub(crate) range_type: AreaNotificationRangeType,
    pub(crate) packet: Vec<u8>,
}
#[derive(PartialEq, Debug)]
pub enum AreaNotificationRangeType {
    #[allow(dead_code)]
    Map, // Notify all players of the map,
    Fov {x: u16, y: u16, exclude_id: Option<u32>} // Notify all players in the FoV of the point.
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